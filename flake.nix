{
  description = "Customizable sound pomodoro timer";
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    flake-compat,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
      buildInputs = with pkgs; [
        openssl
        libnotify
        pkgconfig
        glib
        gdk-pixbuf
        alsaUtils
        alsaLib
      ];
    in {
      defaultPackage = naersk-lib.buildPackage {
        root = ./.;
        buildInputs = buildInputs;
        nativeBuildInputs = buildInputs;
      };

      defaultApp = utils.lib.mkApp {
        drv = self.defaultPackage."${system}";
      };

      devShell = with pkgs;
        mkShell {
          buildInputs = [pkg-config cargo rustup];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    });
}
