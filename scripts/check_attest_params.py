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

"""Fail if any file redeclares a shared attestation parameter locally.

The soundness parameters have one home, nonos-stark/src/attest_params.rs. A
prover and a verifier that read different values run different permutations and
no proof verifies. That is not a loud failure at build time: each side compiles
and passes its own tests, and only a real attestation, the kernel gate at boot,
disagrees.

This happened. The round count moved from three to five in attest_params and
the enrollment tool followed, but the kernel self-attestation, the capsule
spawn gate, and the bootloader pre-jump check each kept a local `const
LOG_ROUNDS: u32 = 3`. The kernel then refused its own boot. A local copy is
never right: import the constant.
"""

import re
import sys
from pathlib import Path

PARAMS = ["LOG_ROUNDS", "N_QUERIES", "GRIND_BITS", "EXTRA_BLOWUP_BITS"]
SOURCE = Path("nonos-stark/src/attest_params.rs")
TREES = ["src", "nonos-bootloader/src", "nonos-stark-enroll/src", "userland"]

DECL = re.compile(rf"^\s*(?:pub(?:\([^)]*\))?\s+)?const ({'|'.join(PARAMS)})\s*:", re.M)


def main() -> int:
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
    found = []
    for tree in TREES:
        base = root / tree
        if not base.exists():
            continue
        for path in base.rglob("*.rs"):
            if "target" in path.parts or path.resolve() == (root / SOURCE).resolve():
                continue
            text = path.read_text(errors="ignore")
            for m in DECL.finditer(text):
                line = text[: m.start()].count("\n") + 1
                rel = path.relative_to(root)
                found.append(f"  {rel}:{line} redeclares {m.group(1)}")

    if found:
        print("attest-params: a soundness parameter is declared outside its one home",
              file=sys.stderr)
        for f in found:
            print(f, file=sys.stderr)
        print(f"  import from {SOURCE.name} instead", file=sys.stderr)
        return 1
    print(f"attest-params: {', '.join(PARAMS)} live only in {SOURCE.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
