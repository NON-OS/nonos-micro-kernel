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
"""Check that every published syscall is one a capsule can actually reach.

`abi/syscalls.toml` is what a libc or a foreign toolchain reads to learn how
to call this kernel. Publishing a number there is a promise, and the promise
is only kept if a capsule placing that number in `rax` arrives at the handler.

Reaching a handler takes five things, and missing any one of them fails the
call silently, at a different place each time:

  registry     `abi/registry/*.rs` turns the raw number into a `SyscallNumber`.
               Missing, and `from_u64` returns None and the entry point
               answers ENOSYS before any of the rest of this exists.
  enum         `numbers/defs.rs` is where the variant and its tag are declared.
  cap table    `contract/cap_table/*.rs` decides who may call it. The table is
               total by a trailing `unwrap_or(false)`, so a variant no family
               claims is refused for everyone, permanently and quietly.
  router       `dispatch/router/*.rs` picks the family that handles it. A
               variant no family matches falls to the router's default.
  handler      for the microkernel family, an arm in `microkernel/dispatch/`
               keyed on the `SYS_*` constant. Without it `route` returns -1.

The earlier version of this check read the enum and the microkernel constants
and asked only whether the tag appeared in one of them. A `SYS_*` constant on
its own dispatches nothing, because `route` is reached through the registry
and the enum, so three syscalls sat published, implemented, and unreachable,
with this check reporting agreement: the attestation document, and both
halves of the developer-root enrolment handshake.

So the direction that matters is: published, therefore reachable. The reverse
direction, a syscall the kernel dispatches and the ABI does not publish, is
listed rather than fatal unless --strict, because it is a capability nobody
outside the tree can name yet.
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import sections  # noqa: E402
import check_syscall_caps as caps  # noqa: E402

ABI = Path("abi/syscalls.toml")
ENUM = Path("src/syscall/numbers/defs.rs")
REGISTRY = Path("src/syscall/abi/registry")
CHECKS = Path("src/syscall/caps/checks")
CAP_TABLE = Path("src/syscall/contract/cap_table")
ROUTER = Path("src/syscall/dispatch/router")
MK_NUMBERS = Path("src/syscall/microkernel/numbers.rs")
MK_DISPATCH = Path("src/syscall/microkernel/dispatch")
MK_ROUTER = ROUTER / "microkernel_ops.rs"

ENUM_RE = re.compile(r"(\w+)\s*=\s*tag4\(b\"(\w{4})\"\)")
# Families spell the constructor `e` or `r`, so match the call shape and not
# whichever letter a file happens to have picked.
REGISTRY_RE = re.compile(r"\b[a-z]\(b\"(\w{4})\"\s*,\s*SyscallNumber::(\w+)")
MK_CONST_RE = re.compile(r"pub const (\w+)\s*:\s*u64\s*=\s*tag4\(b\"(\w{4})\"\)")
MK_ARM_RE = re.compile(r"^\s*(SYS_\w+)(?:\s*\|\s*SYS_\w+)*\s*=>", re.M)
IDENT_RE = re.compile(r"\b([A-Z]\w+)\b")


def tag4(tag):
    b = tag.encode()
    return b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24)


def read_text(root, path):
    return (root / path).read_text()


def read_tree(root, path):
    """Every .rs file under a directory, concatenated. Family split is not
    something this check should have an opinion about."""
    return "\n".join(p.read_text() for p in sorted((root / path).rglob("*.rs")))


def variants_in(text, known):
    """Variant names mentioned in a blob. `use SyscallNumber::*` means arms are
    written bare, so match identifiers against the enum rather than the path."""
    return {name for name in IDENT_RE.findall(text) if name in known}


def read_kernel(root):
    enum = {tag: name for name, tag in ENUM_RE.findall(read_text(root, ENUM))}
    registry = dict(REGISTRY_RE.findall(read_tree(root, REGISTRY)))
    mk_consts = {tag: name for name, tag in MK_CONST_RE.findall(read_text(root, MK_NUMBERS))}
    names = set(enum.values())
    return {
        "enum": enum,
        "registry": registry,
        "mk_consts": mk_consts,
        "capped": variants_in(read_tree(root, CAP_TABLE), names),
        "routed": variants_in(read_tree(root, ROUTER), names),
        "mk_routed": variants_in(read_text(root, MK_ROUTER), names),
        "mk_arms": {arm for text in [read_tree(root, MK_DISPATCH)]
                    for group in MK_ARM_RE.findall(text) for arm in group.split("|")},
    }


def tags_by_source(k):
    """tag -> {source: rust name}, for callers that only want to know where a
    number is declared. `gen_syscall_desc` drafts descriptions from this."""
    out = {}
    for side, table in (("enum", k["enum"]), ("microkernel", k["mk_consts"])):
        for tag, name in table.items():
            out.setdefault(tag, {})[side] = name
    return out


def read_abi(root):
    parsed = sections(read_text(root, ABI))
    numbers = {k: int(v, 16 if v.startswith("0x") else 10)
               for k, v in parsed.get("numbers", [])}
    described = {s.split(".", 1)[1] for s in parsed if s.startswith("desc.")}
    return numbers, described


def check_published(tag, value, k, described):
    """Every reason this published number would not reach a handler."""
    if value != tag4(tag):
        return [f"{tag} DISAGREES: tag4 gives 0x{tag4(tag):X}, abi says 0x{value:X}"]
    if tag not in described:
        return [f"{tag} is in [numbers] with no [desc.{tag}] block"]
    variant = k["registry"].get(tag)
    if variant is None:
        where = "implemented but not" if tag in k["mk_consts"] else "neither implemented nor"
        return [f"{tag} = 0x{value:X} is published and {where} registered: "
                f"from_u64 returns None, so every call answers ENOSYS"]
    if k["enum"].get(tag) != variant:
        return [f"{tag} is registered as {variant}, which the enum does not "
                f"declare with that tag"]
    bad = []
    if variant not in k["capped"]:
        bad.append(f"{tag} ({variant}) is in no cap-table family: the total "
                   f"table's unwrap_or(false) refuses it for every caller")
    if variant not in k["routed"]:
        bad.append(f"{tag} ({variant}) is matched by no router family")
    if variant in k["mk_routed"]:
        const = k["mk_consts"].get(tag)
        if const is None:
            bad.append(f"{tag} ({variant}) routes to the microkernel with no "
                       f"SYS_ constant to dispatch on")
        elif const not in k["mk_arms"]:
            bad.append(f"{tag} ({variant}) routes to the microkernel and {const} "
                       f"has no dispatch arm: route returns -1")
    return bad


def check_kernel_side(k):
    """A SYS_ constant whose tag no capsule can name is a handler nothing
    reaches, which is how the three unreachable syscalls looked from here."""
    return [f"{tag} ({name}) has a microkernel constant and no registry entry: "
            f"nothing can send this number"
            for tag, name in sorted(k["mk_consts"].items())
            if tag not in k["registry"]]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--strict", action="store_true",
                    help="fail when a dispatched syscall is unpublished, rather "
                         "than only listing it")
    args = ap.parse_args()

    try:
        k = read_kernel(args.root)
        published, described = read_abi(args.root)
    except FileNotFoundError as e:
        print(f"syscall-abi: {e}", file=sys.stderr)
        return 2
    if not k["enum"] or not k["registry"] or not published:
        print("syscall-abi: parsed an empty table, the file layout probably changed",
              file=sys.stderr)
        return 2

    fatal = check_kernel_side(k)
    for tag, value in sorted(published.items()):
        fatal += check_published(tag, value, k, described)
    fatal += caps.compare(k["registry"], caps.demanded(args.root / CAP_TABLE, args.root / CHECKS),
                          caps.published(dict(sections(read_text(args.root, ABI)))))

    unpublished = [f"  {tag} ({name})" for tag, name in sorted(k["enum"].items())
                   if tag not in published]
    if unpublished:
        print(f"syscall-abi: {len(unpublished)} dispatched syscalls are unpublished")
        print("\n".join(unpublished))
    if fatal:
        print("syscall-abi: published numbers that do not reach a handler",
              file=sys.stderr)
        for line in fatal:
            print(f"  {line}", file=sys.stderr)
        return 1
    if unpublished and args.strict:
        return 1
    print(f"syscall-abi: {len(published)} published syscalls reach a handler")
    return 0


if __name__ == "__main__":
    sys.exit(main())
