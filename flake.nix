{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
  inputs.koturNixPkgs = {
    url = "github:nkoturovic/kotur-nixpkgs/v0.5";
    flake = false;
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      package = import ./default.nix {inherit system pkgs;};
    in {
      packages.default = package;
      devShells.default = package.shell;
      formatter = pkgs.alejandra;
    });
}
