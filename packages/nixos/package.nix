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
    rev = "c9b0bc17d11d61bf64565d86bce168d695cb2bde";
    hash = "sha256-h24bGJWsbCkt3k4ReV1FndXX9SRWQ2blZ29FLanjeX4=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
