{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.gnome-smbios = naersk-lib.buildPackage {
        pname = "gnome-smbios";
        root = ./.;
        nativeBuildInputs = [ pkgs.gtk4 pkgs.pkg-config pkgs.libadwaita ];
      };
      packages.default = packages.gnome-smbios;

      apps.gnome-smbios = utils.lib.mkApp {
        drv = packages.gnome-smbios;
      };
      apps.default = apps.gnome-smbios;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo gtk4 pkg-config libadwaita ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
