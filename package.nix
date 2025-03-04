{ lib
, fetchFromGitHub
, rustPlatform
,
}:
rustPlatform.buildRustPackage rec {
  pname = "axum-controller";
  version = "0.0.1";

  src = ./.;

  useFetchCargoVendor = true;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    # description = "";
    # homepage = "";
    # license = lib.licenses.unlicense;
    maintainers = [ ];
  };
}
