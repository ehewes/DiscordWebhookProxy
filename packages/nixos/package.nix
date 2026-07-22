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
    rev = "dcf62ee95a3827a2f2acb1cfae7d69185cb073c7";
    hash = "sha256-f1U1Ng+AC5N7nNp04vos/BgloRBL639rpZbbtRi1vYo=";
  };

  cargoLock.lockFile = src + /Cargo.lock;

  meta.mainProgram = "discord-webhook-proxy";
}
