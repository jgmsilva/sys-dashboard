{
  inputs = {
    # Use a github flake URL for real packages
    cargo2nix.url = "github:cargo2nix/cargo2nix/unstable";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = inputs: with inputs; # pass through all inputs and bring them into scope

    # Build the output set for each default system and map system sets into
    # attributes, resulting in paths such as:
    # nix build .#packages.x86_64-linux.<name>
    flake-utils.lib.eachDefaultSystem (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.64.0";
          packageFun = import ./Cargo.nix;
          # packageOverrides = pkgs: pkgs.rustBuilder.overrides.all; # Implied, if not specified
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#sys-dashboard
          # nix build .#packages.x86_64-linux.sys-dashboard
          sys-dashboard = (rustPkgs.workspace.sys-dashboard {}).bin;
          # nix build
          default = packages.sys-dashboard;
        };
      }
    );
}
