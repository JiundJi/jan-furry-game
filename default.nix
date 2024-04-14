{ system ? builtins.currentSystem
, pkgs ? import <nixpkgs> { inherit system; config = {}; overlays = []; }
}:

let
  lib = pkgs.lib;

  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

  buildInputs = with pkgs; [
    cargo
    rustc
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];
  nativeBuildInputs = with pkgs; [ pkg-config ];

  # see https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md#nix
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;


  rustPackage = pkgs.rustPlatform.buildRustPackage rec {
    inherit buildInputs nativeBuildInputs;
    shellHooks = ''
      export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}
      '';
    passthru = {
      inherit shell;
    };
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  };

  shell = pkgs.mkShell rec {
    inherit buildInputs nativeBuildInputs LD_LIBRARY_PATH;
    # inputsFrom = [ rustPackage ];
    # ^ I know this exists but it doesn't work
  };

in
rustPackage
