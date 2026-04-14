{pkgs, config, lib, ...}:
let
  # octocore = config.languages.rust.import ./. { };
in
{
  languages.nix = {
    lsp.enable = true;
    lsp.package = pkgs.nixd;
  };
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "latest";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };
  packages = [ 
    pkgs.alsa-lib
    pkgs.freetype
    pkgs.fontconfig
    pkgs.pkg-config
    pkgs.python3
    # octocore
  ];
  outputs = {
    # inherit octocore;
  };
}
