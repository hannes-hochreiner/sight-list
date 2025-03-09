{
  description = "Sight List";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      rust-bin-custom = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
        targets = [ "x86_64-unknown-linux-gnu" ];
      };

      sight-list-cargo-toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      hashes-toml = (builtins.fromTOML (builtins.readFile ./hashes.toml));

      sight-list-deps = derivation {
        inherit system;
        name = "${sight-list-cargo-toml.package.name}-${hashes-toml.cargo_lock}-deps";
        builder = "${pkgs.nushell}/bin/nu";
        buildInputs = with pkgs; [
          rust-bin-custom
        ];
        args = [ ./builder.nu "fetch" ./. ];

        outputHashAlgo = "sha256";
        outputHashMode = "recursive";
        outputHash = hashes-toml.deps;
        # outputHash = pkgs.lib.fakeHash;
      };

      sight-list-bin = derivation {
          inherit system;
          name = "${sight-list-cargo-toml.package.name}-v${sight-list-cargo-toml.package.version}";
          builder = "${pkgs.nushell}/bin/nu";
          buildInputs = with pkgs; [
            gcc_multi
            rust-bin-custom
          ];
          args = [ ./builder.nu "build" ./. sight-list-deps ];
        };
      in
      {
        packages.${system} = {
          deps = sight-list-deps;
          bin = sight-list-bin;
          default = sight-list-bin;
        };

        devShells.${system}.default = pkgs.mkShell {
          name = "sight-list";
          
          # Inherit inputs from checks.
          # checks = self.checks.${system};

          shellHook = ''
            exec zellij -l zellij.kdl
          '';
          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          buildInputs = with pkgs; [
            zellij
            rust-bin-custom
          ];
        };
      };  
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