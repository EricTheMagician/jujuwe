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
            rustc
            cargo
            pkg-config
            glib
            pango
            atk
            gdk-pixbuf
            gtk3
            libsoup_3
            webkitgtk_4_1
          ];

          shellHook = ''
            echo "jj version: $(jj --version)"
            echo "dolt version: $(dolt version)"

            echo "Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:"
            echo "For Desktop development, run:"
            echo "  bun run tauri dev"
          '';
        };
      }
    );
}
