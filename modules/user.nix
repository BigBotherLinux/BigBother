{ lib, ... }:
{
  options.bigbother.primaryUser = lib.mkOption {
    type = lib.types.str;
    default = "nixos";
    description = "Primary user account name";
  };
}
