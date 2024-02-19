{
    description = "Nix development dependencies for ibc-rs";

    inputs = {
        nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;

        flake-utils.url = github:numtide/flake-utils;

        rust-overlay.url = github:oxalica/rust-overlay;

        sovereign-sdk-src = {
            flake = false;
            url = git+ssh://git@github.com/informalsystems/sovereign-sdk-wip?rev=a1d9ed80af46a0ea6e173204ca708c40ce592d3f;
        };

        celestia-app-src = {
            flake = false;
            url = github:celestiaorg/celestia-app/v1.6.0;
        };

        celestia-node-src = {
            flake = false;
            url = github:celestiaorg/celestia-node/v0.13.0;
        };

        gaia-src = {
            flake = false;
            url = github:cosmos/gaia/v14.1.0;
        };
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
                    config = {
                        permittedInsecurePackages = [
                            "openssl-1.1.1w"
                        ];
                    };
                };

                rust-bin = nixpkgs.rust-bin.stable.latest.complete;

                risc0-rust-tarball = builtins.fetchurl {
                    url = "https://github.com/risc0/rust/releases/download/v2024-01-31.1/rust-toolchain-x86_64-unknown-linux-gnu.tar.gz";
                    sha256 = "sha256:05k8d47zcrascjwwas9pnzg6qz5ambxvfh485flxsn6l7hxq3jf0";
                };

                risc0-rust = import ./nix/risc0.nix {
                    inherit nixpkgs rust-bin risc0-rust-tarball;
                };

                rollup-packages = import ./nix/rollup.nix {
                    inherit nixpkgs rust-bin risc0-rust;
                    inherit (inputs) sovereign-sdk-src;
                };

                gaia = import ./nix/gaia.nix {
                    inherit nixpkgs;

                    inherit (inputs) gaia-src;
                };

                celestia-app = import ./nix/celestia-app.nix {
                    inherit nixpkgs;

                    inherit (inputs) celestia-app-src;
                };

                celestia-node = import ./nix/celestia-node.nix {
                    inherit nixpkgs;

                    inherit (inputs) celestia-node-src;
                };
            in {
                packages = {
                    inherit risc0-rust gaia celestia-app celestia-node;
                    inherit (rollup-packages) rollup rollup-guest-mock rollup-guest-celestia;
                };
            });
}
