{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.vscode
    pkgs.libpcap
  ];
}

