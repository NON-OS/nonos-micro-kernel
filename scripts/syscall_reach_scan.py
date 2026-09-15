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
"""What check_syscall_reach reads: the kernel's tags, the userland corpus, and
the set of tags the corpus never names.
"""

import pathlib
import re

KERNEL_NUMBERS = pathlib.Path("src/syscall/microkernel/numbers.rs")
USERLAND = pathlib.Path("userland")
SKIP = ("/target/", "/vendor/", "/third_party/")

TAG_RE = re.compile(r'pub const (SYS_\w+): u64 = tag4\(b"(....)"\);')


def tag_hex(tag: str) -> str:
    """The little-endian i64 a tag4 call produces, as libc writes it by hand."""
    value = 0
    for shift, byte in enumerate(tag.encode()):
        value |= byte << (8 * shift)
    hex_digits = f"{value:08X}"
    return f"{hex_digits[:4]}_{hex_digits[4:]}"


def kernel_syscalls() -> dict[str, str]:
    text = KERNEL_NUMBERS.read_text()
    return {tag: name for name, tag in TAG_RE.findall(text)}


def userland_text() -> str:
    parts = []
    for path in USERLAND.rglob("*.rs"):
        s = str(path)
        if any(k in s for k in SKIP):
            continue
        parts.append(path.read_text(errors="replace"))
    return "\n".join(parts)


def unreachable(syscalls: dict[str, str], corpus: str) -> list[tuple[str, str]]:
    out = []
    for tag, name in syscalls.items():
        by_tag = f'b"{tag}"' in corpus
        by_hex = tag_hex(tag) in corpus or tag_hex(tag).replace("_", "") in corpus
        if not (by_tag or by_hex):
            out.append((name, tag))
    return sorted(out)


