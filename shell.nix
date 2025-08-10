{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
    name = "sptf";

    packages = 
         with pkgs; [
            pkg-config
            openssl
            cargo
            rust-analyzer
          ];
}
         
