{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.dream2nix = {
    url = "github:nix-community/dream2nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.nixgl.url = "github:guibou/nixGL";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, dream2nix, rust-overlay, nixgl, flake-utils }:
    let
      system = "x86_64-linux";
      toolchain = rust-overlay.packages.${system}.rust;
      pkgs = nixpkgs.legacyPackages.${system};
      GLAdapter = nixgl.packages.${system}.nixGLIntel;
    in
    let d2n = dream2nix.lib.makeFlakeOutputs rec {
      systemsFromFile = ./nix_systems;
      autoProjects = true;
      config.projectRoot = ./.;
      source = ./.;
      packageOverrides = {
        # for build-rust-package builder
        "^.*".set-toolchain.overrideRustToolchain = old: {
          cargo = toolchain;
          rustc = toolchain;
        };
        "^.*".add-pre-build-steps = {
          nativeBuildInputs = old: old ++ (with pkgs; [ cmake pkg-config ]);
          buildInputs = (with pkgs.xorg;[ libX11 libXcursor libXrandr libXi libXaw libXft libXmu libXrender libXt libxkbfile ]) ++ [ pkgs.fontconfig ];
        };
      };
    };
    in
    dream2nix.lib.dlib.mergeFlakes [
      d2n
      rec {
        devShells.${system}.default = d2n.devShells.${system}.default.overrideAttrs (old: { buildInputs = old.buildInputs ++ [ pkgs.rust-analyzer GLAdapter ]; });
        script = pkgs.writeShellScriptBin "script.sh" "${GLAdapter}/bin/nixGLIntel ${d2n.packages.${system}.sys-dashboard}/bin/sys-dashboard";
        apps.${system}.default = flake-utils.lib.mkApp { drv = script; };
      }
    ];

}
