{
    nixpkgs
,   celestia-node-src
}:
let
    celestia-node = nixpkgs.buildGoModule {
        name = "celestia-node";
        version = "0.13.0";
        src = celestia-node-src;
        vendorHash = "sha256-wUyb6gZ9n+wOBagJ1BdKcbBGtLIaVyaRH6NHSJ7VFk8=";
        doCheck = false;
    };
in
celestia-node