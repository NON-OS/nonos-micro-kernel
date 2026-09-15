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
"""The scan behind check_unreachable.py: exported functions nobody calls.

A `pub fn` in a library crate is never dead to rustc, because something
outside the crate might call it. Nothing outside this crate ever will: the
kernel is the top of its own tree. So the dead-code lint is silent across the
whole public surface, and a mechanism can be written, reviewed, merged and
shipped without one line of it ever running.

Two things decide whether a name counts as called, and both were wrong in the
first cut of this file. A `pub use` only carries a name up the module tree, so
an import is not a call: the KASLR validator was re-exported through three
levels of mod.rs and invoked from nowhere. And a definition mentions its own
name, so counting that line makes every function look used.

Architectures the build is not targeting are skipped. Their code does not
compile into anything, so "nothing calls it" says nothing about it.
"""

import re

DEFINITION = re.compile(
    r"^\s*pub(?:\(crate\))?\s+(?:const\s+|async\s+|unsafe\s+|extern\s+\"C\"\s+)*fn\s+([a-z_][A-Za-z0-9_]*)"
)
IMPORT = re.compile(r"^\s*(pub\s+)?use\s")
COMMENT = re.compile(r"^\s*(//|/\*|\*)")
WORD = re.compile(r"\b([a-z_][A-Za-z0-9_]*)\b")
# Reached from assembly or from the linker, never by name from Rust.
EXPORTED_TO_LINKER = re.compile(r"#\[(no_mangle|export_name)")
INACTIVE = ("src/arch/aarch64/", "src/arch/riscv64/")


def sources(root, roots=("src",)):
    for base in roots:
        for path in sorted((root / base).rglob("*.rs")):
            rel = path.relative_to(root)
            if any(str(rel).startswith(skip) for skip in INACTIVE):
                continue
            yield rel, path.read_text(errors="replace").splitlines()


def definitions(root, roots=("src",)):
    """(name, "path:line") for every exported function that builds."""
    found = []
    for rel, lines in sources(root, roots):
        for n, line in enumerate(lines, 1):
            match = DEFINITION.match(line)
            if not match:
                continue
            if EXPORTED_TO_LINKER.search("\n".join(lines[max(0, n - 4):n - 1])):
                continue
            found.append((match.group(1), f"{rel}:{n}"))
    return found


def called(root, roots=("src",)):
    """Every name mentioned somewhere that is not an import or a definition."""
    seen = set()
    for _, lines in sources(root, roots):
        for line in lines:
            if IMPORT.match(line) or COMMENT.match(line) or DEFINITION.match(line):
                continue
            seen.update(WORD.findall(line))
    return seen


def unreachable(root, roots=("src",)):
    """Exported functions whose name appears at no call site in the tree."""
    live = called(root, roots)
    return sorted(f"{site} {name}" for name, site in definitions(root, roots) if name not in live)
