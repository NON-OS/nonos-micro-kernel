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
"""Draft [desc] blocks for syscalls the ABI does not publish yet.

This writes a starting point, not an answer. Argument names and types come out
of the handler signature, which is reliable. Direction does not: a `u64` that
happens to be a user pointer is only an out parameter if the handler writes
through it, so this marks a pointer as `out` when the handler copies to user
and flags anything it could not decide.

Read every block it emits against the handler before publishing it. A wrong
`[desc]` block is worse than a missing one, because a foreign toolchain will
believe it and pass a buffer the kernel never fills.
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_syscall_abi import read_abi, read_kernel  # noqa: E402

# Two dispatch families, two spellings. The microkernel matches on SYS_
# constants, the router matches on SyscallNumber variants, and the crypto and
# admin calls only exist in the second.
DISPATCH = (Path("src/syscall/microkernel/dispatch"),
            Path("src/syscall/dispatch/router"))
HANDLERS = (Path("src/syscall/microkernel"), Path("src/syscall/dispatch"))
ARM = re.compile(r"(?:SYS_(\w+)|SyscallNumber::(\w+))\s*=>\s*(\w+)\(")
# `pub fn`, but also `pub(super) fn` and `pub(in path::to) fn`, which is how
# the admin and input handlers are declared.
SIG = re.compile(r"pub(?:\([^)]*\))?\s+fn (\w+)\s*\(([^)]*)\)\s*->\s*([\w:<> ]+)")


def dispatch_map(root: Path):
    """SYS_ constant and SyscallNumber variant, both -> handler name."""
    out = {}
    for base in DISPATCH:
        for path in (root / base).rglob("*.rs"):
            for sys_const, variant, handler in ARM.findall(path.read_text()):
                out["SYS_" + sys_const if sys_const else variant] = handler
    return out


def signatures(root: Path):
    """handler -> ([(name, type)], return type, source text)."""
    out = {}
    for base in HANDLERS:
        for path in (root / base).rglob("*.rs"):
            text = path.read_text()
            # A handler often validates and copies in sibling files, as
            # policy_push does, so the evidence for a direction is the module
            # rather than the one file.
            module = "\n".join(p.read_text() for p in sorted(path.parent.glob("*.rs")))
            for name, args, ret in SIG.findall(text):
                parsed = []
                for arg in (a.strip() for a in args.split(",") if a.strip()):
                    if ":" not in arg:
                        continue
                    an, at = arg.split(":", 1)
                    parsed.append((an.strip(), at.strip()))
                out[name] = (parsed, ret.strip(), module)
    return out


# The usercopy primitives, plus the thin `copy::` wrappers the crypto handlers
# use. A turbofish may sit between the name and the paren, as in
# `copy::read_array::<32>(ptr)`.
WRITES = ("write_user_bytes", "copy_to_user", "write_user_value",
          "validate_user_write", "copy::write", "copy::write_result")
READS = ("read_user_bytes", "copy_from_user", "read_user_value",
         "read_user_string", "validate_user_read", "copy::read_array",
         "copy::read_slice", "copy::read_vec",
         # a local reader in the hmac handler, one level above copy::read_vec
         "read_optional")
POINTER_NAME = re.compile(r"ptr|buf|addr|_out\b|^out|data|msg|name|path")


def direction(arg_name: str, arg_type: str, body: str):
    """Direction, and whether the handler actually showed us.

    Decided by which usercopy primitive the handler passes the argument to,
    since that is the only evidence in the source. A pointer the handler never
    hands to one of them is reported as undecided rather than guessed: a
    caller who believes a wrong `out` passes a buffer nobody fills.
    """
    # A length is a scalar however it is named. `out_len` and `name_len` sit
    # next to a pointer, they are not one.
    is_len = arg_name == "len" or arg_name.endswith("_len")
    pointerish = not is_len and ("*" in arg_type or POINTER_NAME.search(arg_name))
    if not pointerish:
        # A scalar cannot be an out parameter. Nothing is carried back through
        # a pid or a timeout, so no evidence is needed to call it in.
        return "in", True
    for fns, answer in ((WRITES, "out"), (READS, "in")):
        for fn in fns:
            pat = rf"{re.escape(fn)}(?:::<[^>]*>)?\(\s*{re.escape(arg_name)}\b"
            if re.search(pat, body):
                return answer, True
    return "in", False


def block(tag: str, handler: str, sig):
    args, ret, body = sig
    parts, unsure = [], []
    for name, ty in args:
        d, certain = direction(name, ty, body)
        if not certain:
            unsure.append(name)
        parts.append(f'{{name="{name}",type="{ty}",dir="{d}"}}')
    b = int.from_bytes(tag.encode(), "little")
    # SyscallResult is the in-kernel carrier. What crosses the boundary is the
    # i64 in its value field, which is what the published entries already say.
    wire = "i64" if ret in ("SyscallResult", "i64") else ret
    lines = [f"[desc.{tag}]",
             f"nr   = 0x{b:08X}",
             'caps = ["valid_token"]',
             f"args = [{','.join(parts)}]",
             f'ret  = {{type="{wire}"}}']
    if unsure:
        lines.insert(0, f"# CHECK {handler}: direction unverified for "
                        + ", ".join(unsure))
    return "\n".join(lines), bool(unsure)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    args = ap.parse_args()

    kernel = read_kernel(args.root)
    published, _ = read_abi(args.root)
    arms = dispatch_map(args.root)
    sigs = signatures(args.root)
    by_const = {c: h for c, h in arms.items()}

    drafted, undecided, flagged = [], [], 0
    for tag, names in sorted(kernel.items()):
        if tag in published:
            continue
        const = names.get("microkernel") or names.get("enum")
        handler = by_const.get(const) if const else None
        if handler and handler in sigs:
            text, unsure = block(tag, handler, sigs[handler])
            drafted.append(text)
            flagged += unsure
        else:
            undecided.append(f"# {tag}: no handler found via "
                             f"{const or 'the enum only'}, write by hand")

    print("\n\n".join(drafted))
    if undecided:
        print("\n".join(undecided), file=sys.stderr)
    print(f"{len(drafted)} drafted, {flagged} carry an unverified direction, "
          f"{len(undecided)} need writing by hand", file=sys.stderr)
    return 0


if __name__ == "__main__":
    main()
