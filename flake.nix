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
      nativeBuildInputs = with pkgs; [
        gcc
        pkg-config
      ];
      buildInputs = with pkgs; [
        openssl
      ];
      OPENSSL_DIR = "${pkgs.openssl.dev}";
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      OPENSSL_NO_VENDOR = 1;
      OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
      DISCORD_TOKEN = "<token>";
      shellHook = ''
        fish
      '';
    };
  };
}
