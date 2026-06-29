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
    { flake-parts, ... }@inputs:
    let
      inherit (flake-parts.lib) importApply mkFlake;
    in
    mkFlake { inherit inputs; } (
      { withSystem, ... }:
      {
        systems = [ "x86_64-linux" ];

        perSystem =
          { pkgs, ... }:
          {
            packages.default = pkgs.callPackage ./package.nix { };
          };

        flake.nixosModules.default = importApply ./nixos-module.nix { inherit withSystem; };
      }
    );
}
