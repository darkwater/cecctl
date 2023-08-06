use std::{collections::HashMap, convert::TryFrom, process::Command, thread};

use anyhow::{Context, Result};
use cec_rs::{
    CecCommand, CecConnectionCfgBuilder, CecDeviceType, CecDeviceTypeVec, CecOpcode,
    CecUserControlCode,
};
use config::File;
use heck::ToSnakeCase;
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Config {
    display_name: String,
    port: String,
    binds: HashMap<String, String>,
}

use clap::Parser;

/// Run commands with a TV remote
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether to tell the TV to change input to this device
    #[arg(short, long)]
    take_focus: bool,

    /// Level to show logs at. One of "error", "warn", "info", "debug", "trace", or "off"
    #[arg(short, long, default_value = "info")]
    log_level: LevelFilter,
}

fn main() -> Result<()> {
    let args = Args::parse();

    pretty_env_logger::formatted_builder()
        .filter_level(args.log_level)
        .init();

    let config_path = dirs::config_dir()
        .context("unknown config dir for this platform")?
        .join("cecctl.toml");

    let config = match config::Config::builder()
        .add_source(File::with_name(
            config_path.to_str().context("config path invalid utf-8")?,
        ))
        .build()
    {
        Ok(config) => config,
        Err(err) => {
            println!("No valid config file found.");
            println!("Config path: {}", config_path.display());
            println!("Example config:");
            println!("----------");
            println!(r#"display_name = "HTPC""#);
            println!(r#"port = "/dev/ttyACM0""#);
            println!();
            println!(r#"[binds]"#);
            println!(r#"up = "wtype -k up""#);
            println!(r#"down = "wtype -k down""#);
            println!(r#"left = "wtype -k left""#);
            println!(r#"right = "wtype -k right""#);
            println!(r#"select = "wtype -k return""#);
            println!(r#"quit = "wtype -k escape""#);
            println!("----------");
            println!();

            return Err(err).context("failed to load config");
        }
    };

    let config = config
        .try_deserialize::<Config>()
        .context("failed to parse config")?;

    let mut children = vec![];

    let mut last_pressed = None;
    let mut received_something = false;
    let cmd_handler = {
        let config = config.clone();
        move |e: CecCommand| {
            log::debug!("{:?} -> {:?}: {:?}", e.initiator, e.destination, e.opcode,);

            log::trace!("{:?}", e.parameters);

            if !received_something {
                received_something = true;
                log::info!("Connected to TV");
            }

            match (e.opcode, e.parameters.0.as_slice()) {
                (CecOpcode::UserControlPressed, [code]) => {
                    let Ok(key) = CecUserControlCode::try_from(*code as u32) else { return };
                    let Some(key_name) = format!("{:?}", key).split("::").last().map(|s| s.to_snake_case()) else { return };

                    let key_name = key_name.to_snake_case();

                    log::debug!("pressed {:?}", key_name);

                    last_pressed = Some(key);

                    let Some(cmd) = config.binds.get(&key_name) else {
                        log::info!("unbound key pressed: {:?}", key_name);
                        return;
                    };

                    log::debug!("running {:?}", cmd);

                    let child = Command::new("/bin/sh").arg("-c").arg(cmd).spawn().unwrap();
                    children.push(child);
                }

                (CecOpcode::UserControlRelease, _) => {
                    if let Some(key) = last_pressed.take() {
                        log::debug!("released {:?}", key);
                    } else {
                        log::warn!("released unpressed key");
                    }
                }

                (CecOpcode::None, _) => {
                    log::info!("received empty packet from {:?}", e.initiator);
                }

                _ => {}
            }

            children.retain_mut(|child| child.try_wait().unwrap().is_none());
        }
    };

    log::info!("Connecting to TV...");

    let cec = CecConnectionCfgBuilder::default()
        .port(config.port)
        .device_name(config.display_name.clone())
        .device_types(CecDeviceTypeVec::new(CecDeviceType::PlaybackDevice))
        .command_received_callback(Box::new(cmd_handler))
        .build()
        .context("failed to build cec config")?
        .open()
        .map_err(|e| anyhow::anyhow!("failed to open cec connection: {:?}", e))?;

    if args.take_focus {
        cec.set_active_source(CecDeviceType::PlaybackDevice)
            .map_err(|e| anyhow::anyhow!("failed to open cec connection: {:?}", e))?;
    }

    // let res = cec.transmit(CecCommand {
    //     initiator: CecLogicalAddress::Playbackdevice1,
    //     destination: CecLogicalAddress::Playbackdevice2,
    //     ack: Default::default(),
    //     eom: Default::default(),
    //     opcode: CecOpcode::None,
    //     parameters: CecDatapacket(Default::default()),
    //     opcode_set: false,
    //     transmit_timeout: Duration::from_secs(2),
    // });

    // println!("set power: {:?}", res);

    // let res = cec.transmit(CecCommand {
    //     initiator: CecLogicalAddress::Playbackdevice1,
    //     destination: CecLogicalAddress::Unregistered,
    //     ack: Default::default(),
    //     eom: Default::default(),
    //     opcode: CecOpcode::GiveOsdName,
    //     parameters: CecDatapacket(Default::default()),
    //     opcode_set: true,
    //     transmit_timeout: Duration::from_secs(2),
    // });

    // println!("get name: {:?}", res);

    // for address in 0..16 {
    //     let res = cec.transmit(CecCommand {
    //         initiator: CecLogicalAddress::Playbackdevice1,
    //         destination: CecLogicalAddress::try_from(address).unwrap(),
    //         ack: Default::default(),
    //         eom: Default::default(),
    //         opcode: CecOpcode::None,
    //         parameters: CecDatapacket(Default::default()),
    //         opcode_set: false,
    //         transmit_timeout: Duration::from_secs(2),
    //     });

    //     println!("pinging {:?}: {:?}", CecLogicalAddress::try_from(address).unwrap(), res);

    //     if res.is_ok() {
    //         let name_res = cec.transmit(CecCommand {
    //             initiator: CecLogicalAddress::Playbackdevice1,
    //             destination: CecLogicalAddress::try_from(address).unwrap(),
    //             ack: Default::default(),
    //             eom: Default::default(),
    //             opcode: CecOpcode::GiveOsdName,
    //             parameters: CecDatapacket(Default::default()),
    //             opcode_set: true,
    //             transmit_timeout: Duration::from_secs(2),
    //         });

    //     }

    //     thread::sleep(Duration::from_millis(100));
    // }

    thread::park();

    Ok(())
}
