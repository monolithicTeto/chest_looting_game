{
    description = "chest_looting_game development environment";

    inputs = {

        # Inputs the unstable branch of nixpkgs.
        nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";

    };

    outputs = { nixpkgs, self }:

        let

            system = "x86_64-linux";
            pkgs = import nixpkgs {
                inherit system;
            };

        in

            {

            # Defines a development shell for this project.
            devShells.${system}.default = pkgs.mkShell {

                # Shell packages.
                packages = with pkgs; [
                    rustc
                    cargo
                    rustfmt
                    clippy
                ];

                # Shell startup
                # (this is optional, of course).
                shellHook = /* bash */ ''
                    clear && echo -e "\nchest_looting_game development environment\n"
                    rustc --version
                    cargo --version
                    echo ""
                '';

            };

        };

}
