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

"""python3 -m nonos_build: build NONOS from source the way the installer
installs it, one announced and verified step at a time."""

import argparse
import os
import sys

from . import steps
from .shell import confirm, out

RITUAL = [
    ("check the machine", steps.doctor),
    ("mint the dev identity", steps.identity),
    ("build kernel and capsules", steps.kernel),
    ("prove the kernel", steps.attest),
    ("verify the trust ledger", steps.verify),
    ("pack the boot partition", steps.image),
    ("print the receipt", steps.receipt),
]


def main():
    ap = argparse.ArgumentParser(
        prog="nonos-build",
        description="Guided NONOS source build: seven steps, each verified.",
    )
    ap.add_argument("--yes", action="store_true", help="run all steps without pausing")
    ap.add_argument("--from-step", type=int, default=1, metavar="N", help="resume at step N")
    args = ap.parse_args()

    if not os.path.isfile("Makefile"):
        print("run from the NONOS repository root")
        sys.exit(1)
    env = dict(os.environ)
    env.setdefault("NONOS_DEV", "1")

    out("NONOS build")
    out("===========")
    for index, (title, step) in enumerate(RITUAL, start=1):
        if index < args.from_step:
            continue
        out(f"\n[{index}/{len(RITUAL)}] {title}")
        step(env)
        if index < len(RITUAL):
            confirm("next step", args.yes)
    out("\nbuild complete; boot it with: make qemu")


if __name__ == "__main__":
    main()
