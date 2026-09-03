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

"""python3 tools/nonos-enroll --from <dir>: the enrollment ceremony over
a reproducible builder artifact, five announced and verified steps."""

import argparse
import os
import subprocess
import sys

from . import artifact, ceremony


def main():
    ap = argparse.ArgumentParser(
        prog="nonos-enroll",
        description="Enroll the capsule set from a reproducible builder artifact.",
    )
    ap.add_argument("--from", dest="src", required=True, metavar="DIR",
                    help="downloaded enrollment-build artifact directory")
    args = ap.parse_args()

    if not os.path.isfile("Makefile"):
        sys.exit("run from the NONOS repository root")

    print("[1/5] load and check the builder artifact")
    root, bins, sums, commit = artifact.load(args.src)
    head = subprocess.run(["git", "rev-parse", "HEAD"],
                          capture_output=True, text=True).stdout.strip()
    if commit != head:
        sys.exit(f"builder artifact is for {commit[:9]}, this tree is {head[:9]};\n"
                 "check out the builder's commit so signatures match the sources")
    print(f"  commit {commit[:9]} matches this tree")

    print("[2/5] verify every binary against the builder manifest")
    artifact.verify_elfs(root, bins, sums)

    print("[3/5] place the builder binaries")
    ceremony.place(root, bins)

    print("[4/5] sign and enroll, the ordinary owner flow")
    ceremony.run_make("nonos-mk-all-capsules-attested", "sign-enroll")
    ceremony.reverify(bins, sums)

    print("[5/5] restamp the ledger and print the receipt")
    ceremony.run_make("nonos-mk-trust-ledger", "ledger")
    ceremony.receipt()


if __name__ == "__main__":
    main()
