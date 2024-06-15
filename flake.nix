{
  description = "Sight List";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system}.overrideToolchain
            fenix.packages.${system}.minimal.toolchain;
        sight-list = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          buildInputs = [
            # Add additional build inputs here
          ];

          postInstall = ''
            mkdir $out/var
            cp -r static $out/var
            cp -r templates $out/var
          '';
        };
      in
      {
        checks = {
          inherit sight-list;
        };

        packages.default = sight-list;

        apps.default = flake-utils.lib.mkApp {
          drv = sight-list;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      }
    );
  
  # nixConfig = {
  #   substituters = [
  #     "https://cache.nixos.org"
  #     "https://hannes-hochreiner.cachix.org"
  #   ];
  #   trusted-public-keys = [
  #     "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
  #     "hannes-hochreiner.cachix.org-1:+ljzSuDIM6I+FbA0mdBTSGHcKOcEZSECEtYIEcDA4Hg="
  #   ];
  # };
}