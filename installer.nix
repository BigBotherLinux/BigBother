# everything in this file is modified during installation
{...}:
{
  users.users.test = {
    group = "nixos";
    initialPassword = "nixos";
    isNormalUser = true;
  };
  home-manager.users.test = import ./home.nix;
}
