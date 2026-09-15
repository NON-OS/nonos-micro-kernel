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
"""Find the places a driver or the kernel admits it does not do the job.
A word like unsupported or stub in shipping code is a promise the hardware
will not get what it asked for. Each site is listed in scripts/baselines/
stubs.txt, a list that may only shrink. Tests and proof crates are not
scanned, since saying what is unsupported is their job."""

import re
import sys
import tempfile
from pathlib import Path

import gate

BASELINE = Path("scripts/baselines/stubs.txt")
ROOTS = ["src", "userland"]
MARK = re.compile(r"\bstubs?\b|\bnot (?:yet )?(?:supported|implemented)\b|\bunsupported\b|\bunimplemented\b", re.I)


def shipping(path):
    s = str(path)
    return "/target/" not in s and "_proofs/" not in s and "/tests/" not in s and not s.endswith("tests.rs")


def sites(root):
    """`path:line` for every marker in shipping Rust source under the roots."""
    out = []
    for base in ROOTS:
        for p in sorted((root / base).rglob("*.rs")):
            if not shipping(p):
                continue
            for n, line in enumerate(p.read_text(errors="replace").splitlines(), 1):
                if MARK.search(line):
                    out.append(f"{p.relative_to(root)}:{n}")
    return out


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for base in ROOTS:
            (root / base).mkdir()
        (root / "src/decoy.rs").write_text("fn f() -> i64 {\n    -38 // unsupported on this part\n}\n")
        found = sites(root)
        if found != ["src/decoy.rs:2"]:
            print(f"stubs: self-test failed, found {found}")
            return 1
    print("stubs: self-test passed, the decoy was reported")
    return 0


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        return self_test()
    return gate.run("stubs", "new admission of unsupported work at", sites, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())
