{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          pkgs.mkShell.override
            {
              stdenv = pkgs.llvmPackages.libcxxStdenv;
            }
            {
              buildInputs = with pkgs; [
                openssl
                pkg-config
                llvmPackages.clangUseLLVM
                (rust-bin.nightly.latest.default.override {
                  extensions = [ "llvm-tools-preview" ];
                })
                cargo-fuzz
                cargo-tarpaulin
                cargo-llvm-cov
              ];

              CC = "clang";
              CXX = "clang++";
              TARGET_CC = "clang";
              TARGET_CXX = "clang++";
              HOST_CC = "clang";
              HOST_CXX = "clang++";
              CXXFLAGS = "-stdlib=libc++";
              CXXSTDLIB = "c++";
              RUSTFLAGS = "-C linker=clang++ -C link-arg=-lc++";
            };
      }
    );
}
