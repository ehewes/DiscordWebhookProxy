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
    rev = "8b0696132e61eb4068cf9f1029031926222d764c";
    hash = "sha256-t/QOaxxhSAAhFBd2kKG9Se1RIdsq+lt1XU1Rwy5WAlM=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
