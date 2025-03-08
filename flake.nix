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

      sight-list-deps = pkgs.stdenv.mkDerivation {
        pname = "${sight-list-cargo-toml.package.name}-deps";
        version = sight-list-cargo-toml.package.version;
        src = ./.;
        doCheck = false;
        dontFixup = true;
        nativeBuildInputs = with pkgs; [ rust-bin-custom ];
        buildPhase = ''
          runHook preBuild
          cd $src
          mkdir -p $out/cargo_home
          CARGO_HOME=$out/cargo_home cargo fetch
          runHook postBuild
        '';
        installPhase = ''
          runHook preInstall
          cp -r .. $out
          runHook postInstall
        '';
        outputHashAlgo = "sha256";
        outputHashMode = "recursive";
        # outputHash = "sha256-CYQALaf3QL+I651FRmBmabeRaZ/NFFJzi5W3Gr7YHv8=";
        outputHash = pkgs.lib.fakeHash;
      };
      in
      {
        packages.${system} = {
          "sight-list" = pkgs.stdenv.mkDerivation {
            pname = sight-list-cargo-toml.package.name;
            version = sight-list-cargo-toml.package.version;

            src = sight-list-deps;

            buildInputs = with pkgs; [
              rust-bin-custom
            ];

            dontUnpack = true;
            dontPatch = true;
            dontConfigure = true;
            buildPhase = ''
              cd $src
              mkdir -p $out/cargo_target
              mkdir -p $out/bin
              CARGO_HOME=$src/cargo_home CARGO_TARGET_DIR=$out/cargo_target cargo build --release --offline --frozen --verbose
              cp $out/cargo_target/release/sightlist $out/bin/sightlist
              rm -r $out/cargo_target
              cp -r static $out/var
              cp -r templates $out/var
            '';
            dontInstall = true;

            # builder = ./builder.sh;
          };
        };
        # packages.default = webapp;

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