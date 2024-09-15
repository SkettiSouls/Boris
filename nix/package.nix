{ rustPlatform, fetchFromGitHub, version ? "git" }:

rustPlatform.buildRustPackage rec {
  pname = "boris";
  inherit version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
