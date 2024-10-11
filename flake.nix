{
  description = "Light weight, blazing fast blockchain indexer";

  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, foundry, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default foundry.overlay ];
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        cargoTomlContents = builtins.readFile ./Cargo.toml;
        version = (builtins.fromTOML cargoTomlContents).package.version;

        ethereumEs = pkgs.rustPlatform.buildRustPackage {
          inherit version;
          name = "ethereumEs";
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];

          src = pkgs.lib.cleanSourceWith { src = self; };

          cargoLock.lockFile = ./Cargo.lock;

          GIT_COMMIT_HASH_SHORT = self.shortRev or "unknown";

        };

        packages = {
          ethereumEs = ethereumEs;
          default = packages.ethereumEs;
        };

       overlays.default = final: prev: { ethereumEs = packages.ethereumEs; };

        gitRev = if (builtins.hasAttr "rev" self) then self.rev else "dirty";
      in {
        inherit packages overlays;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            foundry-bin
            solc
            toolchain
            openssl
            pkg-config
            eza
            rust-bin.beta.latest.default
            rust-analyzer-unwrapped
            watchexec
            rustup
            postgresql
          ];
          shellHook = ''
            alias ls='eza --icons'
            alias find=fd
            export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
            ##
            ## Don't pollute local cache of cargo registry index
            ## If you dont care about that, remvoe the line below
            ## to amortize on your existing local cache
            export CARGO_HOME="$(pwd)/.cargo"
            ##
            export PATH="$CARGO_HOME/bin:$PATH"
            export RUST_BACKTRACE=1
            ##
            ## POSTGRES related environment variables
            ##
            export PGHOST=localhost
            export PGPORT=5432
            export PGUSER='postgres'
            export PGPASSWORD='postgres'
            export PGDATABASE='lib9'
            export DB_URL='postgres://postgres:postgres@127.0.0.1:5432/library'
            ## create postgres DB
            createdb  $PGDATABASE
            #
          '';
        };
      });

  ## use iog cache
  nixConfig = {
    extra-substituters = [
      "https://cache.iog.io"
      "https://cache.sc.iog.io"
    ];
    extra-trusted-public-keys = [
      "hydra.iohk.io:f/Ea+s+dFdN+3Y/G+FDgSq+a5NEWhJGzdjvKNGv0/EQ="
      "cache.sc.iog.io:b4YIcBabCEVKrLQgGW8Fylz4W8IvvfzRc+hy0idqrWU="
    ];
    allow-import-from-derivation = true;
    accept-flake-config = true;
  };

}
