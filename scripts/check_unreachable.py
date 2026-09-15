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
"""Exported kernel functions that nothing calls.

rustc's dead-code lint never fires on a `pub fn` in a library crate, and the
kernel is a library crate. So a security mechanism can be written, reviewed,
merged and shipped with no line of it ever running, which is what happened to
KASLR, to the stack guards, to the kernel W^X check and to the periodic
security scan above it. Each site is listed in
scripts/baselines/unreachable.txt, a list that may only shrink.

A name is reachable when some other file mentions it outside an import. A
`pub use` only carries the name up the module tree; it is not a call.
"""

import sys
import tempfile
from pathlib import Path

import gate
from unreachable_scan import unreachable

BASELINE = Path("scripts/baselines/unreachable.txt")


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        (root / "src").mkdir()
        (root / "src/decoy.rs").write_text("pub fn never_called() {}\npub fn called() {}\n")
        (root / "src/user.rs").write_text("use crate::decoy::never_called;\nfn f() { called(); }\n")
        found = unreachable(root)
        if found != ["src/decoy.rs:1 never_called"]:
            print(f"unreachable: self-test failed, found {found}")
            return 1
    print("unreachable: self-test passed, the decoy was reported and the import did not save it")
    return 0


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        return self_test()
    return gate.run("unreachable", "nothing calls", unreachable, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())
