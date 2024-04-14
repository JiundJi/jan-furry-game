{ system ? builtins.currentSystem
, pkgs ? import <nixpkgs> { inherit system; config = {}; overlays = []; }
}:

(import ./default.nix {inherit system pkgs;}).shell
