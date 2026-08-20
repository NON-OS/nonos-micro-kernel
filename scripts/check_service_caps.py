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
"""Find service capability constants that alias a different kernel capability.

The `CAP_*` constants in `src/services/caps/types.rs` are tested against the
process capability bitmap, which is built from `Capability::bit()`. A constant
written as an independent `1 << N` is therefore not a separate permission at
all. It is whichever kernel capability already owns that bit.

The gate still compiles, still reads correctly, and still returns true and
false. It just answers a different question than the one written down. Five of
these were found and bound to real bits; the rest were left, so this checks all
of them and says which are actually reached by a gate.
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import read_kernel  # noqa: E402

TYPES = Path("src/services/caps/types.rs")
CONST = re.compile(r"pub const (CAP_\w+)\s*:\s*u64\s*=\s*([^;]+);")


def read_constants(root: Path):
    """Constant name -> (value, whether it is bound to a kernel capability)."""
    out = {}
    for name, expr in CONST.findall((root / TYPES).read_text()):
        expr = expr.strip()
        bound = re.search(r"Capability::(\w+)\.bit\(\)", expr)
        if bound:
            out[name] = (None, bound.group(1))
            continue
        shift = re.fullmatch(r"1\s*<<\s*(\d+)", expr)
        if shift:
            out[name] = (1 << int(shift.group(1)), None)
    return out


def gated(root: Path):
    """Constants reached by a live gate, not merely defined and exported."""
    used = set()
    for path in (root / "src").rglob("capability.rs"):
        for line in path.read_text().splitlines():
            if "has_capability" in line or "check_service_cap" in line:
                used.update(re.findall(r"\bCAP_[A-Z_]+\b", line))
    return used


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--all", action="store_true",
                    help="report unreached constants too, not just live gates")
    args = ap.parse_args()

    try:
        kernel = read_kernel(args.root)
        consts = read_constants(args.root)
    except FileNotFoundError as e:
        print(f"service-caps: {e}", file=sys.stderr)
        return 2
    if not consts:
        print("service-caps: parsed no constants, the file layout probably changed",
              file=sys.stderr)
        return 2

    by_value = {bit: name for name, bit in kernel.values()}
    grantable = 0
    for _, bit in kernel.values():
        grantable |= bit
    live = gated(args.root)
    live_alias, idle_alias, sealed = [], [], []
    for name, (value, bound) in sorted(consts.items()):
        if bound is not None:
            continue
        if value in by_value:
            where = "reached by a gate" if name in live else "defined, no gate"
            row = f"  {name} = 0x{value:X} is really {by_value[value]}  ({where})"
            (live_alias if name in live else idle_alias).append(row)
        elif value & grantable == 0 and name in live:
            sealed.append(f"  {name} = 0x{value:X} is on no kernel capability, "
                          f"so its gate can never open")

    if live_alias:
        print("service-caps: live gates test a capability they do not name",
              file=sys.stderr)
        for row in live_alias:
            print(row, file=sys.stderr)
    if sealed:
        print("service-caps: live gates that nothing can satisfy", file=sys.stderr)
        for row in sealed:
            print(row, file=sys.stderr)
    if idle_alias and args.all:
        print("service-caps: unreached constants that would alias if used")
        for row in idle_alias:
            print(row)

    total = len(consts)
    bound = sum(1 for _, b in consts.values() if b is not None)
    print(f"service-caps: {total} constants, {bound} bound to kernel bits, "
          f"{len(live_alias)} live aliases, {len(sealed)} sealed gates, "
          f"{len(idle_alias)} dormant")
    return 1 if live_alias or sealed else 0


if __name__ == "__main__":
    sys.exit(main())
