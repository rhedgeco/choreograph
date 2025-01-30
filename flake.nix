{
  description = "rust-shells";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [(import inputs.rust-overlay)];
        };

        # create a function that builds data for a rust shell
        mkRust = {
          version,
          profile,
        }: let
          rust-bin = pkgs.rust-bin.${version}.latest.${profile}.override {
            extensions = ["rust-src"];
          };
        in {
          path = "${rust-bin}/lib/rustlib/src/rust/library";

          deps =
            [rust-bin]
            ++ (with pkgs; [
              just
              openssh
              rust-analyzer
            ]);
        };
      in {
        formatter = pkgs.alejandra;

        devShells = {
          default = let
            rust = mkRust {
              version = "stable";
              profile = "default";
            };
          in
            pkgs.mkShell {
              name = "rust-stable";
              RUST_SRC_PATH = rust.path;
              buildInputs = rust.deps;
            };

          nightly = let
            rust = mkRust {
              version = "nightly";
              profile = "default";
            };
          in
            pkgs.mkShell {
              name = "rust-nightly";
              RUST_SRC_PATH = rust.path;
              buildInputs = rust.deps;
            };
        };
      };
    };
}
