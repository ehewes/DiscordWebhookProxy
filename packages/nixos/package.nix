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
    rev = "81fb6a16b9cb7b02f6b7d2930980b0dd6e97a488";
    hash = "sha256-fKgjDp6QftZlvj3Z8xx5tTOdNKMgk23OlJKos2GouVA=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
