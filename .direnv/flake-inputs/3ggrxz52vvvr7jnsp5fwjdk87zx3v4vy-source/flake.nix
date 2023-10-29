{
  description = "Advent of Code monorepo";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    makeDevShell = system: let
      pkgs = import nixpkgs { inherit system; };
      isDarwin = pkgs.stdenv.hostPlatform.isDarwin;
      darwinPackages = if isDarwin then [ ] else [ ];

    in
    pkgs.mkShell {
      buildInputs = [
        pkgs.rustup
        pkgs.git
      ] ++ darwinPackages;

      shellHook = '''';
    };

  in {
    devShell = {
      x86_64-linux = makeDevShell "x86_64-linux";
      x86_64-darwin = makeDevShell "x86_64-darwin";
      aarch64-darwin = makeDevShell "aarch64-darwin";
    };
  };
}
