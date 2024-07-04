{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          # rust = pkgs.rust-bin.nightly."2024-05-15".default.override {
          rust = pkgs.rust-bin.stable."1.78.0".default.override {
            extensions = [
              "rust-src" # for rust-analyzer
              "rust-analyzer"
            ];
          };
        in
        with pkgs;
        {
          formatter = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
          devShells.default = mkShell {
            buildInputs = [
              rust
            ];
            nativeBuildInputs = [
              pkg-config
            ];

            RUSTFLAGS = map (a: "-C link-arg=${a}") [
              "-Wl,--push-state,--no-as-needed"
              "-Wl,--pop-state"
            ];

            # LD_LIBRARY_PATH = lib.makeLibraryPath [
            #   xorg.libXrandr
            # ];
          };
        }
      );
}
