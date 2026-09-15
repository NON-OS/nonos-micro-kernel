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
"""Which syscalls the kernel dispatches that no userland code can reach.

The failure this catches has one shape: a capability that is implemented,
sits in the dispatch table, and is referenced by nothing. No compiler warns
about it and no test exercises it, so it survives until someone notices the
feature it was meant to serve does not work. In one day that shape was
"""

import argparse
import pathlib
import sys

from syscall_reach_scan import KERNEL_NUMBERS, kernel_syscalls, unreachable, userland_text


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--count-only", action="store_true", help="print the number alone")
    ap.add_argument(
        "--baseline",
        type=pathlib.Path,
        help="fail if the count exceeds the integer in this file",
    )
    args = ap.parse_args()

    if not KERNEL_NUMBERS.exists():
        print("run from the repository root", file=sys.stderr)
        return 2

    dark = unreachable(kernel_syscalls(), userland_text())
    if args.count_only:
        print(len(dark))
        return 0
    for name, tag in dark:
        print(f"{name:<26} {tag}")
    print(f"unreachable from userland: {len(dark)}")
    if args.baseline is None:
        return 0
    allowed = int(args.baseline.read_text().strip())
    if len(dark) > allowed:
        print(
            f"::error::{len(dark)} syscalls are unreachable, baseline allows {allowed}. "
            "A syscall was added or a caller removed without anything reaching it; "
            "wire it, or explain it in scripts/baselines/syscall-unreachable.notes.md "
            "and raise the count in the same commit."
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
