{
  description = "Flake for discord-ollama rust project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      # some dependencies, OPENSSL is required for the ollama rust lib.
      # i might be forgetting some dependency that i already have installed on my system, if you find one open a pr adding it.
      # also i don't remember what gcc is for ¯\_(ツ)_/¯
      nativeBuildInputs = with pkgs; [
        gcc
        pkg-config
        cargo
        rustc
      ];
      buildInputs = with pkgs; [
        openssl
      ];

      ### required env vars for OPENSSL to work properly under nixos.
      OPENSSL_DIR = "${pkgs.openssl.dev}";
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      OPENSSL_NO_VENDOR = 1;
      OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
      # bot token
      DISCORD_TOKEN = "<token>";
      shellHook = ''
        fish
      '';
    };
    # i use this to run the discord bot.
    devShells.x86_64-linux.run = pkgs.mkShell {
      DISCORD_TOKEN = "<discord_token>";
      shellHook = ''
        ./target/debug/discord-ollama
      '';
    };
  };
}
