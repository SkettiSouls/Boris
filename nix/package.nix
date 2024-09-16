{ rustPlatform, version ? "git", lib }:

rustPlatform.buildRustPackage rec {
  pname = "boris";
  inherit version;

  src = lib.cleanSource ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
