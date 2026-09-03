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

"""The builder artifact: its layout, its self-consistency, and the
hash checks. Everything is verified before a single byte is placed."""

import subprocess
import sys
from pathlib import Path


def b3(path):
    out = subprocess.run(["b3sum", str(path)], capture_output=True, text=True)
    if out.returncode != 0:
        sys.exit("b3sum is required on PATH for the ceremony")
    return out.stdout.split()[0]


def load(root):
    root = Path(root)
    for name in ["bins.list", "BLAKE3SUMS", "COMMIT", "COUNT", "elfs"]:
        if not (root / name).exists():
            sys.exit(f"artifact incomplete: {name} missing under {root}")
    bins = [l for l in (root / "bins.list").read_text().splitlines() if l]
    sums = {}
    for line in (root / "BLAKE3SUMS").read_text().splitlines():
        h, n = line.split(maxsplit=1)
        sums[n.strip()] = h
    count = int((root / "COUNT").read_text().strip())
    if not (len(bins) == len(sums) == count):
        sys.exit(f"artifact inconsistent: {len(bins)} paths, {len(sums)} sums, COUNT {count}")
    commit = (root / "COMMIT").read_text().strip()
    return root, bins, sums, commit


def verify_elfs(root, bins, sums):
    bad = []
    for path in bins:
        name = Path(path).name
        elf = root / "elfs" / name
        if name not in sums or not elf.is_file():
            bad.append(f"{name}: missing")
        elif b3(elf) != sums[name]:
            bad.append(f"{name}: hash mismatch")
    if bad:
        sys.exit("builder artifact refused:\n  " + "\n  ".join(bad))
    print(f"  {len(bins)} capsule binaries verified against the builder manifest")
