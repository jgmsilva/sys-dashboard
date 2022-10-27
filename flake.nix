{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.dream2nix.url = "github:nix-community/dream2nix";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  outputs = {self, nixpkgs, dream2nix, rust-overlay}:
    let 
      system = "x86_64-linux";
      toolchain = rust-overlay.packages.${system}.rust;
      pkgs = nixpkgs.legacyPackages.${system};
    in dream2nix.lib.makeFlakeOutputs {
      systemsFromFile = ./nix_systems;
      config.projectRoot = ./.;
      source = ./.;
      packageOverrides = rec {
      # for build-rust-package builder
        "^.*".set-toolchain.overrideRustToolchain = old: {
          cargo = toolchain;
          rustc = toolchain;
        };
        buildInputs = old: old ++ [pkgs.xorg.libX11 pkgs.xorg.libX11.dev];
        LD_LIBRARY_PATH = "/home/joao/.config/nixpkgs";
      };
    };
}
