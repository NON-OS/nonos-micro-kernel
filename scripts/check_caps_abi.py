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
"""Check the published capability ABI against the bits the kernel enforces.

`abi/caps.toml` is what an external toolchain reads to learn what a capability
bit means. The kernel is the only thing that decides that, so the file is
correct exactly when it agrees with `src/capabilities/types/bit.rs`.

The existing CI check compares four graphics entries against values written
into the check itself, which makes the check a third copy of the table rather
than a comparison between the two that exist.

Two failures are worth separating. A named bit whose value disagrees is a
disagreement and is obvious once looked at. A bit that the ABI names but the
kernel does not, sitting on a value the kernel already uses for something else,
is worse: a toolchain grants what it reads and the kernel enforces what it
knows, both without error.
"""

import argparse
import re
import sys
from pathlib import Path

KERNEL_BITS = Path("src/capabilities/types/bit.rs")
ABI_CAPS = Path("abi/caps.toml")


def canonical(name: str) -> str:
    return name.replace("_", "").upper()


def read_kernel(root: Path):
    text = (root / KERNEL_BITS).read_text()
    pairs = re.findall(r"Self::(\w+)\s*=>\s*(\d+)\s*,", text)
    return {canonical(n): (n, int(v)) for n, v in pairs}


def sections(text: str):
    """Split a flat TOML file into section name -> list of (key, value)."""
    out, current = {}, None
    for line in text.splitlines():
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        header = re.fullmatch(r"\[([\w.]+)\]", line)
        if header:
            current = header.group(1)
            out.setdefault(current, [])
            continue
        if current and "=" in line:
            key, value = line.split("=", 1)
            out[current].append((key.strip(), value.strip()))
    return out


def read_abi(root: Path):
    text = (root / ABI_CAPS).read_text()
    parsed = sections(text)
    bits = {}
    for key, raw in parsed.get("bits", []):
        bits[key] = int(raw.replace("_", ""), 16 if raw.startswith("0x") else 10)
    names = set()
    for group in ("groups", "delegation"):
        for _, raw in parsed.get(group, []):
            names.update(re.findall(r'"([^"]+)"', raw))
    return bits, names


def compare(kernel, abi_bits):
    by_value = {bit: name for name, (_, bit) in kernel.items()}
    fatal, absent = [], []
    for name, bit in sorted(abi_bits.items(), key=lambda kv: kv[1]):
        key = canonical(name)
        if key in kernel:
            expected = kernel[key][1]
            if expected != bit:
                fatal.append(f"  {name} DISAGREES: kernel 0x{expected:X}, "
                             f"abi 0x{bit:X}")
        elif bit in by_value:
            fatal.append(f"  {name} = 0x{bit:X} is not a kernel capability, and "
                         f"that bit is {by_value[bit]}")
        else:
            fatal.append(f"  {name} = 0x{bit:X} is not a kernel capability")
    for key, (name, bit) in sorted(kernel.items(), key=lambda kv: kv[1][1]):
        if not any(canonical(n) == key for n in abi_bits):
            absent.append(f"  {name} (0x{bit:X}) is enforced but not published")
    return fatal, absent


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--strict", action="store_true",
                    help="also fail when a kernel capability is unpublished")
    ap.add_argument("--allow-stale-groups", action="store_true",
                    help="report group names that [bits] does not define, but do "
                         "not fail on them; which capabilities a group should "
                         "hold is policy and cannot be derived from the enum")
    args = ap.parse_args()

    try:
        kernel = read_kernel(args.root)
        abi_bits, group_names = read_abi(args.root)
    except FileNotFoundError as e:
        print(f"caps-abi: {e}", file=sys.stderr)
        return 2
    if not kernel or not abi_bits:
        print("caps-abi: parsed an empty table, the file layout probably changed",
              file=sys.stderr)
        return 2

    fatal, absent = compare(kernel, abi_bits)
    stale = [f"  a group names {name}, which [bits] does not define"
             for name in sorted(group_names - set(abi_bits))]
    if stale and args.allow_stale_groups:
        print(f"caps-abi: {len(stale)} group names are undefined")
        for line in stale:
            print(line)
    else:
        fatal.extend(stale)

    if absent:
        print(f"caps-abi: {len(absent)} kernel capabilities are not published")
        for line in absent:
            print(line)
    if fatal:
        print("caps-abi: the published ABI contradicts the kernel", file=sys.stderr)
        for line in fatal:
            print(line, file=sys.stderr)
        print("  the kernel is authoritative; fix abi/caps.toml", file=sys.stderr)
        return 1
    if absent and args.strict:
        print("caps-abi: capabilities are enforced but unpublished", file=sys.stderr)
        return 1
    print(f"caps-abi: {len(abi_bits)} published bits agree with the kernel")
    return 0


if __name__ == "__main__":
    sys.exit(main())
