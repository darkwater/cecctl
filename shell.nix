with import <nixpkgs> {
  overlays = [
    (import (builtins.fetchGit {
      url = "https://github.com/darkwater/onyx";
      ref = "master";
    }) {}).overlay
  ];
};

onyx-shells.rust {
  name = "cecctl";
  # nightly = "2020-12";

  buildInputs = [ cmake libcec pkg-config xorg.libX11 xorg.libXi xorg.libXtst ];
}
