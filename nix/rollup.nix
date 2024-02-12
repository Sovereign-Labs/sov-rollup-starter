{
    nixpkgs
,   rust-bin
,   risc0-rust
}:
let
    rollup-guest-src = nixpkgs.stdenv.mkDerivation {
        name = "rollup-guest-src";
        src = ../crates;
        dontBuild = true;

        installPhase = ''
            mkdir -p $out/crates
            cp -r . $out/crates
            cp ${../Cargo.toml} $out/Cargo.toml
            cp ${../constants.json} $out/constants.json
        '';
    };

    rollup-src = nixpkgs.stdenv.mkDerivation {
        name = "rollup-guest-src";
        src = ../crates;
        dontBuild = true;

        installPhase = ''
            mkdir -p $out/crates
            cp -r . $out/crates
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
            lockFile = ./crates/provers/risc0/guest-mock/Cargo.lock;
            outputHashes = {
                "sov-accounts-0.3.0" = "sha256-Bmzo0xe1GdSKEIAyYx0PHhauNdVBMawOSSIflhdfi6U=";
                "jmt-0.9.0" = "sha256-pq1v6FXS//6Dh+fdysQIVp+RVLHdXrW5aDx3263O1rs=";
                "risc0-binfmt-0.19.1" = "sha256-Av3rpNhDny8FroOcn8eyvZcR8hFSNukA7n9impm1HHU=";
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
                "jmt-0.9.0" = "sha256-pq1v6FXS//6Dh+fdysQIVp+RVLHdXrW5aDx3263O1rs=";
                "nmt-rs-0.1.0" = "sha256-jcHbqyIKk8ZDDjSz+ot5YDxROOnrpM4TRmNFVfNniwU=";
                "sov-accounts-0.3.0" = "sha256-Bmzo0xe1GdSKEIAyYx0PHhauNdVBMawOSSIflhdfi6U=";
                "tendermint-0.32.0" = "sha256-FtY7a+hBvQryATrs3mykCWFRe8ABTT6cuf5oh9IBElQ=";
                "risc0-binfmt-0.19.1" = "sha256-Av3rpNhDny8FroOcn8eyvZcR8hFSNukA7n9impm1HHU=";
                "ibc-app-transfer-0.50.0" = "sha256-+F6/YQ2wGhCFfuWJRQbhJx+cfurT/8hh8hkw1FAEiPM=";
                "ibc-proto-0.41.0" = "sha256-uUfB6K/WuLb2+OMX8MB2r5ptFsgkF3OVbBWFFMRdlTw=";
                "risc0-cycle-utils-0.3.0" = "sha256-nWDM/GJkpXvlqOzRKKiAZTBVHRqxE54dvkNeJ2SH6UM=";
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
                "sov-accounts-0.3.0" = "sha256-Bmzo0xe1GdSKEIAyYx0PHhauNdVBMawOSSIflhdfi6U=";
                "tendermint-0.32.0" = "sha256-FtY7a+hBvQryATrs3mykCWFRe8ABTT6cuf5oh9IBElQ=";
                "bonsai-sdk-0.5.1" = "sha256-vBcJIbMMYmWhU/NHMODm+8HxXbF+tBjB/DV4HYwlVo0=";
                "ibc-app-transfer-0.50.0" = "sha256-+F6/YQ2wGhCFfuWJRQbhJx+cfurT/8hh8hkw1FAEiPM=";
                "ibc-proto-0.41.0" = "sha256-uUfB6K/WuLb2+OMX8MB2r5ptFsgkF3OVbBWFFMRdlTw=";
                "risc0-cycle-utils-0.3.0" = "sha256-nWDM/GJkpXvlqOzRKKiAZTBVHRqxE54dvkNeJ2SH6UM=";
            };
        };

        doCheck = false;
        buildNoDefaultFeatures = true;
        buildFeatures = [ "celestia_da" ];

        PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

        ROLLUP_ELF_PATH = "${rollup-guest-celestia}/rollup";
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