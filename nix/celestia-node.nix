{
    nixpkgs
,   celestia-node-src
}:
let
    celestia-node = nixpkgs.buildGoModule {
        name = "celestia-node";
        version = "0.12.4";
        src = celestia-node-src;
        vendorHash = "sha256-X00vCmGN8QQPU/c2/gccwcosk+TfCASMoiN0uBdXQAA=";
        doCheck = false;
    };
in
celestia-node