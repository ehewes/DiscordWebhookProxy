{ flakePackages }: 
{ config, lib, pkgs, ... }: let
	inherit (lib) mkIf mkEnableOption;
	inherit (pkgs) system;
in {
	options.programs.discord-webhook-proxy = {
  		enable = lib.mkEnableOption "discord-webhook-proxy";

		port = lib.mkOption {
    		type = lib.types.str;
			default = "8042";
    		description = "";
  		};

		sledDbPath = lib.mkOption {
    		type = lib.types.str;
    		description = "";
  		};
	};

	config = mkIf config.programs.discord-webhook-proxy.enable {
		systemd.services.discord-webhook-proxy = {
			description = "Refract Backend";

      		serviceConfig = {
        		Type = "simple";
				Environment = [
    				"ROCKET_PORT=${config.programs.discord-webhook-proxy.port}"
    				"SLED_DB_PATH=${config.programs.discord-webhook-proxy.sledDbPath}"
  				];

				ExecStart = "${flakePackages.${system}.default}/bin/discord-webhook-proxy";
        		Restart = "on-failure";
      		};

			wantedBy = [ "multi-user.target" ];
		};
	};
}
