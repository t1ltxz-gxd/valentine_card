{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixpkgs-unstable.tar.gz") {} }:

pkgs.stdenv.mkDerivation {
  name = "valentine_card";

  buildInputs = [
    pkgs.nodejs
    pkgs.nodePackages.typescript
    pkgs.rustc
    pkgs.cargo
  ];

  src = ./.;
}