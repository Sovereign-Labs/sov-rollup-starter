{
    nixpkgs
,   rust-bin
,   risc0-rust
,   sovereign-sdk-src
,   risc0-cycle-macros-src
,   risc0-circuit
}:
let
    rollup-guest-src = nixpkgs.stdenv.mkDerivation {
        name = "rollup-guest-src";
        src = ../crates;
        dontBuild = true;

        installPhase = ''
            mkdir -p $out/crates $out/vendor
            cp -r . $out/crates
            cp -r ${sovereign-sdk-src} $out/vendor/sovereign-sdk
            cp -r ${risc0-cycle-macros-src} $out/vendor/risc0-cycle-macros
            cp ${../Cargo.toml} $out/Cargo.toml
            cp ${../constants.json} $out/constants.json
        '';
    };


    rollup-src = nixpkgs.stdenv.mkDerivation {
        name = "rollup-src";
        src = ../crates;
        dontBuild = true;

        installPhase = ''
            mkdir -p $out/crates $out/vendor
            cp -r . $out/crates
            cp -r ${sovereign-sdk-src} $out/vendor/sovereign-sdk
            cp -r ${risc0-cycle-macros-src} $out/vendor/risc0-cycle-macros
            cp ${../Cargo.toml} $out/Cargo.toml
            cp ${../Cargo.lock} $out/Cargo.lock
            cp ${../constants.json} $out/constants.json
        '';
    };

    rollup-guest-mock = nixpkgs.rustPlatform.buildRustPackage {
        name = "rollup-guest-mock";

        src = rollup-guest-src;

        sourceRoot = "rollup-guest-src/crates/provers/risc0/guest-mock";

        cargoLock = {
            lockFile = ../crates/provers/risc0/guest-mock/Cargo.lock;
            outputHashes = {
                "jmt-0.9.0" = "sha256-pq1v6FXS//6Dh+fdysQIVp+RVLHdXrW5aDx3263O1rs=";
                "crypto-bigint-0.5.2" = "sha256-9rh8z3vwOQ7/mtzVbyADoRWgTzARF/nkhBwfKb7+A6I=";
                "curve25519-dalek-4.1.0" = "sha256-H8YMea3AIcUn9NGRfataNjCTzCK4NAjo4ZhWuPfT6ts=";
                "sha2-0.10.8" = "sha256-vuFQFlbDXEW+n9+Nx2VeWanggCSd6NZ+GVEDFS9qZ2M=";
                "ibc-app-transfer-0.50.0" = "sha256-8iWoYw9xX1D/Z+H7IVUP4AoEI4LjL3jzseAOvPBDFbU=";
                "ibc-proto-0.41.0" = "sha256-OXqtIFDK5KdYW39EkNGGtfuDvOAMjmxzfnSpm1NWpRc=";
                "risc0-cycle-utils-0.3.0" = "sha256-tl6TvAUghcJvlnbD1iYH4mHjgSEtNKsAYN9ZZP69pyc=";
                "sov-celestia-client-0.1.0" = "sha256-5o3GYYXfpcqI5qyCSzIKbYmm/wj2Zs+k+6WoVctvfW0=";
            };
        };

        nativeBuildInputs = [
            rust-bin
            nixpkgs.lld
        ];

        doCheck = false;

        CONSTANTS_MANIFEST = rollup-guest-src;

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

    rollup-guest-celestia = nixpkgs.rustPlatform.buildRustPackage {
        name = "rollup-guest-celestia";

        src = rollup-guest-src;

        sourceRoot = "rollup-guest-src/crates/provers/risc0/guest-celestia";

        cargoLock = {
            lockFile = ../crates/provers/risc0/guest-celestia/Cargo.lock;
            outputHashes = {
                "celestia-proto-0.1.0" = "sha256-iUgrctxdJUyhfrEQ0zoVj5AKIqgj/jQVNli5/K2nxK0=";
                "crypto-bigint-0.5.2" = "sha256-9rh8z3vwOQ7/mtzVbyADoRWgTzARF/nkhBwfKb7+A6I=";
                "curve25519-dalek-4.1.0" = "sha256-H8YMea3AIcUn9NGRfataNjCTzCK4NAjo4ZhWuPfT6ts=";
                "ibc-app-transfer-0.50.0" = "sha256-8iWoYw9xX1D/Z+H7IVUP4AoEI4LjL3jzseAOvPBDFbU=";
                "ibc-proto-0.41.0" = "sha256-OXqtIFDK5KdYW39EkNGGtfuDvOAMjmxzfnSpm1NWpRc=";
                "jmt-0.9.0" = "sha256-pq1v6FXS//6Dh+fdysQIVp+RVLHdXrW5aDx3263O1rs=";
                "nmt-rs-0.1.0" = "sha256-jcHbqyIKk8ZDDjSz+ot5YDxROOnrpM4TRmNFVfNniwU=";
                "sha2-0.10.8" = "sha256-vuFQFlbDXEW+n9+Nx2VeWanggCSd6NZ+GVEDFS9qZ2M=";
                "sov-celestia-client-0.1.0" = "sha256-5o3GYYXfpcqI5qyCSzIKbYmm/wj2Zs+k+6WoVctvfW0=";
                "tendermint-0.32.0" = "sha256-FtY7a+hBvQryATrs3mykCWFRe8ABTT6cuf5oh9IBElQ=";
                "risc0-cycle-utils-0.3.0" = "sha256-tl6TvAUghcJvlnbD1iYH4mHjgSEtNKsAYN9ZZP69pyc=";
            };
        };

        nativeBuildInputs = [
            rust-bin
            nixpkgs.lld
            nixpkgs.protobuf
        ];

        doCheck = false;

        CONSTANTS_MANIFEST = rollup-guest-src;

        buildPhase = ''
            RUSTC=${risc0-rust}/bin/rustc \
                CARGO_ENCODED_RUSTFLAGS=$'-C\x1fpasses=loweratomic\x1f-C\x1flink-arg=-Ttext=0x00200800\x1f-C\x1flink-arg=--fatal-warnings\x1f-C\x1fpanic=abort\x1f-C\x1flinker=lld' \
                cargo build --release --target riscv32im-risc0-zkvm-elf -p sov-demo-prover-guest-celestia
        '';

        installPhase = ''
            mkdir -p $out
            cp target/riscv32im-risc0-zkvm-elf/release/rollup $out/
        '';
    };


    rollup = nixpkgs.rustPlatform.buildRustPackage {
        name = "sov-rollup-starter";

        src = rollup-src;

        cargoLock = {
            lockFile = ../Cargo.lock;
            outputHashes = {
                "celestia-proto-0.1.0" = "sha256-iUgrctxdJUyhfrEQ0zoVj5AKIqgj/jQVNli5/K2nxK0=";
                "jmt-0.9.0" = "sha256-pq1v6FXS//6Dh+fdysQIVp+RVLHdXrW5aDx3263O1rs=";
                "nmt-rs-0.1.0" = "sha256-jcHbqyIKk8ZDDjSz+ot5YDxROOnrpM4TRmNFVfNniwU=";
                "tendermint-0.32.0" = "sha256-FtY7a+hBvQryATrs3mykCWFRe8ABTT6cuf5oh9IBElQ=";
                "crypto-bigint-0.5.2" = "sha256-9rh8z3vwOQ7/mtzVbyADoRWgTzARF/nkhBwfKb7+A6I=";
                "curve25519-dalek-4.1.0" = "sha256-H8YMea3AIcUn9NGRfataNjCTzCK4NAjo4ZhWuPfT6ts=";
                "ibc-0.50.0" = "sha256-8iWoYw9xX1D/Z+H7IVUP4AoEI4LjL3jzseAOvPBDFbU=";
                "ibc-proto-0.41.0" = "sha256-OXqtIFDK5KdYW39EkNGGtfuDvOAMjmxzfnSpm1NWpRc=";
                "sov-celestia-client-0.1.0" = "sha256-5o3GYYXfpcqI5qyCSzIKbYmm/wj2Zs+k+6WoVctvfW0=";
                "risc0-cycle-utils-0.3.0" = "sha256-tl6TvAUghcJvlnbD1iYH4mHjgSEtNKsAYN9ZZP69pyc=";
            };
        };

        doCheck = false;
        # buildType = "debug";
        buildNoDefaultFeatures = true;
        buildFeatures = [ "celestia_da" ];

        PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

        ROLLUP_ELF_PATH = "${rollup-guest-celestia}/rollup";
        RECURSION_SRC_PATH = "${risc0-circuit}";
        CONSTANTS_MANIFEST = rollup-src;

        nativeBuildInputs = [
            nixpkgs.pkg-config
            nixpkgs.protobuf
            nixpkgs.rustPlatform.bindgenHook
        ];

    };
in
{
    inherit rollup rollup-guest-celestia rollup-guest-mock;
}