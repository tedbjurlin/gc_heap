{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {inherit system overlays;};

      toolchain = pkgs.rust-bin.selectLatestNightlyWith (
        toolchain:
          toolchain.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "cargo"
              "llvm-tools-preview"
            ];
          }
      );

    in {
      toolchain = toolchain;
      devShell = pkgs.mkShell {
        name = "OS";
        buildInputs = [
          toolchain
          pkgs.cargo-bootimage
        ];
        # nativeBuildInputs = with pkgs; [
        #   rustc
        #   cargo
        #   rustfmt
        # ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        packages = with pkgs; [
          just
          qemu
        ];
      };
    });
}
