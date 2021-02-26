let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  rust = (nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [
      "rust-analysis"
      "clippy-preview"
      "rustfmt-preview"
      "rls-preview"
    ];
  });
in with nixpkgs;

  stdenv.mkDerivation {
    name = "shell-nix";
    buildInputs = [ rust ];

  }
