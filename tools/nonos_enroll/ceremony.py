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

"""Place, sign, enroll, and prove. The signing make flow is the normal
owner flow; what makes this a ceremony is the check on either side of
it: the builder bytes are verified before placement, and re-verified
after signing so nothing that got enrolled differs from what the
reproducible builder produced."""

import shutil
import subprocess
import sys
from pathlib import Path

from .artifact import b3


def place(root, bins):
    for path in bins:
        src = root / "elfs" / Path(path).name
        dst = Path(path)
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
        dst.touch()
    print(f"  {len(bins)} binaries placed with fresh mtimes")


def run_make(target, log_name):
    log = Path("target/enroll-ceremony") / f"{log_name}.log"
    log.parent.mkdir(parents=True, exist_ok=True)
    with open(log, "w") as sink:
        proc = subprocess.run(["make", target], stdout=sink, stderr=subprocess.STDOUT)
    if proc.returncode != 0:
        tail = log.read_text(errors="replace").splitlines()[-12:]
        print(f"  {target} failed; log {log}")
        for line in tail:
            print(f"    {line}")
        sys.exit(proc.returncode)


def reverify(bins, sums):
    drifted = [p for p in bins if b3(Path(p)) != sums[Path(p).name]]
    if drifted:
        sys.exit(
            "REFUSED: signing rebuilt these binaries, the enrolled bytes would\n"
            "not be the builder's. Ledger not restamped. Drifted:\n  "
            + "\n  ".join(drifted)
        )
    print(f"  {len(bins)} binaries unchanged through signing; enrolled = builder bytes")


def receipt():
    root = Path("nonos-data/trust/policy/zk_capsule_policy_root.bin")
    if root.is_file():
        print(f"  policy root  {root.read_bytes().hex()}")
    ledger = Path("nonos-data/trust/MANIFEST.sha256")
    print(f"  ledger       {sum(1 for _ in open(ledger))} artifacts restamped")
    print("\nreview the nonos-data diff, then commit the keystore and bump the pin")
