{ pkgs ? import <nixpkgs> {} }:
let
  pkgPath = "${pkgs.glib.dev}/lib/pkgconfig";
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "romodoro";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "0bfr5pckkgybmzqwsvswwsn3bxhcl9qnis3lkjq9dmlngjbq8b77";
  nativeBuildInputs = with pkgs; [
    libnotify
    pkgconfig
    glib
    gdk-pixbuf
    alsaUtils
    alsaLib
  ];
  buildInputs = nativeBuildInputs;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
  PKG_CONFIG_PATH= pkgPath;
}
