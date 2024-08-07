{
  description = "Simple tool to find out if a pr was merged into a nixpkgs branch. Uses the github api";

  # All inputs for the system
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };
  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} (_: let
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    in {
      inherit systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = {
        config,
        pkgs,
        lib,
        ...
      }: let
        buildInputs = with pkgs; [
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        name = "merged-yet";

        merged-yet = pkgs.rustPlatform.buildRustPackage {
          inherit buildInputs nativeBuildInputs name;

          cargoLock.lockFile = ./Cargo.lock;
          src = ./.;

          meta = with lib; {
            maintainers = [maintainers.cch000];
            mainProgram = name;
            platforms = systems;
            license = licenses.gpl3Plus;
          };
        };
      in {
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true;
            deadnix.enable = true;
            statix.enable = true;
            rustfmt.enable = true;
          };
        };
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          inputsFrom = [config.treefmt.build.devShell];

          packages = with pkgs; [
            nil
            rustc
            cargo
            clippy
            rust-analyzer
          ];
        };

        packages = {
          inherit merged-yet;
          default = merged-yet;
        };
      };
    });
}
