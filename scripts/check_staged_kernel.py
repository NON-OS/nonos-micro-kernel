#!/usr/bin/env python3
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
"""The kernel on the ESP is the kernel that was just built, or the pack fails.

The ESP is packed from a chain: the kernel ELF is signed, the signed image
takes an attestation trailer, and the result is copied to EFI/nonos/kernel.bin.
Every step appends, so the ELF is a byte prefix of the staged image. When it is
not, the image came from an earlier link, and every boot of it is a statement
about a kernel that is no longer in the tree.

That is not a hypothetical. Two make goals on one command line let the pack
chain race the compile under -j, and a boot of a freshly checked out branch
then stages the previous branch's kernel. The result is worse than a failure,
because it passes: the log shows the old behaviour and the reader attributes it
to the new code.

The prefix is what gets checked, rather than the timestamps. A timestamp says
the staged file was written after the ELF; it does not say it was written from
it, and under -j those are not the same event.
"""

import argparse
import sys
from pathlib import Path

CHUNK = 1 << 20


def matched_prefix(elf, staged):
    """How many bytes of `elf` the head of `staged` reproduces."""
    seen = 0
    with open(elf, "rb") as want, open(staged, "rb") as got:
        while True:
            block = want.read(CHUNK)
            if not block:
                return seen
            packed = got.read(len(block))
            if packed != block:
                for i, (a, b) in enumerate(zip(block, packed)):
                    if a != b:
                        return seen + i
                return seen + len(packed)
            seen += len(block)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--elf", type=Path, required=True, help="the kernel ELF just linked")
    ap.add_argument("--staged", type=Path, required=True, help="EFI/nonos/kernel.bin on the ESP")
    a = ap.parse_args()

    for path in (a.elf, a.staged):
        if not path.is_file():
            print(f"staged-kernel: {path} does not exist")
            return 2

    size = a.elf.stat().st_size
    seen = matched_prefix(a.elf, a.staged)
    if seen != size:
        print(f"staged-kernel: {a.staged} was packed from a different link.")
        print(f"  it stops matching {a.elf} at byte {seen} of {size}.")
        print("  build the kernel and pack the ESP as separate make invocations;")
        print("  as two goals on one line they race under -j.")
        return 1

    trailer = a.staged.stat().st_size - size
    print(f"staged-kernel: ESP carries this kernel ({size} bytes, +{trailer} signed and attested)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
