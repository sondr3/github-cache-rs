with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "github-cache-rs";
  buildInputs = with pkgs; [
    pkgconfig
    openssl
  ];
}
