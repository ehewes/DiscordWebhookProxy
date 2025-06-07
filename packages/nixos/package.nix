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
    	rev = "";
    	hash = "";
  	};

	cargoLock.lockFile = src + /Cargo.lock;

	inherit buildInputs;
}
