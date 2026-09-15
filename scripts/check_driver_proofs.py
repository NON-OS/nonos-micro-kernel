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
"""Every driver capsule has a conformance proof crate, or the build says which
one does not. A driver without a proof is a claim about hardware nobody can
check without the hardware.

The drivers still without one are listed in
scripts/baselines/unproved_drivers.txt, a list that may only shrink: a new
driver ships with its proof or not at all.
"""

import sys
from pathlib import Path

import gate

BASELINE = Path("scripts/baselines/unproved_drivers.txt")
# Proof crates whose name is not <driver>_proofs.
ALIASES = {"usb_hid": "usb_proofs", "ahci": "ahci_link_proofs"}


def drivers(root):
    prefix = "capsule_driver_"
    return sorted(p.name[len(prefix):] for p in (root / "userland").glob(f"{prefix}*"))


def unproved(root):
    """The drivers whose proof crate is not in the tree."""
    out = []
    for d in drivers(root):
        crate = ALIASES.get(d, f"{d}_proofs")
        if not (root / "userland" / crate / "Cargo.toml").exists():
            out.append(f"capsule_driver_{d}")
    return out


def main():
    args = gate.parser(__doc__, self_test=False).parse_args()
    status = gate.run("driver-proofs", "no proof crate for", unproved, BASELINE, args)
    if not args.write_baseline:
        total, missing = len(drivers(args.root)), len(unproved(args.root))
        print(f"driver-proofs: {total - missing} of {total} drivers carry a proof crate")
    return status


if __name__ == "__main__":
    sys.exit(main())
