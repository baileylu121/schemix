{
  lib,
  self,
  ...
}:
{
  perSystem =
    { pkgs, ... }:
    let
      cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
      schemix = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
        pname = cargoToml.package.name;
        inherit (cargoToml.package) version;

        src = self;

        cargoLock = {
          lockFile = "${finalAttrs.src}/Cargo.lock";
        };

        meta = {
          description = "Simple DSL language for writing nix module options";
          homepage = "https://github.com/baileylu121/schemix";
          license = lib.licenses.mit;
          maintainers = [ lib.maintainers.baileylu ];
          mainProgram = "schemix";
        };
      });
    in
    {
      packages = {
        inherit schemix;
        default = schemix;
      };

      checks = {
        inherit schemix;
      };
    };

}
