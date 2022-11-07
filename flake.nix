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
        "^.*".add-pre-build-steps.buildInputs = old: old ++ (with pkgs.xorg;[libX11 libXcursor libXrandr libXi]);
        LD_LIBRARY_PATH = "/home/joao/.config/nixpkgs";
      };
      settings = [  {devShells.env = [{name = "TESTING_VAR"; value = "/home/"; }];} ];
    };
}
