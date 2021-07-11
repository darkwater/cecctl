use std::{mem, thread};

use cec_rs::{CecCommand, CecConnectionCfgBuilder, CecDeviceType, CecDeviceTypeVec, CecLogicalAddress, CecOpcode, CecUserControlCode};
use rdev::{EventType, Key};

fn main() {
    let mut last_pressed = None;
    let cmd_handler = move |e: CecCommand| {
        println!("{:?} -> {:?}: {:?}\n{:?}", e.initiator, e.destination, e.opcode, e.parameters);

        match (e.opcode, e.parameters.0.as_slice()) {
            (CecOpcode::UserControlPressed, [ code ]) => {
                println!("pressed {:?}", unsafe { mem::transmute::<_, CecUserControlCode>(*code as u32) });

                last_pressed = match unsafe { mem::transmute(*code as u32) } {
                    CecUserControlCode::Up       => Some(Key::UpArrow),
                    CecUserControlCode::Down     => Some(Key::DownArrow),
                    CecUserControlCode::Left     => Some(Key::LeftArrow),
                    CecUserControlCode::Right    => Some(Key::RightArrow),
                    CecUserControlCode::Select   => Some(Key::Return),
                    CecUserControlCode::Exit     => Some(Key::Escape),
                    CecUserControlCode::Play |
                    CecUserControlCode::Pause    => Some(Key::Space),
                    CecUserControlCode::F2Red    => Some(Key::F1),
                    CecUserControlCode::F3Green  => Some(Key::F2),
                    CecUserControlCode::F4Yellow => Some(Key::F3),
                    CecUserControlCode::F1Blue   => Some(Key::F4),
                    _ => None
                };

                if let Some(key) = last_pressed {
                    rdev::simulate(&EventType::KeyPress(key)).expect("key press");
                }
            }

            (CecOpcode::UserControlRelease, _) => {
                println!("released");
                if let Some(key) = last_pressed.take() {
                    rdev::simulate(&EventType::KeyRelease(key)).expect("key release");
                }
            }

            _ => {}
        }
    };

    let cec = CecConnectionCfgBuilder::default()
        .port("/dev/ttyACM0".to_owned())
        .device_name("Sinon".to_owned())
        .device_types(CecDeviceTypeVec::new(CecDeviceType::PlaybackDevice))
        .command_received_callback(Box::new(cmd_handler))
        .build()
        .expect("build cec config")
        .open()
        .expect("open cec connection");

    cec.set_active_source(CecDeviceType::PlaybackDevice).expect("set active source");

    thread::park();
}
