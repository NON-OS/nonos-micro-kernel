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
"""Check the published syscall ABI against the numbers the kernel dispatches.

`abi/syscalls.toml` is what a libc or a foreign toolchain reads to learn how
to call this kernel. The kernel decides, in two places: the `SyscallNumber`
enum and the microkernel `SYS_*` constants, both built from `tag4`.

The only check that touched this file compared it against a handful of
graphics IDs written into the check itself, so nothing ever compared the
published contract to the kernel.

A syscall the kernel dispatches and the ABI does not publish is not a hole,
it is a capability nobody outside the tree can reach. The reverse is worse: a
published number the kernel does not answer is a documented call that returns
whatever the dispatch default happens to be.
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import sections  # noqa: E402

ABI = Path("abi/syscalls.toml")
SOURCES = {
    "enum": (Path("src/syscall/numbers/defs.rs"),
             re.compile(r"(\w+)\s*=\s*tag4\(b\"(\w{4})\"\)")),
    "microkernel": (Path("src/syscall/microkernel/numbers.rs"),
                    re.compile(r"pub const (\w+)\s*:\s*u64\s*=\s*tag4\(b\"(\w{4})\"\)")),
}


def tag4(tag: str) -> int:
    b = tag.encode()
    return b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24)


def read_kernel(root: Path):
    """tag -> {source: rust name}. A tag may legitimately appear in both."""
    out = {}
    for side, (path, pattern) in SOURCES.items():
        for name, tag in pattern.findall((root / path).read_text()):
            out.setdefault(tag, {})[side] = name
    return out


def read_abi(root: Path):
    parsed = sections((root / ABI).read_text())
    numbers = {}
    for key, raw in parsed.get("numbers", []):
        numbers[key] = int(raw, 16 if raw.startswith("0x") else 10)
    described = {s.split(".", 1)[1] for s in parsed if s.startswith("desc.")}
    return numbers, described


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--strict", action="store_true",
                    help="fail when a dispatched syscall is unpublished, rather "
                         "than only listing it")
    args = ap.parse_args()

    try:
        kernel = read_kernel(args.root)
        published, described = read_abi(args.root)
    except FileNotFoundError as e:
        print(f"syscall-abi: {e}", file=sys.stderr)
        return 2
    if not kernel or not published:
        print("syscall-abi: parsed an empty table, the file layout probably changed",
              file=sys.stderr)
        return 2

    fatal, unpublished = [], []
    for tag, value in sorted(published.items()):
        if tag not in kernel:
            fatal.append(f"  {tag} = 0x{value:X} is published and the kernel "
                         f"does not dispatch it")
        elif value != tag4(tag):
            fatal.append(f"  {tag} DISAGREES: tag4 gives 0x{tag4(tag):X}, "
                         f"abi says 0x{value:X}")
        elif tag not in described:
            fatal.append(f"  {tag} is in [numbers] with no [desc.{tag}] block")
    for tag, names in sorted(kernel.items()):
        if tag not in published:
            where = ", ".join(f"{s}:{n}" for s, n in sorted(names.items()))
            unpublished.append(f"  {tag} ({where})")

    if unpublished:
        print(f"syscall-abi: {len(unpublished)} dispatched syscalls are unpublished")
        for line in unpublished:
            print(line)
    if fatal:
        print("syscall-abi: the published ABI contradicts the kernel", file=sys.stderr)
        for line in fatal:
            print(line, file=sys.stderr)
        return 1
    if unpublished and args.strict:
        return 1
    print(f"syscall-abi: {len(published)} published syscalls agree with the kernel")
    return 0


if __name__ == "__main__":
    sys.exit(main())
