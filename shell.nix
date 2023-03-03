let
  nixpkgs = import <nixpkgs> {};
in

  with nixpkgs;

  mkShell {
    buildInputs = [
      alsaLib
      cmake
      freetype
      rustup
      cargo
      expat
      openssl
      pkgconfig
      fontconfig
      vulkan-validation-layers
      xorg.libX11
      rust-analyzer

    ];
    
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      wayland
      egl-wayland
      libxkbcommon
      libGL
    ];
  }