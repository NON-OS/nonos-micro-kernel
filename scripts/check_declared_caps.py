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
"""Refuse to sign a capsule whose manifest claims different powers than its
source declared.

The source writes its capability set into a `.nonos.caps` section. The manifest
carries a number a human typed into `Capsule.mk`. Nothing previously compared
them, so a capsule could be signed for powers its code never asked for, or ask
for powers the manifest never granted. Both are silent until something fails at
runtime, and for a capsule written by an agent rather than a person the
manifest is the only thing standing between a user and whatever it decided to
do.

ELF is parsed here rather than shelled out to `nm` or `readelf`, because those
report symbols and release builds strip them.
"""

import argparse
import struct
import sys
from pathlib import Path
from typing import Optional

SECTION = ".nonos.caps"


def read_section(path: Path, want: str) -> Optional[bytes]:
    """Return the contents of a named section of a 64-bit little-endian ELF."""
    blob = path.read_bytes()
    if len(blob) < 64 or blob[:4] != b"\x7fELF":
        raise ValueError(f"{path} is not an ELF file")
    if blob[4] != 2 or blob[5] != 1:
        raise ValueError(f"{path} is not 64-bit little-endian ELF")

    e_shoff, = struct.unpack_from("<Q", blob, 0x28)
    e_shentsize, e_shnum, e_shstrndx = struct.unpack_from("<HHH", blob, 0x3A)
    if e_shoff == 0 or e_shnum == 0:
        raise ValueError(f"{path} has no section table")

    def section(i):
        off = e_shoff + i * e_shentsize
        name, _type, _flags, _addr, offset, size = struct.unpack_from("<IIQQQQ", blob, off)
        return name, offset, size

    _, strtab_off, strtab_size = section(e_shstrndx)
    strtab = blob[strtab_off:strtab_off + strtab_size]

    for i in range(e_shnum):
        name_off, offset, size = section(i)
        end = strtab.find(b"\0", name_off)
        if strtab[name_off:end].decode("ascii", "replace") == want:
            return blob[offset:offset + size]
    return None


def parse_caps(text: str) -> int:
    """Accept 0x-prefixed hex or decimal, the two forms Capsule.mk uses."""
    text = text.strip()
    return int(text, 16) if text.lower().startswith("0x") else int(text, 10)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("binary", type=Path, help="built capsule ELF")
    ap.add_argument("--manifest-caps", required=True,
                    help="CAPSULE_REQUIRED_CAPS from Capsule.mk")
    ap.add_argument("--allow-missing", action="store_true",
                    help="pass capsules built without the SDK, which have no section")
    args = ap.parse_args()

    try:
        raw = read_section(args.binary, SECTION)
    except ValueError as e:
        print(f"check-caps: {e}", file=sys.stderr)
        return 2

    if raw is None:
        if args.allow_missing:
            print(f"check-caps: {args.binary.name}: no {SECTION}, not an SDK capsule")
            return 0
        print(f"check-caps: {args.binary.name}: no {SECTION} section", file=sys.stderr)
        print("  built without sdk_main!, or the section was discarded by the linker",
              file=sys.stderr)
        return 1

    if len(raw) < 8:
        print(f"check-caps: {SECTION} is {len(raw)} bytes, expected at least 8",
              file=sys.stderr)
        return 1

    declared, = struct.unpack_from("<Q", raw, 0)
    manifest = parse_caps(args.manifest_caps)

    if declared == manifest:
        print(f"check-caps: {args.binary.name}: 0x{declared:X} declared and granted")
        return 0

    # Report the difference in both directions. Which way it points changes what
    # the developer has to fix: extra manifest bits are an over-grant the kernel
    # will honour, extra source bits are a program that will fail at runtime.
    print(f"check-caps: {args.binary.name}: MANIFEST DISAGREES WITH SOURCE", file=sys.stderr)
    print(f"  source declares : 0x{declared:X}", file=sys.stderr)
    print(f"  manifest grants : 0x{manifest:X}", file=sys.stderr)
    over = manifest & ~declared
    under = declared & ~manifest
    if over:
        print(f"  granted but not declared : 0x{over:X}  (over-grant, kernel would honour it)",
              file=sys.stderr)
    if under:
        print(f"  declared but not granted : 0x{under:X}  (app would fail at runtime)",
              file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
