{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;

  };

  outputs = inputs: let
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

        risc0-rust = nixpkgs.stdenv.mkDerivation {
            name = "risc0-rust";

            unpackPhase = "true";

            nativeBuildInputs = [
                rust-bin
                nixpkgs.zlib
                nixpkgs.autoPatchelfHook
            ];

            dontBuild = true;

            installPhase = ''
                mkdir -p $out
                cd $out
                tar xzf ${risc0-rust-tarball}
                chmod +x bin/*
                runHook postInstall
            '';
        };

        rollup-guest-src = nixpkgs.stdenv.mkDerivation {
            name = "rollup-guest-src";
            src = ./.;
            dontBuild = true;

            installPhase = ''
              mkdir -p $out
              cp -r crates $out/
              cp Cargo.toml constants.json $out/
            '';
        };

        rollup-guest = nixpkgs.rustPlatform.buildRustPackage {
            name = "rollup-guest";

            src = rollup-guest-src;

            sourceRoot = "rollup-guest-src/crates/provers/risc0/guest-mock";

            cargoLock = {
                lockFile = ./crates/provers/risc0/guest-mock/Cargo.lock;
                outputHashes = {
                  "sha2-0.10.6" = "sha256-1dDg6mujDC+vp9buyErWKq+pml2+xsjifxDDyiuoq8M=";
                  "sov-accounts-0.3.0" = "sha256-AnF5Ly5dWFgRSt1DDBX2GXE0WtHsctCVojgAkAqs7HM=";
                };
            };

            cargoSha256 = "";

            nativeBuildInputs = [
                rust-bin
                nixpkgs.lld
            ];

            doCheck = false;

            buildPhase = ''
                RUSTC=${risc0-rust}/bin/rustc \
                    CARGO_ENCODED_RUSTFLAGS=$'-C\x1fpasses=loweratomic\x1f-C\x1flink-arg=-Ttext=0x00200800\x1f-C\x1flink-arg=--fatal-warnings\x1f-C\x1fpanic=abort\x1f-C\x1flinker=lld' \
                    cargo build --release --target riscv32im-risc0-zkvm-elf -p guest-mock-starter
            '';

            installPhase = ''
                mkdir -p $out
                cp target/riscv32im-risc0-zkvm-elf/release/mock_da $out/
            '';
        };
    in {
      packages = {
        inherit risc0-rust rollup-guest;
      };
    });
}
