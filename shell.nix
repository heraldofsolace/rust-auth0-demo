{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  buildInputs = with pkgs; [
    cargo
    rustc
    openssl # Common dependency for many crates
  ];
  
  # Tells cargo where to find specific libraries if you get linker errors
  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkg-config";
  '';
}
