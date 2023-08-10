{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    # Basic tools
    pkgs.cargo
    pkgs.rustc
    pkgs.pkgconfig

    # For Druid and its dependencies
    pkgs.gtk3
    pkgs.cairo
    pkgs.pango
    pkgs.atk
    pkgs.gdk-pixbuf
  ];

  # Ensure Druid can find GTK and other dependencies
  PKG_CONFIG_PATH = "${pkgs.gtk3}/lib/pkgconfig:${pkgs.cairo}/lib/pkgconfig:${pkgs.pango}/lib/pkgconfig:${pkgs.atk}/lib/pkgconfig:${pkgs.gdk-pixbuf}/lib/pkgconfig";

  # Other environment settings if needed
  shellHook = ''
    echo "Welcome to the Druid development environment!"
  '';
}

