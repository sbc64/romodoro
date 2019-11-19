with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "romodoro";

  buildInputs = [
    libnotify
    pkgconfig
    glib
    gdk-pixbuf
    alsaUtils
    alsaLib
    rustup
  ];

  # RUST_BACKTRACE = 1;
}
