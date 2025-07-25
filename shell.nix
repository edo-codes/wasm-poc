{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    # Rust
    cargo
    rustc
    clippy
    rust-analyzer
    rustfmt
    wasm-pack
    lld
    cargo-watch

    # TypeScript
    nodejs_24
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

}
