{
  description = "Akasha Discord Bot";

  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = pkgs.rustPlatform;
      in rec {
        packages.default = toolchain.buildRustPackage {
          pname = "get_traffic";
          version = "0.0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
        apps.default = utils.lib.mkApp { drv = packages.default; };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (with toolchain; [ cargo rustc rustLibSrc ])
            clippy
            rustfmt
            pkg-config
            openssl
          ];
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
        };
      });
}
