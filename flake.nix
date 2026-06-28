{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    flake-parts = {
      url = "https://github.com/hercules-ci/flake-parts";
      type = "git";
      rev = "57928607ea566b5db3ad13af0e57e921e6b12381";
    };

    rust-overlay = {
      url = "https://github.com/oxalica/rust-overlay";
      type = "git";
      rev = "11a396520bf911e4ed01e78e11633d3fc63b350e";
    };

    discord-webhook-proxy.url = "./packages/nixos";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem =
        { lib, system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            config = {
              allowUnfreePredicate = pkg: builtins.elem (lib.getName pkg) [ ];
            };
            overlays = [ (import inputs.rust-overlay) ];
          };

          rustVersion = "1.96.0";

          rustToolchain = pkgs.rust-bin.stable.${rustVersion}.complete.override {
            targets = [ ];
          };

          buildInputs = with pkgs; [
            git
            openssl
          ];

          mkDevShellRust =
            toolchain:
            pkgs.mkShell {
              inherit buildInputs;

              nativeBuildInputs = with pkgs; [
                pkg-config
                toolchain
                rust-analyzer
                sccache
              ];

              RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

              shellHook = ''

              '';
            };
        in
        {
          devShells.default = mkDevShellRust rustToolchain;
        };
    };
}
