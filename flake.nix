{
  description = "Build barretenberg-sys";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-22.11";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      # All of these inputs (a.k.a. dependencies) need to align with inputs we
      # use so they use the `inputs.*.follows` syntax to reference our inputs
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      # All of these inputs (a.k.a. dependencies) need to align with inputs we
      # use so they use the `inputs.*.follows` syntax to reference our inputs
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        flake-compat.follows = "flake-compat";
        rust-overlay.follows = "rust-overlay";
      };
    };

    barretenberg = {
      url = "github:AztecProtocol/barretenberg";
      # All of these inputs (a.k.a. dependencies) need to align with inputs we
      # use so they use the `inputs.*.follows` syntax to reference our inputs
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs =
    { self, nixpkgs, crane, flake-utils, rust-overlay, barretenberg, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
          barretenberg.overlays.default
        ];
      };

      rustToolchain = pkgs.rust-bin.stable."1.66.0".default.override {
        # We include rust-src to ensure rust-analyzer works.
        # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/4
        extensions = [ "rust-src" ];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      environment = {
        # rust-bindgen needs to know the location of libclang
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

        # Barretenberg fails if tests are run on multiple threads, so we set the test thread
        # count to 1 throughout the entire project
        #
        # Note: Setting this allows for consistent behavior across build and shells, but is mostly
        # hidden from the developer - i.e. when they see the command being run via `nix flake check`
        RUST_TEST_THREADS = "1";
      };

      # Combine the environment and other configuration needed for crane to build our Rust packages
      commonArgs = environment // {
        # As per https://discourse.nixos.org/t/gcc11stdenv-and-clang/17734/7 since it seems that aarch64-linux uses
        # gcc9 instead of gcc11 for the C++ stdlib, while all other targets we support provide the correct libstdc++
        stdenv = with pkgs;
          if (stdenv.targetPlatform.isGnu && stdenv.targetPlatform.isAarch64) then
            overrideCC llvmPackages.stdenv (llvmPackages.clang.override { gccForLibs = gcc11.cc; })
          else
            llvmPackages.stdenv;

        src = craneLib.cleanCargoSource ./.;


        # Running checks don't do much more than compiling itself and increase
        # the build time by a lot, so we disable them throughout all our flakes
        doCheck = false;

        nativeBuildInputs = [
          # This provides the pkg-config tool to find barretenberg & other native libraries
          pkgs.pkg-config
          # This provides the `lld` linker to cargo
          pkgs.llvmPackages.bintools
        ];

        buildInputs = [
          pkgs.llvmPackages.openmp
          pkgs.barretenberg
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          # Need libiconv on Darwin. See https://github.com/ipetkov/crane/issues/156
          pkgs.libiconv
        ];
      };

      # Build *just* the cargo dependencies, so we can reuse all of that work between runs
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      barretenberg-sys = craneLib.buildPackage (commonArgs // {
        pname = "barretenberg-sys";
        # x-release-please-start-version
        version = "0.1.1";
        # x-release-please-end

        inherit cargoArtifacts;
      });
    in
    rec {
      checks = {
        cargo-clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;

          # TODO(blaine): It'd be nice to include these flags when running `cargo clippy` in a devShell.
          cargoClippyExtraArgs = "--all-targets -- -D warnings";

          doCheck = true;
        });

        cargo-test = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;

          # TODO(blaine): It'd be nice to include this flag when running `cargo test` in a devShell.
          cargoTestExtraArgs = "--workspace";

          doCheck = true;
        });
      };

      packages.default = barretenberg-sys;

      # Combine the environment settings with the inputs from our checks
      # derivations and extra tooling via `nativeBuildInputs`
      devShells.default = pkgs.mkShell (environment // {
        inputsFrom = builtins.attrValues checks;

        LIBBARRETENBERG = pkgs.barretenberg;

        nativeBuildInputs = with pkgs; [
          which
          starship
          git
          nil
          nixpkgs-fmt
          llvmPackages.lldb # This ensures the right lldb is in the environment for running rust-lldb
        ];

        shellHook = ''
          eval "$(starship init bash)"
          echo LIBBARRETENBERG=$LIBBARRETENBERG
        '';
      });
    });
}
