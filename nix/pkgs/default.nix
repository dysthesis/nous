{
  perSystem = {
    craneLib,
    pkgs,
    commonArgs,
    cargoArtifacts,
    ...
  }: let
    inherit (pkgs) callPackage;
  in {
    packages = rec {
      nous = callPackage ./nous {
        inherit
          craneLib
          pkgs
          commonArgs
          cargoArtifacts
          ;
      };
      default = nous;
    };
  };
}
