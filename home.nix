{
  pkgs,
  ...
}:
{
  imports = [
    ./modules/home-manager
  ];
  bigbother.theme = {
    enable = true;
    cursor = false;
    font = true;
  };
  programs = {
    home-manager.enable = true;
    vim.enable = true;

    plasma = {
      enable = true;
      #   configFile."kwinrc"."ElectricBorders"."BottomLeft" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."Left" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."BottomRight" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."Right" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."Top" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."TopLeft" = "LockScreen";
      #   configFile."kwinrc"."ElectricBorders"."TopRight" = "LockScreen";
      #   configFile."kwinrc"."Windows"."ElectricBorderDelay" = 0;
      #   configFile."ksplashrc"."KSplash"."Engine" = "none";
    };
  };
  home = {
    packages = [
      pkgs.fastfetch
    ];
    username = "test";
    stateVersion = "23.05";
  };
}
