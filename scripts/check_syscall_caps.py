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
"""The capability each published syscall lists against the one the cap table
demands. When they disagree a capsule author requests the published one, is
granted it, and gets EPERM on the first call with nothing in a log to say why.
Predicates resolve through their definitions under src/syscall/caps/checks; a
predicate naming no capability publishes as valid_token.
"""
import re
from pathlib import Path

ARM_RE = re.compile(r"((?:\s*\|?\s*SyscallNumber::\w+)+)\s*=>\s*(.*?)(?=\n\s*(?:/\*|//|SyscallNumber::|\|\s*SyscallNumber::|\}\)|$))", re.S)
VARIANT_RE = re.compile(r"SyscallNumber::(\w+)")
PRED_RE = re.compile(r"can_(\w+)\(\)|Capability::(\w+)")
DEF_RE = re.compile(r"pub fn (can_\w+)\(&self\) -> bool \{(.*?)\n    \}", re.S)
CAP_RE = re.compile(r"Capability::(\w+)")


def predicates(checks_dir):
    """can_x -> the capabilities its body grants on."""
    text = "\n".join(p.read_text() for p in sorted(Path(checks_dir).rglob("*.rs")))
    return {name: set(CAP_RE.findall(body)) for name, body in DEF_RE.findall(text)}


def demanded(cap_table_dir, checks_dir):
    """variant -> the capabilities the kernel asks for, or valid_token alone."""
    preds = predicates(checks_dir)
    out = {}
    text = "\n".join(p.read_text() for p in sorted(Path(cap_table_dir).rglob("*.rs")))
    for arms, expr in ARM_RE.findall(text):
        names = set()
        for can, cap in PRED_RE.findall(expr):
            names |= preds.get("can_" + can, set()) if can else {cap}
        for variant in VARIANT_RE.findall(arms):
            out[variant] = names or {"valid_token"}
    return out


def published(sections):
    """tag -> set of caps the ABI lists, from the parsed TOML sections."""
    out = {}
    for name, pairs in sections.items():
        if name.startswith("desc."):
            for key, raw in pairs:
                if key == "caps":
                    out[name[5:]] = set(re.findall(r'"([^"]+)"', raw))
    return out


def compare(registry, kernel, abi):
    """One line per syscall whose published caps are not what the kernel asks."""
    bad = []
    for tag, variant in sorted(registry.items()):
        want, have = kernel.get(variant), abi.get(tag)
        if want is None or have is None:
            continue
        if want != have:
            bad.append(f"{tag} ({variant}) publishes caps {sorted(have)} but the cap "
                       f"table demands {sorted(want)}")
    return bad
