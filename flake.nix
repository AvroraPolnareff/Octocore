{
  description = "Description for the project";

  inputs = {
    devshell.url = "github:numtide/devshell";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };
          devShells.default = pkgs.mkShell {
            buildInputs = [
              pkgs.alsa-lib
              pkgs.freetype
              pkgs.fontconfig
            ];
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.rust-bin.stable.latest.default
            ];
          };
          treefmt.programs.nixfmt = {
            enable = true;
            package = pkgs.nixfmt;
          };
        };
    };
}
