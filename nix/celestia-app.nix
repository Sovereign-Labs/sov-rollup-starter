{
    nixpkgs
,   celestia-app-src
}:
let
    celestia-app = nixpkgs.buildGoModule {
        name = "celestia";
        version = "1.6.0";
        src = celestia-app-src;
        vendorHash = "sha256-KvkVqJZ5kvkKWXTYgG7+Ksz8aLhGZPBG5zkM44fVNT4=";
        doCheck = false;
    };
in
celestia-app