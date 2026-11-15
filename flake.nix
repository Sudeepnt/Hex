{
  description = "Utter local-first voice dictation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      package = pkgs: self.packages.${pkgs.stdenv.hostPlatform.system}.utter;
    in
    {
      packages = forSystems (
        pkgs:
        let
          utter = pkgs.callPackage ./nix/package.nix { };
        in
        {
          inherit utter;
          default = utter;
        }
      );

      overlays.default = final: _prev: {
        utter = final.callPackage ./nix/package.nix { };
      };

      nixosModules.utter =
        { pkgs, ... }:
        {
          imports = [ ./nix/module.nix ];
          programs.utter.package = nixpkgs.lib.mkDefault (package pkgs);
        };
      nixosModules.default = self.nixosModules.utter;

      homeManagerModules.utter =
        { pkgs, ... }:
        {
          imports = [ ./nix/hm-module.nix ];
          programs.utter.package = nixpkgs.lib.mkDefault (package pkgs);
        };
      homeManagerModules.default = self.homeManagerModules.utter;

      devShells = forSystems (pkgs: {
        default = (package pkgs).devShell;
      });
      checks = forSystems (pkgs: import ./nix/checks.nix { inherit self pkgs; });
      formatter = forSystems (pkgs: pkgs.nixfmt);
    };
}
