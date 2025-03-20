{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "rust-monitoring";
  version = "0.2";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativebuildInputs = [ pkgs.libpcap ];
  buildInputs = [ pkgs.libpcap ];

  meta = with pkgs; {
    description = "My simple system monitoring tool";
    license = lib.licenses.mit;
    maintainers = [ "evcareeria" "tuuhoo" ];
  };
}
