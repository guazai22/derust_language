let

  ################################################################################
  depended_pkgs = with pkgs;
    [
      # pkg-config
      # openssl
      # gcc
    ];
  ################################################################################

  pkgs = import <nixpkgs-unstable> { };
  rust_pkgs = with pkgs; [ rustc cargo rustfmt clippy rust-analyzer ];
in pkgs.mkShell {
  buildInputs = rust_pkgs ++ depended_pkgs;
  RUST_BACKTRACE = 1;
  shellHook = ''
    echo hello, nix-shell_of_rust!
  '';
  CARGO_TARGET_DIR = "/mnt/A/Rust_target/";
}
