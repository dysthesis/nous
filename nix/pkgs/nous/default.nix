{
  craneLib,
  cargoArtifacts,
  commonArgs,
  ...
}:
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    pname = "nous";
    CARGO_PROFILE = "release";
  }
)
