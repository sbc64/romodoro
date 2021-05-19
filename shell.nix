{ pkgs ? import <nixpkgs> {} }:
let
  pkgPath = "${pkgs.glib.dev}/lib/pkgconfig";
in pkgs.mkShell rec {
  name = "romodoro";
  nativeBuildInputs = with pkgs; [
    libnotify
    pkgconfig
    glib
    gdk-pixbuf
    alsaUtils
    alsaLib
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
  PKG_CONFIG_PATH= pkgPath;
  shellHook = ''
    export PATH="$PWD/node_modules/.bin/:$PATH"
  '';
}

