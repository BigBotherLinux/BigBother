{ config, pkgs, lib, inputs, ... }:
let
  cfg = config.bigbother.sudo;
in
{
  
  options.bigbother.sudo = {
    enable = lib.mkEnableOption "Big Bother sudo config";
    description = "Enable Big Bother sudo config";
  };
  
  config = lib.mkIf cfg.enable {
  environment.etc = { 
    # Lecture file for sudo
    "/bb-sudoers.lecture".text = ''
      You are trying to run a command with root privileges, hopefully you know what you're about to do.
    '';
  };

  # change badpass message and add lecture on sudo
  security.sudo.extraConfig = ''
    Defaults  badpass_message = "Wrong password, maybe try to type it correctly?"
    Defaults  lecture = always
    Defaults  lecture_file = /etc/bb-sudoers.lecture
  '';
  };
}