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
    rev = "af3f125c741513e74dc9d040ea937f98e4556549";
    hash = "sha256-zT/fkH9VsRISyTFEchyMUUKf+3rHohIuOYfVJGlGLBs=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
