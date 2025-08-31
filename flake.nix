{
  description = "A very basic flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.pkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naerskLib = pkgs.callPackage naersk { };
        fenixLib = fenix.packages.${system};
        rustToolchain = fenixLib.stable.toolchain;
      in
      {
        packages = {
          default =
            (naersk.lib.${system}.override {
              cargo = rustToolchain;
              rustc = rustToolchain;
            }).buildPackage
              {
                name = "creb";
                src = ./.;
                # cargoLock.lockFile = ./Cargo.lock;
              };
        };
        devShell = pkgs.mkShell {
          name = "creb";

          buildInputs = with pkgs; [
            rustToolchain
            git
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
