{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.utter;
in
{
  options.programs.utter = {
    enable = lib.mkEnableOption "Utter local voice dictation";
    package = lib.mkOption {
      type = lib.types.package;
      description = "Utter package to install.";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = lib.elem pkgs.stdenv.hostPlatform.system [
          "x86_64-linux"
          "aarch64-linux"
        ];
        message = "The Utter Nix package supports x86_64-linux and aarch64-linux only.";
      }
    ];
    environment.systemPackages = [ cfg.package ];
  };
}
