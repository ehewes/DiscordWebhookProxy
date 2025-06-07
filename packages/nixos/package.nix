{ lib, fetchFromGitHub, rustPlatform, pkgs }:

let
	buildInputs = with pkgs; [ ];

	libraryPaths = lib.makeLibraryPath buildInputs;
in rustPlatform.buildRustPackage rec {
	pname = "discord-webhook-proxy";
  	version = "0.0.0";

  	src = fetchFromGitHub {
    	owner = "ehewes";
    	repo = "DiscordWebhookProxy";
    	rev = "f347fc69877378512e86003b0794c86f11bc3353";
    	hash = "sha256-7n7333kzYIx2wBpFta3jP51z6cpEVTpADVYCQSI6iMc=";
  	};

	cargoLock.lockFile = src + /Cargo.lock;

	inherit buildInputs;
}
