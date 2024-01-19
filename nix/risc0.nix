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
in
risc0-rust