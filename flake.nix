{
    description = "Nix development dependencies for ibc-rs";

    inputs = {
        nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;

        flake-utils.url = github:numtide/flake-utils;

        rust-overlay.url = github:oxalica/rust-overlay;
    };

    outputs = inputs:
        let
            utils = inputs.flake-utils.lib;
        in
            utils.eachSystem
            [
            "x86_64-linux"
            ]
            (system: let
                nixpkgs = import inputs.nixpkgs {
                    inherit system;
                    overlays = [
                        inputs.rust-overlay.overlays.default
                    ];
                };

                rust-bin = nixpkgs.rust-bin.stable.latest.complete;

                risc0-rust-tarball = builtins.fetchurl {
                url = "https://github.com/risc0/rust/releases/download/test-release-2/rust-toolchain-x86_64-unknown-linux-gnu.tar.gz";
                sha256 = "sha256:1nqgpx6ww0rla5c4jzam6fr43v6lf0flsj572racjqwq9xk86l4a";
                };

                risc0-rust = import ./nix/risc0.nix {
                    inherit nixpkgs rust-bin risc0-rust-tarball;
                };

                rollup-packages = import ./nix/rollup.nix {
                    inherit nixpkgs rust-bin risc0-rust;
                };
            in {
                packages = {
                    inherit risc0-rust;
                    inherit (rollup-packages) rollup rollup-guest-mock rollup-guest-celestia;
                };
            });
}
