{
  description = "Haskell, Haskell über Alles";
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/23.11";
    };
  };
  outputs = {self, nixpkgs, ...}:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; config = {}; overlays = [];};
    in
      {
        packages.${system}.default = pkgs.callPackage ./default.nix { inherit system; };
      };
}
