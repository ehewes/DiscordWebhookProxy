{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";

    flake-parts = {
      url = "https://github.com/hercules-ci/flake-parts";
      type = "git";
      rev = "57928607ea566b5db3ad13af0e57e921e6b12381";
    };
  };

  outputs =
    { self, flake-parts, ... }@inputs:
    let
      inherit (flake-parts.lib) importApply mkFlake;
    in
    mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem =
        { pkgs, ... }:
        {
          packages.default = pkgs.callPackage ./package.nix { inherit pkgs; };
        };

      flake.nixosModules.default = importApply ./nixos-module.nix { flakePackages = self.packages; };
    };
}
