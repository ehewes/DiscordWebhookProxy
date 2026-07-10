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
    rev = "0b333ea7afd35915897db9ad5d982f4d19e22b4a";
    hash = "sha256-kIcCu1/UHu6Rl9f0GXK4yDkVI/HyWfVOrdf8V0EiPrk=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
