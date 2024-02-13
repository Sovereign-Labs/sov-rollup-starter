{
    nixpkgs
,   rust-bin
,   risc0-rust-tarball
}:
let
    risc0-rust = nixpkgs.stdenv.mkDerivation {
        name = "risc0-rust";

        unpackPhase = "true";

        nativeBuildInputs = [
            rust-bin
            nixpkgs.zlib
            nixpkgs.stdenv.cc.cc.lib
            nixpkgs.autoPatchelfHook
            nixpkgs.openssl_1_1
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
in
risc0-rust