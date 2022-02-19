{
  description = "Nix Debugging Envionment";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/release-21.11;
  inputs.flake-utils.url = github:numtide/flake-utils;
  inputs.crate2nix = {
    url = github:kolloch/crate2nix;
    flake = false;
  };

  outputs = { self, nixpkgs, flake-utils, crate2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        generatedCargoNix = (import "${crate2nix}/tools.nix" { inherit pkgs; }).generatedCargoNix;
      in
      rec {
        packages = {
          nde-core = (import (generatedCargoNix {
            name = "nde-core";
            src = ./nde-core;
          }) {
            inherit pkgs;
          }).rootCrate.build;
        };
        defaultPackage = packages.nde-core;
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-edit
            pkgs.nixpkgs-fmt
          ];
        };
      }
    );
}
