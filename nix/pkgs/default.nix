{
  perSystem = {
    craneLib,
    commonArgs,
    cargoArtifacts,
    ...
  }: let
    buildPkg = name: cargoExtraArgs:
      craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
          pname = name;
          cargoExtraArgs = cargoExtraArgs;
          CARGO_PROFILE = "release";
        }
      );
  in let
    nous-cli = buildPkg "nous-cli" "-p nous-cli";
  in {
    packages = {
      inherit nous-cli;
      default = nous-cli;
    };

    apps = {
      nous-cli = {
        type = "app";
        program = "${nous-cli}/bin/nous";
      };
    };
  };
}
