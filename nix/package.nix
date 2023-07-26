{ rustPlatform, openssl, pkg-config, lib, config, ... }:
  rustPlatform.buildRustPackage {
    pname = "railboard-api";
    version = (builtins.fromTOML (builtins.readFile ../railboard-api/Cargo.toml)).package.version;
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
    meta = with lib; {
      description = config.description;
      homepage = "https://github.com/StckOverflw/railboard-api";
      license = licenses.gpl3;
    };
    
    doCheck = false;

    buildInputs = [
      openssl
    ];
    nativeBuildInputs = [
      pkg-config
    ];
  }