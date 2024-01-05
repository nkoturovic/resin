{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    koturNixPkgs = {
      url = "github:nkoturovic/kotur-nixpkgs/v0.5";
      flake = false;
    };
    rustOverlay.url = "github:oxalica/rust-overlay";
  };
  outputs = {
    self,
    nixpkgs,
    koturNixPkgs,
    rustOverlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rustOverlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      # pkgs = nixpkgs.legacyPackages.${system};
      package = import ./default.nix {inherit system pkgs;};
    in {
      packages.default = package;
      devShells.default = package.shell;
      formatter = pkgs.alejandra;
    });
}
