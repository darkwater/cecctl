//  = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libmtdev.so.1, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
//          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libevdev.so.2, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
//          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libwacom.so.9, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
//  = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libgudev-1.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
//          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libgobject-2.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
//          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libglib-2.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
//  = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libffi.so.8, needed by /nix/store/0mcsib9r75kzr8lnwrfi3wm0jbs4751l-glib-2.74.3/lib/libgobject-2.0.so, not found (try using -rpath or -rpath-link)
//          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libpcre2-8.so.0, needed by /nix/store/0mcsib9r75kzr8lnwrfi3wm0jbs4751l-glib-2.74.3/lib/libglib-2.0.so, not found (try using -rpath or -rpath-link)

fn main() {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=udev");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=Xau");
    println!("cargo:rustc-link-lib=Xdmcp");
    println!("cargo:rustc-link-lib=mtdev");
    println!("cargo:rustc-link-lib=evdev");
    println!("cargo:rustc-link-lib=wacom");
    println!("cargo:rustc-link-lib=gudev-1.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=ffi");
    println!("cargo:rustc-link-lib=pcre2-8");
}
