{
  description = "Haskell, Haskell über Alles";
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = {self, flake-utils, nixpkgs, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; config = {}; overlays = [];};
      in
        {
          packages.default = pkgs.callPackage ./default.nix { inherit pkgs system; };
          devShells.default = pkgs.callPackage ./shell.nix {inherit pkgs system;};
        }
    );
}
