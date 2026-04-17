{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];

  perSystem.treefmt.programs = {
    deadnix.enable = true;
    nixfmt.enable = true;
    statix.enable = true;
    nixf-diagnose.enable = true;

    shfmt.enable = true;
    shellcheck.enable = true;

    rustfmt.enable = true;

    oxfmt.enable = true;
  };
}
