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
"""Keep the kernel and userland capability tables from drifting apart.

The kernel decides what a capability bit means; userland only needs to name the
same bits so a capsule can declare them. Nothing checked that, and the two had
already diverged by six capabilities: the kernel enforced `TimeSet` through
`EnrolDevRoot` while userland stopped at `InputSource`, so no capsule could
declare them at all.

A mismatched value is worse than a missing one. A capsule would declare what it
believed was one permission and be granted another, and every layer downstream,
manifest, attestation and the kernel's own install, would agree with each other
and be wrong together.
"""

import argparse
import re
import sys
from pathlib import Path

KERNEL_BITS = Path("src/capabilities/types/bit.rs")
USER_BITS = Path("userland/nonos_cap/src/bits.rs")
MANIFEST_BITS = Path("userland/platform/nonos_manifest/src/caps/bit.rs")


def canonical(name: str) -> str:
    """Compare names ignoring word separation.

    The two sides do not agree on a convention: the kernel's `FileSystem` is
    `CAP_FILESYSTEM` here and `CAP_CORE_EXEC` there, both from CamelCase. Since
    underscore placement carries no meaning, it is removed before comparing, so
    the check reports value mismatches rather than styling.
    """
    return name.replace("_", "").upper()


def read_kernel(root: Path):
    text = (root / KERNEL_BITS).read_text()
    pairs = re.findall(r"Self::(\w+)\s*=>\s*(\d+)\s*,", text)
    return {canonical(n): int(v) for n, v in pairs}


def read_userland(root: Path):
    text = (root / USER_BITS).read_text()
    pairs = re.findall(r"pub const CAP_(\w+)\s*:\s*u64\s*=\s*(\d+)\s*;", text)
    return {canonical(n): int(v) for n, v in pairs}


def read_manifest(root: Path):
    """The name a manifest may use, resolved by the signing tool.

    A name missing here is not a silent hole: `cap_mask` rejects it. It does
    mean the capability cannot be declared by name at all, which is why the
    capsules needing one of the eight that were missing carried a raw hex mask
    and a hand-written comment instead.
    """
    text = (root / MANIFEST_BITS).read_text()
    pairs = re.findall(r'"(\w+)"\s*=>\s*(\d+)\s*,', text)
    return {canonical(n): int(v) for n, v in pairs}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    args = ap.parse_args()

    try:
        kernel = read_kernel(args.root)
        mirrors = {"userland": read_userland(args.root),
                   "manifest": read_manifest(args.root)}
    except FileNotFoundError as e:
        print(f"cap-parity: {e}", file=sys.stderr)
        return 2

    if not kernel or not all(mirrors.values()):
        print("cap-parity: parsed an empty table, the file layout probably changed",
              file=sys.stderr)
        return 2

    problems = []
    for side, table in mirrors.items():
        for name, bit in sorted(kernel.items(), key=lambda kv: kv[1]):
            if name not in table:
                problems.append(
                    f"  {side} cannot name {name} (kernel bit {bit}, 0x{bit:X})")
            elif table[name] != bit:
                problems.append(f"  {name} DISAGREES: kernel 0x{bit:X}, "
                                f"{side} 0x{table[name]:X}")
        for name, bit in sorted(table.items(), key=lambda kv: kv[1]):
            if name not in kernel:
                problems.append(f"  {side} names {name} (0x{bit:X}) which the "
                                f"kernel does not define")

    if problems:
        print("cap-parity: the capability tables disagree", file=sys.stderr)
        for p in problems:
            print(p, file=sys.stderr)
        print("  the kernel is authoritative; fix the mirror that differs",
              file=sys.stderr)
        return 1

    print(f"cap-parity: {len(kernel)} capabilities, kernel and "
          f"{len(mirrors)} mirrors agree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
