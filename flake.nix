{

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            rust-analyzer
            helix
            lldb
            bacon
            vscode-langservers-extracted
            # Assume rust-rover is directly available, replace with the actual package name or method to include it
          ];
          shellHook = ''
            rustup component add rust-analyzer
            rustup component add cargo
          '';

          # Setup environment variables or additional shell hooks if needed
        };
      });
}

