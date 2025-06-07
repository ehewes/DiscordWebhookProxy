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
    	rev = "218b09134bb0c3792cc0b0f2b4607cba55c31332";
    	hash = "sha256-wN3SK2aQX//TIStzhziytnOwmiJ4CJ+OVkhkIqz5a4E=";
  	};

	cargoLock.lockFile = src + /Cargo.lock;

	inherit buildInputs;
}
