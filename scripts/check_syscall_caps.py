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

A gate is either all of a set of capabilities (`caps` in the ABI) or any one of
them (`caps_any`). Predicates resolve through their definitions under
src/syscall/caps/checks and src/capabilities/token/types. A predicate the
checker cannot find is an error, never a gate that asks for nothing: reading it
as `valid_token` is how twenty-five hardware gates were published as open.
"""
import re
from pathlib import Path

CHECK_DIRS = (Path("src/syscall/caps/checks"), Path("src/capabilities/token/types"))
ARM_RE = re.compile(r"((?:\s*\|?\s*SyscallNumber::\w+)+)\s*=>\s*(.*?)(?=\n\s*(?:/\*|//|SyscallNumber::|\|\s*SyscallNumber::|\}\)|$))", re.S)
VARIANT_RE = re.compile(r"SyscallNumber::(\w+)")
PRED_RE = re.compile(r"can_(\w+)\(\)|Capability::(\w+)")
DEF_RE = re.compile(r"pub fn (can_\w+)\(&self\) -> bool \{(.*?)\n    \}", re.S)
CAP_RE = re.compile(r"Capability::(\w+)")
OPEN = ("all", frozenset({"valid_token"}))


def gate(caps, joined_by_or):
    """A set of capabilities as the ABI would publish it."""
    if not caps:
        return OPEN
    if len(caps) == 1 or not joined_by_or:
        return ("all", frozenset(caps))
    return ("any", frozenset(caps))


def predicates(root):
    """can_x -> its gate, from every file a token predicate is defined in."""
    out = {}
    for d in CHECK_DIRS:
        for p in sorted((root / d).rglob("*.rs")):
            for name, body in DEF_RE.findall(p.read_text()):
                caps = set(CAP_RE.findall(body))
                if "||" in body and "&&" in body.replace("&& self.is_valid()", ""):
                    out[name] = ("mixed", frozenset(caps))
                else:
                    out[name] = gate(caps, "||" in body)
    return out


def demanded(root, cap_table_dir):
    """variant -> the gate the kernel applies, and every arm it could not read."""
    preds = predicates(root)
    out, problems = {}, []
    text = "\n".join(p.read_text() for p in sorted((root / cap_table_dir).rglob("*.rs")))
    for arms, expr in ARM_RE.findall(text):
        variants = VARIANT_RE.findall(arms)
        parts = []
        for can, cap in PRED_RE.findall(expr):
            if not can:
                parts.append(("all", frozenset({cap})))
            elif "can_" + can in preds:
                parts.append(preds["can_" + can])
            else:
                problems.append(f"{'/'.join(variants)} is gated on can_{can}, which no "
                                f"predicate file defines")
        found = _combine(parts, "||" in expr)
        if found is None:
            problems.append(f"{'/'.join(variants)} combines gates the ABI cannot express")
            continue
        for variant in variants:
            out[variant] = found
    return out, problems


def _combine(parts, joined_by_or):
    """One gate from an arm's parts, or None when it mixes all-of and any-of."""
    if not parts:
        return OPEN
    if len(parts) == 1:
        return None if parts[0][0] == "mixed" else parts[0]
    if any(kind != "all" for kind, _ in parts) or joined_by_or:
        return None
    return ("all", frozenset().union(*(s for _, s in parts)))


def published(sections):
    """tag -> the gate the ABI publishes, from `caps` or `caps_any`."""
    out = {}
    for name, pairs in sections.items():
        if not name.startswith("desc."):
            continue
        fields = dict(pairs)
        names = {k: frozenset(re.findall(r'"([^"]+)"', v)) for k, v in fields.items()
                 if k in ("caps", "caps_any")}
        if len(names) == 2:
            out[name[5:]] = ("both", names["caps"] | names["caps_any"])
        elif "caps" in names:
            out[name[5:]] = ("all", names["caps"])
        elif "caps_any" in names:
            out[name[5:]] = gate(names["caps_any"], True)
    return out


def show(g):
    kind, names = g
    return f"{'caps' if kind == 'all' else 'caps_any' if kind == 'any' else kind} {sorted(names)}"


def compare(registry, kernel, abi, problems=()):
    """One line per syscall whose published gate is not the one the kernel applies."""
    bad = list(problems)
    for tag, variant in sorted(registry.items()):
        want, have = kernel.get(variant), abi.get(tag)
        if want is None or have is None:
            continue
        if want != have:
            bad.append(f"{tag} ({variant}) publishes {show(have)} but the cap table "
                       f"demands {show(want)}")
    return bad
