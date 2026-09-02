{
  # The NONOS build environment, pinned. `nix develop` drops any Linux or
  # macOS machine into a shell with every tool the build and boot need, at
  # versions locked by flake.lock, so "works on my machine" stops being a
  # sentence anyone has to say. CI and a stranger's laptop get the same
  # environment from the same lock.
  #
  # Scope, stated honestly: this pins the environment, not yet the build
  # graph. The kernel and capsules still build through make + cargo with the
  # toolchain pinned by rust-toolchain.toml, which rustup resolves inside
  # this shell. Turning each capsule into a content-addressed derivation so
  # the whole image is a pure function of its inputs is the next step, and
  # it is the one that makes the attestation chain source-to-silicon: pure
  # build -> reproducible artifact -> STARK-verified execution.
  description = "NONOS: capability microkernel, RAM-resident, STARK-attested";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          name = "nonos";
          packages = with pkgs; [
            # the Rust entry point; rust-toolchain.toml pins the actual nightly
            rustup
            # boot and measure
            qemu
            swtpm
            # image assembly, reproducible
            xorriso
            mtools
            # build tooling
            gnumake
            python3
            git
            sccache
          ];
          shellHook = ''
            echo "NONOS dev shell. make doctor to confirm, make to build."
          '';
        };
      });
}
