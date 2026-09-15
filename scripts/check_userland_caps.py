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
"""Check every userland copy of the capability table against the kernel's.

The kernel decides what a capability bit means, in `src/capabilities/types/
bit.rs`. Three places in userland restate that table so they can print a mask
by name: the desktop shell, the About window and the terminal. They cannot
include the kernel file, because capsules build for a different target, so
they are hand-synced mirrors.

`check_cap_parity.py` covers the two tables capsules declare through. The
three that only print a mask by name had no check, so a capability appended
in the wrong position would make one screen name a different capability from
the others, with every build green.

The positional arrays (desktop shell, terminal) must list every kernel
capability in bit order; About pairs a bit with a name and shows a subset,
so each name it carries must sit on the kernel's bit.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import read_kernel  # noqa: E402
from userland_caps_mirrors import (  # noqa: E402
    ABOUT, POSITIONAL, by_position, check_about, check_positional,
)

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    args = ap.parse_args()

    try:
        kernel = read_kernel(args.root)
        expected = by_position(kernel)
        bad = []
        for label, path in POSITIONAL.items():
            bad += check_positional(args.root, label, path, expected)
        bad += check_about(args.root, kernel)
    except FileNotFoundError as e:
        print(f"userland-caps: {e}", file=sys.stderr)
        return 2
    if not kernel:
        print("userland-caps: kernel table parsed empty", file=sys.stderr)
        return 2

    if bad:
        print("userland-caps: a userland copy of the capability table disagrees "
              "with the kernel", file=sys.stderr)
        for line in bad:
            print(f"  {line}", file=sys.stderr)
        return 1
    print(f"userland-caps: {len(POSITIONAL) + 1} mirrors agree with the "
          f"kernel's {len(kernel)} capabilities")
    return 0

if __name__ == "__main__":
    sys.exit(main())
