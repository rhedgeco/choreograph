{
  pkgs,
  inputs,
  ...
}: [
  (pkgs.callPackage ./code.nix {inherit inputs;})
]
