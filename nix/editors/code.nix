{
  pkgs,
  inputs,
  ...
}: (pkgs.vscode-with-extensions.override {
  vscodeExtensions = with pkgs.vscode-extensions;
    [
      jnoortheen.nix-ide
      kamadorueda.alejandra
      rust-lang.rust-analyzer
      tamasfe.even-better-toml
      vadimcn.vscode-lldb
    ]
    ++ (with inputs.vscode-extensions.extensions.${pkgs.system}.vscode-marketplace; [
      kokakiwi.vscode-just
      citreae535.sparse-crates
    ]);
})
