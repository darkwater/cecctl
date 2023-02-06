#   = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libmtdev.so.1, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
#           /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libevdev.so.2, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
#           /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libwacom.so.9, needed by /nix/store/srx0ivm8sa3mxqc49wmml423ffwdkd8l-libinput-1.21.0/lib/libinput.so, not found (try using -rpath or -rpath-link)
#  = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libgudev-1.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
#          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libgobject-2.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
#          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libglib-2.0.so.0, needed by /nix/store/wdbmadc3ra88cmkbnxdja6wynf1wsl9l-libwacom-2.4.0/lib/libwacom.so, not found (try using -rpath or -rpath-link)
#  = note: /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libffi.so.8, needed by /nix/store/0mcsib9r75kzr8lnwrfi3wm0jbs4751l-glib-2.74.3/lib/libgobject-2.0.so, not found (try using -rpath or -rpath-link)
#          /nix/store/r2b9k28c6aghczpqfvh71y9xavm7rr68-binutils-2.39/bin/ld: warning: libpcre2-8.so.0, needed by /nix/store/0mcsib9r75kzr8lnwrfi3wm0jbs4751l-glib-2.74.3/lib/libglib-2.0.so, not found (try using -rpath or -rpath-link)

{
  inputs = {
    naersk = { url = "github:nix-community/naersk"; inputs.nixpkgs.follows = "nixpkgs"; };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        inputs = with pkgs; [
          cargo rustc rustfmt pre-commit rustPackages.clippy
          libcec_platform udev
          cmake libcec pkg-config xorg.libX11 xorg.libXi xorg.libXtst
          xorg.libXtst xorg.libxcb xorg.libXext # libstdcxx5
          stdenv.cc.cc.lib
          xorg.libXau xorg.libXdmcp
          libinput
          mtdev libevdev libwacom
          glib libgudev
          libffi pcre2
        ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = inputs;
          nativeBuildInputs = inputs;
        };
        devShell = with pkgs; mkShell {
          buildInputs = inputs;
          nativeBuildInputs = inputs;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
