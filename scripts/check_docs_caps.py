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
"""Check the published capability table in the docs against the kernel.

The docs are a mirror of the same table the kernel enum defines, and they had
drifted: the page said twenty-two variants against a kernel that enforced
twenty-eight, and cited `src/capabilities/types.rs`, a path that no longer
exists.

A stale table in documentation is worse than none. A reader who grants what
the page says gets what the kernel says, and the difference is silent.

The docs live in their own repository, checked out here as a submodule, so
this skips rather than fails when it is not present.
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import read_kernel  # noqa: E402

PAGE = Path("docs/security/capabilities-and-tokens.md")
ENTRY = re.compile(r"\b([A-Z][A-Za-z]+)\s+(\d+)\b")
WORDS = {22: "twenty-two", 23: "twenty-three", 27: "twenty-seven",
         28: "twenty-eight", 29: "twenty-nine", 30: "thirty",
         31: "thirty-one", 32: "thirty-two", 33: "thirty-three"}


def table(text: str):
    """The fenced block holding the bit table, as name -> value."""
    block = re.search(r"```\n(.*?CoreExec.*?)```", text, re.S)
    if not block:
        return {}
    return {n: int(v) for n, v in ENTRY.findall(block.group(1))}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    args = ap.parse_args()

    page = args.root / PAGE
    if not page.exists():
        print(f"docs-caps: {PAGE} not checked out, skipping")
        return 0
    try:
        kernel = read_kernel(args.root)
    except FileNotFoundError as e:
        print(f"docs-caps: {e}", file=sys.stderr)
        return 2

    text = page.read_text()
    documented = table(text)
    if not documented:
        print("docs-caps: found no bit table on the page", file=sys.stderr)
        return 2

    expected = {name: bit for name, bit in kernel.values()}
    problems = []
    for name, bit in sorted(documented.items(), key=lambda kv: kv[1]):
        if name not in expected:
            problems.append(f"  the page lists {name}, which the kernel does not define")
        elif expected[name] != bit:
            problems.append(f"  {name} DISAGREES: kernel {expected[name]}, page {bit}")
    for name, bit in sorted(expected.items(), key=lambda kv: kv[1]):
        if name not in documented:
            problems.append(f"  {name} ({bit}) is enforced and the page omits it")

    word = WORDS.get(len(expected))
    if word and word not in text:
        problems.append(f"  the page does not say the set has {word} variants")
    for dead in re.findall(r"`src/capabilities/types\.rs[^`]*`", text):
        problems.append(f"  cites {dead}, which no longer exists")

    if problems:
        print("docs-caps: the documented capability table is out of date",
              file=sys.stderr)
        for p in problems:
            print(p, file=sys.stderr)
        return 1
    print(f"docs-caps: {len(documented)} documented capabilities agree with the kernel")
    return 0


if __name__ == "__main__":
    sys.exit(main())
