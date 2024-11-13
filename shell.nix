{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixpkgs-unstable.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs
    pkgs.nodePackages.typescript
    pkgs.rustc
    pkgs.cargo
  ];

  shellHook = ''
    echo "Welcome to your development shell!"
  '';
}