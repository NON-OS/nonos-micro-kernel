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
"""No lint switch may be added to the tree.

An #[allow] silences a diagnostic instead of answering it. Every one in src
and userland is listed against scripts/baselines/allows.txt, a list that may
only shrink. Vendored and upstream sources are not ours to lint.
"""

import re
import sys
import tempfile
from pathlib import Path

import gate

BASELINE = Path("scripts/baselines/allows.txt")
ROOTS = ["src", "userland"]
SKIP = ("userland/vendor/", "userland/upstream-src/", "/target/")
MARK = re.compile(r"#!?\[\s*allow\s*\(")


def sites(root):
    out = []
    for base in ROOTS:
        for p in sorted((root / base).rglob("*.rs")):
            rel = str(p.relative_to(root))
            if any(s in rel + "/" for s in SKIP):
                continue
            for n, line in enumerate(p.read_text(errors="replace").splitlines(), 1):
                if MARK.search(line):
                    out.append(f"{rel}:{n}")
    return out


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for base in ROOTS:
            (root / base).mkdir()
        (root / "src/decoy.rs").write_text("#[allow(dead_code)]\nfn f() {}\n")
        found = sites(root)
        if found != ["src/decoy.rs:1"]:
            print(f"allows: self-test failed, found {found}")
            return 1
    print("allows: self-test passed, the decoy was reported")
    return 0


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        return self_test()
    return gate.run("allows", "new lint switch at", sites, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())
