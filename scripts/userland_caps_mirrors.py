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
"""The three screen-side copies of the capability table and the two checks
that hold them to the kernel. Read by check_userland_caps."""

import re
from pathlib import Path

POSITIONAL = {
    "desktop shell": Path("userland/capsule_desktop_shell/src/render/cap_names.rs"),
    "terminal": Path("userland/capsule_terminal/src/command/builtin/cap_names.rs"),
}
ABOUT = Path("userland/capsule_about/src/about/data/caps.rs")
ABOUT_ENTRY = re.compile(r'bit:\s*(\w+),\s*name:\s*b"(\w+)"')
ARRAY_ENTRY = re.compile(r'^\s*b"(\w+)",\s*$', re.M)
SHIFT_CONST = re.compile(r"pub const (\w+): u64 = 1 << (\d+);")


def by_position(kernel):
    """bit position -> kernel name, from the exact bit values."""
    return {v.bit_length() - 1: name for name, v in kernel.values()}


def check_positional(root, label, path, expected):
    names = ARRAY_ENTRY.findall((root / path).read_text())
    bad = []
    if len(names) != len(expected):
        bad.append(f"{label}: {len(names)} entries, kernel defines {len(expected)}")
    for i, name in enumerate(names):
        want = expected.get(i)
        if name != want:
            bad.append(f"{label}: position {i} is {name}, kernel bit {i} is {want}")
    return bad


def check_about(root, kernel):
    """About names its bits through `pub const X: u64 = 1 << n;` and then
    pairs those constants with display names, so resolve the constant first."""
    text = (root / ABOUT).read_text()
    shifts = {c: int(n) for c, n in SHIFT_CONST.findall(text)}
    bad = []
    for const, name in ABOUT_ENTRY.findall(text):
        want = kernel.get(name.replace("_", "").upper())
        if want is None:
            bad.append(f"about: {name} is not a kernel capability")
        elif const not in shifts:
            bad.append(f"about: {name} uses {const}, which is not a plain shift")
        elif 1 << shifts[const] != want[1]:
            bad.append(f"about: {name} is 1 << {shifts[const]}, kernel says {want[1]}")
    return bad
