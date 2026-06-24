{
  description = "crdt-doc - typed CRDT-text border for pleme-io";
  inputs = {
    nixpkgs.follows = "substrate/nixpkgs";
    fenix = { url = "github:nix-community/fenix"; inputs.nixpkgs.follows = "nixpkgs"; };
    substrate = { url = "github:pleme-io/substrate"; inputs.fenix.follows = "fenix"; };
    crate2nix = { url = "github:nix-community/crate2nix"; inputs.nixpkgs.follows = "nixpkgs"; };
  };
  outputs = { self, nixpkgs, substrate, crate2nix, ... }:
    (import "${substrate}/lib/rust-library.nix" {
      inherit nixpkgs substrate crate2nix;
    }) {
      inherit self;
      crateName = "crdt-doc";
    };
}
