{
  fetchFromGitHub,
  rustPlatform,
}:

rustPlatform.buildRustPackage rec {
  pname = "discord-webhook-proxy";
  version = "0.0.0";

  src = fetchFromGitHub {
    owner = "ehewes";
    repo = "DiscordWebhookProxy";
    rev = "1bbb412cbbd1aedef25f5a32fe61a6f7d6870af5";
    hash = "sha256-gG6OkHj0Nwk2tNtsQ3aGp5CFUA24VFSxgLpZL77KNPE=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
