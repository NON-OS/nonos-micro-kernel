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
"""The matrix as a table on the terminal and as JSON on disk."""

import json
from dataclasses import asdict


def write_json(runs, path):
    path.write_text(json.dumps([asdict(r) for r in runs], indent=2) + "\n")


def table(runs):
    """One row per cell: passes over attempts, accelerator, slowest boot."""
    rows = {}
    for r in runs:
        row = rows.setdefault(r.cell, {"ok": 0, "n": 0, "accel": r.accel, "worst": 0.0, "why": []})
        row["n"] += 1
        row["worst"] = max(row["worst"], r.seconds)
        if r.failures:
            row["why"].extend(f"attempt {r.attempt}: {f}" for f in r.failures)
        else:
            row["ok"] += 1
    lines = [f"{'cell':<18} {'boots':>7} {'accel':<5} {'slowest':>8}"]
    for name, row in rows.items():
        lines.append(f"{name:<18} {row['ok']:>3}/{row['n']:<3} {row['accel']:<5} {row['worst']:>7.1f}s")
        lines.extend(f"    {why}" for why in row["why"])
    return "\n".join(lines)


def failed(runs):
    return [r for r in runs if r.failures]
