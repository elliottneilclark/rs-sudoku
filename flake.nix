{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };


    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, treefmt-nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;

          buildInputs = [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        toolchain = "latest";
        rustPkg = fenix.packages.${system}.${toolchain}.withComponents
          [
            "cargo"
            "clippy"
            "rust-src"
            "llvm-tools"
            "rustc"
            "rustfmt"
          ];

        craneLibLLvmTools = craneLib.overrideToolchain rustPkg;

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        rs-sudoku = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        formatter = treefmt-nix.lib.mkWrapper pkgs
          {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.prettier.enable = true;

            settings.formatter.prettier.excludes = [
              ".nix-cargo/*"
            ];
          };
      in
      {
        inherit formatter;
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit rs-sudoku;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          rs-sudoku-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          rs-sudoku-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          rs-sudoku-fmt = craneLib.cargoFmt {
            inherit src;
            cargoClippyExtraArgs = "--all --check";
          };

          # Audit dependencies
          rs-sudoku-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `rs-sudoku` if you do not want
          # the tests to run twice
          rs-sudoku-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          rs-sudoku-coverage = craneLib.cargoTarpaulin (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        packages = {
          default = rs-sudoku;
          rs-sudoku-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = rs-sudoku;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          nativeBuildInputs = with pkgs; [
            rustPkg
            rust-analyzer-nightly
            pkg-config
            git
            cmake
            openssl
          ];

          shellHook = ''
            BASE=$(git rev-parse --show-toplevel || echo ".")

            # This keeps cargo self contained in this dir
            export CARGO_HOME=$BASE/.nix-cargo
            mkdir -p $CARGO_HOME
            export PATH=$CARGO_HOME/bin:$PATH
          '';
        };
      });
}
