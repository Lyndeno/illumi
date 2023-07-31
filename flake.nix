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
      packages.illumi = naersk-lib.buildPackage {
        pname = "illumi";
        root = ./.;
        nativeBuildInputs = [ pkgs.gtk4 pkgs.pkg-config pkgs.libadwaita ];
      };
      packages.default = packages.illumi;

      apps.illumi = utils.lib.mkApp {
        drv = packages.illumi;
      };
      apps.default = apps.illumi;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo gtk4 pkg-config libadwaita ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
