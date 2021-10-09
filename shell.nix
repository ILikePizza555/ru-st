{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = ["rust-src"];
    })
  ];
}
