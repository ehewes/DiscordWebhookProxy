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
    rev = "19e1c11eeb2b8f59e1b424983066dc9368563b46";
    hash = "sha256-n692LlptrPk/k2qhp9Kjm+nTPjuRQJMcynenXdAL7XY=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
