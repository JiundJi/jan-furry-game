{
  description = "Haskell, Haskell über Alles";
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
  };
  outputs = {self, nixpkgs, ...}:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; config = {}; overlays = [];};
    in
      {
        packages.${system}.default = pkgs.callPackage ./default.nix { inherit system; };
        devShells.${system}.default = pkgs.callPackage ./shell.nix {inherit pkgs;};
      };
}
