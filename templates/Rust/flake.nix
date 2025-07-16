{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          runtimeDeps = with pkgs; [ 
            cargo-vet
            cargo-expand
          ];
          buildDeps = with pkgs; [];
          devDeps = with pkgs; [ gdb ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.default;
              rustc = pkgs.rust-bin.stable.latest.default;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = pkgs.lib.cleanSourceWith {
                src = ./.;
                filter = path: type:
                  let
                    rel = pkgs.lib.removePrefix (toString ./. + "/") (toString path);
                  in
                    (builtins.match "src(/.*)?" rel) != null ||
                    rel == "Cargo.toml" ||
                    rel == "Cargo.lock";
              };
              
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              # Uncomment if your cargo tests require networking or otherwise
              # don't play nicely with the Nix build sandbox:
              # doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              shellHook = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              '';
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };

        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          packages.default = self'.packages.rust_project;
          devShells.default = self'.devShells.stable;

          packages.rust_project = (rustPackage "");

          # devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
          #   (toolchain: toolchain.default)));
          devShells.stable = (mkDevShell pkgs.rust-bin.stable.latest.default);
          # devShells.msrv = (mkDevShell pkgs.rust-bin.stable.${msrv}.default);
        };
    };
}