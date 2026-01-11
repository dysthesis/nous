{
  perSystem = {
    lib,
    pkgs,
    self',
    advisory-db,
    craneLib,
    commonArgs,
    cargoArtifacts,
    src,
    ...
  }: {
    checks = let
      inherit (lib) foldr;
      defaultCheckArgs = {
        inherit
          craneLib
          commonArgs
          cargoArtifacts
          src
          pkgs
          advisory-db
          ;
        inherit self';
      };

      mkCheck = name: {
        "package-${name}" =
          import (./. + "/${name}.nix") defaultCheckArgs;
      };

      checkNames = [
        "clippy"
        "doc"
        "fmt"
        "audit"
        "nextest"
        "taplo"
        "deny"
        "hakari"
      ];
    in
      foldr
      (curr: acc: acc // mkCheck curr)
      {package = self'.packages.nous-cli;}
      checkNames;
  };
}
