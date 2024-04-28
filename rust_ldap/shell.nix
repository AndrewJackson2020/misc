let 
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "music-reader-env";
    buildInputs = [ 
      cargo
      rustc
      pkg-config
      openssl.dev 
      nix
      ];
    OPENSSL_DEV=openssl.dev;
  }
