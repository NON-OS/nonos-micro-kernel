# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""The seven build steps. Each announces itself, does one thing through
the ordinary make surface, and proves its own outcome."""

from . import checks
from .shell import out, run_make

EFI = "nonos-bootloader/target/x86_64-unknown-uefi/release/nonos_boot.efi"
ATTESTED = "target/kernel_attested.bin"


def doctor(env):
    out("  Toolchain, targets, and host dependencies, checked not assumed.")
    run_make("nonos-mk-check-deps", "doctor", env)
    out("  toolchain ready")


def identity(env):
    out("  A throwaway development identity is minted from local randomness.")
    out("  Seeds stay in gitignored paths on this machine and are never")
    out("  shipped; a release is signed by its own held keys, not these.")
    run_make("nonos-mk-ensure-signing-key", "identity", env)
    seed = env.get("SIGNING_KEY", "nonos-bootloader/keys/signing_key_v1.bin")
    checks.require(seed, "Ed25519 dev signing seed", 32)


def kernel(env):
    out("  The kernel and every included capsule, built and signed.")
    run_make("nonos-mk-zerostate", "kernel", env)
    checks.show("target/kernel_signed.bin", "signed kernel")


def attest(env):
    out("  The kernel proves its own measurement; the STARK trailer is")
    out("  minted fresh here, which is why two builds never share one.")
    run_make("nonos-mk-attest", "attest", env)
    checks.show(ATTESTED, "attested kernel")


def verify(env):
    out("  The baked trust ledger is re-checked over the tree that built.")
    run_make("nonos-mk-check-trust-manifest", "verify", env)
    out("  ledger verified")


def image(env):
    out("  Loader and attested kernel packed into a bootable ESP.")
    run_make("nonos-mk-esp", "image", env)
    checks.show(EFI, "bootloader")
    checks.show("target/esp/EFI/nonos/kernel.bin", "ESP kernel image")


def receipt(env):
    out("  What this machine just built, in numbers worth writing down:")
    for path, what in [
        ("target/x86_64-nonos/release/nonos-kernel", "kernel ELF"),
        (ATTESTED, "attested image"),
        (EFI, "loader"),
    ]:
        checks.show(path, what)
    out("  The kernel ELF hash is reproducible: an independent clean build")
    out("  of this commit lands on the same value.")
