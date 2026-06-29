{ withSystem }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  inherit (lib)
    mkEnableOption
    mkOption
    types
    mapAttrs'
    nameValuePair
    filterAttrs
    ;

  cfg = config.services.discord-webhook-proxy;

  enabledInstances = filterAttrs (_: instance: instance.enable) cfg.instances;

  package = withSystem pkgs.stdenv.hostPlatform.system ({ config, ... }: config.packages.default);
in
{
  options.services.discord-webhook-proxy = {
    package = mkOption {
      type = types.package;
      default = package;

      description = "Package override";
    };

    instances = mkOption {
      default = { };

      type = types.attrsOf (
        types.submodule {
          options = {
            enable = mkEnableOption "discord-webhook-proxy instance";

            proxyServer = mkOption {
              default = { };
              description = "Webhook proxy server settings";

              type = types.submodule {
                options = {
                  address = mkOption {
                    type = types.str;
                    default = "127.0.0.1";
                    example = "127.0.0.1";

                    description = "Address for the server to bind to";
                  };

                  port = mkOption {
                    type = types.port;
                    default = 8042;

                    description = "Port for the server to listen on";
                  };
                };
              };
            };

            database = mkOption {
              default = { };
              description = "Database configuration";

              type = types.submodule {
                options = {
                  backend = mkOption {
                    type = types.enum [
                      "diesel"
                      "sled"
                    ];
                    default = "diesel";

                    description = "Database backend to use";
                  };

                  dataDirectory = mkOption {
                    type = types.path;
                    example = "/var/lib/discord-webhook-proxy";

                    description = "Directory used to store database data";
                  };
                };
              };
            };
          };
        }
      );
    };

    description = "discord-webhook-proxy instances";
  };

  config = {
    systemd.services = mapAttrs' (
      name: instance:
      nameValuePair "discord-webhook-proxy-${name}" {
        description = "Discord Webhook Proxy [${name}]";
        wantedBy = [ "multi-user.target" ];

        serviceConfig = {
          Type = "simple";
          Environment = [
            "ROCKET_PORT=${toString instance.proxyServer.port}"
            "ROCKET_ADDRESS=${instance.proxyServer.address}"
          ];
          ExecStart = "${lib.getExe cfg.package}";
          Restart = "on-failure";
        };
      }
    ) enabledInstances;
  };
}
