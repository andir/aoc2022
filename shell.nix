with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [ cargo rust-analyzer rustfmt rustc ];
}
