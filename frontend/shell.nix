with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "orderbook";
  
  buildInputs = with pkgs; [
    nodePackages.create-react-app
    nodejs
    yarn
  ];
}