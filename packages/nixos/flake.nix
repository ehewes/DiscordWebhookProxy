{
  	inputs = {
    	nixpkgs.url = "github:nixos/nixpkgs/release-24.11";

		flake-parts = {
			url = "https://github.com/hercules-ci/flake-parts";
			type = "git";
			rev = "205b12d8b7cd4802fbcb8e8ef6a0f1408781a4f9";
		};
  	};

	outputs = { self, flake-parts, ... }@inputs: 
	let
  		inherit (flake-parts.lib) importApply mkFlake;
	in mkFlake { inherit inputs; } {
		systems = [ "x86_64-linux" ];

		perSystem = { pkgs, ... }: {
			packages.default = pkgs.callPackage ./package.nix { inherit pkgs; };
		};

		flake.nixosModules.default = importApply ./nixos-module.nix { flakePackages = self.packages; };
	};
}
