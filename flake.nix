{
  description = "Native Gtk client for Lemmy";

  inputs = {
      flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs { inherit system; };
  in {
    packages = rec {
        lemoa = pkgs.rustPlatform.buildRustPackage {
            pname = "lemoa";
            version = "0.3.0";
            src = ./.;
            buildInputs = with pkgs; [
                openssl
                gtk4
                libadwaita
            ];
            nativeBuildInputs = with pkgs; [
                openssl
                pkg-config
                gtk4
                libadwaita
            ];
            cargoLock = {
                lockFile = ./Cargo.lock;
            };
            meta = with pkgs.lib; {
                description = "Native Gtk client for Lemmy";
                homepage = "https://github.com/lemmygtk/lemoa";
                license = with licenses; [
                    gpl3
                ];
            };
        };
        default = lemoa;
    };

    devShells.default = pkgs.mkShell {
        packages = with pkgs; [
                openssl
                pkg-config
                gtk4
                libadwaita

                #rust
                cargo
                rustc
                rust-analyzer
                rustfmt
        ];
    };
  });
}
