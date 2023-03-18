{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.pkgconfig
    pkgs.dbus
  ];
}

