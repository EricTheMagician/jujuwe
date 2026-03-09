{
  description = "Dev shell with jujutsu and dolt";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            jujutsu
            dolt
            bun
          ];

          shellHook = ''
            echo "jj version: $(jj --version)"
            echo "dolt version: $(dolt version)"
          '';
        };
      }
    );
}
