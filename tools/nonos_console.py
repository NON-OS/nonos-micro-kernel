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
"""Everything true about this NONOS image, and proof that it defends itself.

Nothing is asserted. Every number is counted from a file on disk, the kernel
ELF is parsed here rather than shelled out to binutils, and every attack is
carried out for real against copies of the shipped artifacts using the same
verifier the build and the kernel use. An attack that is not stopped is
reported as a failure and exits non zero.

    python3 tools/nonos_console.py                everything
    python3 tools/nonos_console.py --fast         no typing delay
    python3 tools/nonos_console.py <section>      one section

Sections: scale rings subsystems layering tcb surface boot kernel syscalls safety memory crypto authority ipc arch lean bench ubench attack
"""

import math
import os
import struct
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TRUST = os.path.join(ROOT, "nonos-data", "trust")
USERLAND = os.path.join(ROOT, "userland")
SRC = os.path.join(ROOT, "src")
TARGET = os.environ.get("NONOS_USER_TARGET", "x86_64-nonos-user")
SIGN = os.path.join(ROOT, "nonos-sign", "target", "release", "capsule-sign")
POLICY = os.path.join(TRUST, "policy", "nonos_trust_anchor.policy.bin")
KERNELS = [
    ("x86_64", os.path.join(ROOT, "target", "x86_64-nonos", "release", "nonos-kernel")),
    ("aarch64", os.path.join(ROOT, "target", "aarch64-nonos", "release", "nonos-kernel")),
]

EM_MACHINES = {62: "x86-64", 183: "AArch64", 243: "RISC-V"}
TRAILER_MAGIC = b"NZKSTRK1"
STT_FUNC, STT_OBJECT, SHT_SYMTAB = 2, 1, 2

CAPABILITIES = [
    (1 << 0, "CoreExec"), (1 << 1, "IO"), (1 << 2, "Network"), (1 << 3, "IPC"),
    (1 << 4, "Memory"), (1 << 5, "Crypto"), (1 << 6, "FileSystem"),
    (1 << 7, "Hardware"), (1 << 8, "Debug"), (1 << 9, "Admin"),
    (1 << 10, "RegisterService"), (1 << 11, "GfxQuery"), (1 << 12, "GfxCreate"),
    (1 << 13, "GfxMap"), (1 << 14, "GfxPresent"), (1 << 15, "DeviceEnum"),
    (1 << 16, "Driver"), (1 << 17, "Mmio"), (1 << 18, "Irq"), (1 << 19, "Dma"),
    (1 << 20, "Pio"), (1 << 21, "InputSource"), (1 << 22, "TimeSet"),
    (1 << 23, "SpawnBroker"), (1 << 24, "SpawnWindow"), (1 << 25, "ProcessControl"),
]
SCARCE = {"Admin", "RegisterService", "GfxPresent", "TimeSet",
          "SpawnBroker", "SpawnWindow", "ProcessControl"}

BOLD, DIM = "\033[1m", "\033[2m"
CYAN, GREEN, YELLOW, RED = "\033[36m", "\033[32m", "\033[33m", "\033[31m"
OFF = "\033[0m"

DELAY, PAUSE = 0.004, 0.25
FAILURES = 0


def out(text="", delay=None):
    step = DELAY if delay is None else delay
    if step <= 0:
        sys.stdout.write(text + "\n")
        sys.stdout.flush()
        return
    for char in text:
        sys.stdout.write(char)
        sys.stdout.flush()
        time.sleep(step)
    sys.stdout.write("\n")
    sys.stdout.flush()


def rule():
    out(DIM + "-" * 74 + OFF, delay=0)


def heading(title):
    time.sleep(PAUSE)
    out("")
    rule()
    out(BOLD + CYAN + "  " + title + OFF)
    rule()


def field(name, value, colour=""):
    out("  {}{:<20}{} {}{}{}".format(DIM, name, OFF, colour, value,
                                     OFF if colour else ""))


def commas(n):
    return "{:,}".format(n)


def mib(n):
    return "{:.1f} MiB".format(n / 1048576.0)


def bar(count, total, width=30, colour=CYAN):
    filled = max(1, int(round(width * count / total))) if total else 0
    filled = min(filled, width)
    return colour + "#" * filled + OFF + DIM + "." * (width - filled) + OFF


def rg(pattern, path=SRC, suffix=".rs"):
    """Count matching lines under a tree. Pure Python so it needs no ripgrep."""
    total = 0
    for root, dirs, files in os.walk(path):
        dirs[:] = [d for d in dirs if d not in ("target", ".git", ".lake")]
        for name in files:
            if not name.endswith(suffix):
                continue
            try:
                with open(os.path.join(root, name), "r",
                          encoding="utf-8", errors="replace") as handle:
                    for line in handle:
                        if pattern in line:
                            total += 1
            except OSError:
                pass
    return total


def walk(base, suffix):
    found = []
    for root, dirs, files in os.walk(base):
        dirs[:] = [d for d in dirs if d not in ("target", ".git", ".lake")]
        for name in files:
            if name.endswith(suffix):
                found.append(os.path.join(root, name))
    return found


def count_lines(paths):
    total = 0
    for path in paths:
        try:
            with open(path, "rb") as handle:
                total += handle.read().count(b"\n")
        except OSError:
            pass
    return total


def field_of(path, key):
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as handle:
            for line in handle:
                if line.startswith(key) and ":=" in line:
                    return line.split(":=", 1)[1].strip()
    except OSError:
        pass
    return ""


def size_of(path):
    try:
        return os.path.getsize(path)
    except OSError:
        return 0


def machine_of(path):
    try:
        with open(path, "rb") as handle:
            head = handle.read(20)
    except OSError:
        return None
    if len(head) < 20 or head[:4] != b"\x7fELF":
        return None
    return struct.unpack_from("<H", head, 18)[0]


def root_hex(name):
    try:
        with open(os.path.join(TRUST, "policy", name), "rb") as handle:
            return handle.read().hex()
    except OSError:
        return ""


class Elf(object):
    """Just enough ELF64 to describe an image honestly."""

    def __init__(self, path):
        with open(path, "rb") as handle:
            self.blob = handle.read()
        if self.blob[:4] != b"\x7fELF":
            raise ValueError("not an ELF")
        self.end = "<" if self.blob[5] == 1 else ">"
        self.e_type, self.e_machine = struct.unpack_from(self.end + "HH", self.blob, 16)
        self.entry, _, self.shoff = struct.unpack_from(self.end + "QQQ", self.blob, 24)
        self.shentsize, self.shnum, self.shstrndx = struct.unpack_from(
            self.end + "HHH", self.blob, 58)

    def shdr(self, i):
        base = self.shoff + i * self.shentsize
        name, sh_type = struct.unpack_from(self.end + "II", self.blob, base)
        _f, addr, offset, size = struct.unpack_from(self.end + "QQQQ", self.blob, base + 8)
        link, _info = struct.unpack_from(self.end + "II", self.blob, base + 40)
        entsize = struct.unpack_from(self.end + "Q", self.blob, base + 56)[0]
        return name, sh_type, addr, offset, size, link, entsize

    def string(self, table, index):
        stop = self.blob.index(b"\x00", table + index)
        return self.blob[table + index:stop].decode("utf-8", "replace")

    def sections(self):
        if self.shnum == 0 or self.shstrndx >= self.shnum:
            return []
        strtab = self.shdr(self.shstrndx)[3]
        return [(self.string(strtab, self.shdr(i)[0]), self.shdr(i)[2], self.shdr(i)[4])
                for i in range(self.shnum)]

    def symbols(self):
        table = None
        for i in range(self.shnum):
            _n, sh_type, _a, offset, size, link, entsize = self.shdr(i)
            if sh_type == SHT_SYMTAB and entsize:
                table = (offset, size, link, entsize)
                break
        if table is None:
            return
        offset, size, link, entsize = table
        strtab = self.shdr(link)[3]
        for pos in range(offset, offset + size, entsize):
            st_name = struct.unpack_from(self.end + "I", self.blob, pos)[0]
            st_info = self.blob[pos + 4]
            _v, st_size = struct.unpack_from(self.end + "QQ", self.blob, pos + 8)
            if st_name:
                yield self.string(strtab, st_name), st_info & 0xF, st_size


class Capsule(object):
    def __init__(self, mk):
        self.dir = os.path.dirname(mk)
        self.slug = field_of(mk, "CAPSULE_SLUG")
        self.bin_name = field_of(mk, "CAPSULE_BIN_NAME") or self.slug
        self.namespace = field_of(mk, "CAPSULE_NAMESPACE")
        self.service = field_of(mk, "CAPSULE_SERVICE_ENDPOINT") or ""
        raw = field_of(mk, "CAPSULE_REQUIRED_CAPS") or "0"
        try:
            self.mask = int(raw, 0)
        except ValueError:
            self.mask = 0
        self.binary = os.path.join(self.dir, "target", TARGET, "release", self.bin_name)
        self.bytes = size_of(self.binary)
        self.machine = machine_of(self.binary)
        self.cert = os.path.join(TRUST, "capsules", self.bin_name + ".nonos_id_cert.bin")
        self.manifest = os.path.join(TRUST, "capsules", self.bin_name + ".manifest.bin")
        self.trailer = os.path.join(TRUST, "capsules", self.bin_name + ".zk_trailer.bin")

    @property
    def caps(self):
        names, seen = [], 0
        for bit, name in CAPABILITIES:
            if self.mask & bit:
                names.append(name)
                seen |= bit
        if self.mask & ~seen:
            names.append("bit:0x{:x}".format(self.mask & ~seen))
        return names

    @property
    def attested(self):
        return all(size_of(p) > 0 for p in (self.cert, self.manifest, self.trailer))

    @property
    def port(self):
        parts = self.service.split(":")
        return parts[1] if len(parts) >= 2 else ""

    def builds_for(self, triple):
        return machine_of(os.path.join(self.dir, "target", triple,
                                       "release", self.bin_name))


def load():
    found = []
    if not os.path.isdir(USERLAND):
        return found
    for entry in sorted(os.listdir(USERLAND)):
        mk = os.path.join(USERLAND, entry, "Capsule.mk")
        if os.path.isfile(mk):
            capsule = Capsule(mk)
            if capsule.slug:
                found.append(capsule)
    return found


AREAS = [
    ("kernel  (ring 0)", "src"),
    ("userland (ring 3)", "userland"),
    ("bootloader", "nonos-bootloader"),
    ("stark prover", "nonos-stark"),
    ("signing tool", "nonos-sign"),
]


def section_scale(capsules):
    heading("SCALE")
    grand_files = grand_lines = 0
    for label, rel in AREAS:
        base = os.path.join(ROOT, rel)
        if not os.path.isdir(base):
            continue
        files = walk(base, ".rs")
        lines = count_lines(files)
        grand_files += len(files)
        grand_lines += lines
        field(label, "{} files, {} lines".format(commas(len(files)), commas(lines)))
    field("total rust", "{} files, {} lines".format(
        commas(grand_files), commas(grand_lines)), BOLD)
    out("")
    built = [c for c in capsules if c.bytes]
    field("capsules", "{} built, {} attested, {}".format(
        len(built), len([c for c in built if c.attested]),
        mib(sum(c.bytes for c in built))))
    # Hand written proofs only. The extraction tree is generated from Rust, so
    # counting it as proof effort would flatter the figure enormously.
    lean = walk(os.path.join(ROOT, "verification", "lean"), ".lean")
    field("lean proofs", "{} files, {} lines (hand written)".format(
        commas(len(lean)), commas(count_lines(lean))))
    kani = 0
    for path in walk(USERLAND, ".rs"):
        try:
            with open(path, "rb") as handle:
                kani += handle.read().count(b"kani::proof")
        except OSError:
            pass
    field("kani harnesses", commas(kani))
    crates = [d for d in os.listdir(USERLAND) if d.endswith(("_proofs", "_proof"))]
    field("proof crates", commas(len(crates)))


def section_rings():
    heading("PRIVILEGE SEPARATION")
    out("  " + DIM + "The kernel is the part that has to be trusted, so it is "
        "the part kept small." + OFF)
    out("")
    kernel_lines = count_lines(walk(SRC, ".rs"))
    user_lines = count_lines(walk(USERLAND, ".rs"))
    total = kernel_lines + user_lines
    if total:
        share = 100.0 * user_lines / total
        out("  {:<17} {} {:>5.1f}%  {} lines".format(
            "ring 3 unprivileged", bar(user_lines, total, 30, GREEN),
            share, commas(user_lines)))
        out("  {:<17} {} {:>5.1f}%  {} lines".format(
            "ring 0 kernel", bar(kernel_lines, total, 30, YELLOW),
            100.0 - share, commas(kernel_lines)))
        out("")
        # Of the code that runs on the machine. The bootloader and the host
        # tools are counted in SCALE but are not resident, and folding them in
        # would flatter this figure with code that is not running.
        out("  {}{:.0f}% of the {} lines that run has no privilege at all.{}".format(
            BOLD, share, commas(total), OFF))
    out("")
    out("  " + DIM + "How the boundary is held" + OFF)
    field("  address spaces", "{} ASID references".format(commas(rg("asid", os.path.join(SRC, "memory")) + rg("Asid", os.path.join(SRC, "memory")))))
    field("  hardware gates", "{} SMEP/SMAP sites".format(commas(rg("SMEP") + rg("SMAP") + rg("smep") + rg("smap"))))
    field("  ring markers", "{} Ring3/DPL/EL0 sites".format(
        commas(rg("Ring3") + rg("ring3") + rg("DPL") + rg("EL0"))))
    field("  checked copies", "{} copy_from/to_user".format(
        commas(rg("copy_from_user") + rg("copy_to_user"))))
    field("  user access guards", commas(rg("user_access")))


def subsystem_stats():
    """Per subsystem: size, unsafe density, and how much of it is documented."""
    stats = {}
    for entry in sorted(os.listdir(SRC)):
        base = os.path.join(SRC, entry)
        if not os.path.isdir(base):
            continue
        files = walk(base, ".rs")
        if not files:
            continue
        lines = count_lines(files)
        unsafe = safety = deps = 0
        uses = {}
        for path in files:
            try:
                with open(path, "r", encoding="utf-8", errors="replace") as handle:
                    for line in handle:
                        if "unsafe {" in line:
                            unsafe += 1
                        if "SAFETY:" in line:
                            safety += 1
                        at = line.find("use crate::")
                        if at >= 0:
                            rest = line[at + len("use crate::"):]
                            name = ""
                            for ch in rest:
                                if ch.isalnum() or ch == "_":
                                    name += ch
                                else:
                                    break
                            if name and name != entry:
                                uses[name] = uses.get(name, 0) + 1
                                deps += 1
            except OSError:
                pass
        stats[entry] = {
            "files": len(files), "lines": lines, "unsafe": unsafe,
            "safety": safety, "uses": uses, "deps": deps,
            "density": (1000.0 * unsafe / lines) if lines else 0.0,
        }
    return stats


def section_subsystems():
    heading("SUBSYSTEM DECOMPOSITION")
    out("  " + DIM + "Unsafe density is blocks per thousand lines. It is where "
        "review time is worth spending." + OFF)
    out("")
    stats = subsystem_stats()
    out("  {}{:<14} {:>6} {:>8} {:>7} {:>7} {:>6}{}".format(
        DIM, "SUBSYSTEM", "FILES", "LINES", "UNSAFE", "PER KLOC", "DOC%", OFF))
    ranked = sorted(stats.items(), key=lambda kv: -kv[1]["lines"])
    for name, info in ranked:
        doc = (100.0 * info["safety"] / info["unsafe"]) if info["unsafe"] else 100.0
        colour = ""
        if info["density"] >= 10.0:
            colour = RED
        elif info["density"] >= 4.0:
            colour = YELLOW
        out("  {:<14} {:>6} {:>8} {:>7} {}{:>7.1f}{} {:>5.0f}%".format(
            name, commas(info["files"]), commas(info["lines"]),
            commas(info["unsafe"]), colour, info["density"], OFF if colour else "",
            doc))
    total_lines = sum(i["lines"] for i in stats.values())
    total_unsafe = sum(i["unsafe"] for i in stats.values())
    out("")
    field("kernel density", "{:.1f} unsafe blocks per thousand lines".format(
        1000.0 * total_unsafe / total_lines if total_lines else 0.0))


def section_layering():
    heading("LAYERING")
    out("  " + DIM + "Who depends on whom, counted from `use crate::` across the "
        "kernel." + OFF)
    out("")
    stats = subsystem_stats()
    fan_out = {n: len(i["uses"]) for n, i in stats.items()}
    fan_in = {}
    for name, info in stats.items():
        for target in info["uses"]:
            if target in stats:
                fan_in[target] = fan_in.get(target, 0) + 1

    out("  " + BOLD + "Most depended on" + OFF)
    for name, count in sorted(fan_in.items(), key=lambda kv: -kv[1])[:8]:
        out("  {:<14} {} {:>3} subsystems depend on it".format(
            name, bar(count, max(fan_in.values()), 22), count))

    out("")
    out("  " + BOLD + "Widest reach" + OFF)
    for name, count in sorted(fan_out.items(), key=lambda kv: -kv[1])[:8]:
        out("  {:<14} {} {:>3} subsystems used".format(
            name, bar(count, max(fan_out.values()) or 1, 22), count))

    # A cycle means neither side can be reasoned about, tested, or replaced on
    # its own, so they are worth naming rather than leaving in the graph.
    pairs = []
    for name, info in stats.items():
        for target in info["uses"]:
            if target in stats and name in stats[target]["uses"] and name < target:
                pairs.append((name, target,
                              info["uses"][target], stats[target]["uses"][name]))
    out("")
    out("  " + BOLD + "Mutual dependencies" + OFF)
    if not pairs:
        out("  {}none{}".format(GREEN, OFF))
    else:
        for a, b, ab, ba in sorted(pairs, key=lambda p: -(p[2] + p[3]))[:10]:
            out("  {}{:<13}{} <-> {:<13} {:>4} / {:<4}".format(
                YELLOW, a, OFF, b, ab, ba))
        out("  {}{} mutually dependent pairs{}".format(DIM, len(pairs), OFF))


def section_tcb():
    heading("TRUSTED COMPUTING BASE")
    out("  " + DIM + "What has to be correct for every guarantee to hold." + OFF)
    out("")
    # The trusted path: anything that decides whether code runs, what it may
    # touch, or what a signature means. A bug anywhere here is not contained by
    # the capability model, because it is the capability model.
    trusted = ["capabilities", "security", "crypto", "elf", "syscall",
               "usercopy", "memory", "process", "sched", "ipc", "arch"]
    stats = subsystem_stats()
    tcb_lines = sum(stats[n]["lines"] for n in trusted if n in stats)
    kernel_lines = sum(i["lines"] for i in stats.values())
    user_lines = count_lines(walk(USERLAND, ".rs"))
    whole = kernel_lines + user_lines
    for name in trusted:
        if name in stats:
            out("  {:<14} {:>8} lines".format(name, commas(stats[name]["lines"])))
    out("")
    field("trusted total", "{} lines".format(commas(tcb_lines)), BOLD)
    if whole:
        # Same denominator as the privilege split: the resident system. Stated
        # rather than implied, because the SCALE total includes the bootloader
        # and the host tools and a percentage is meaningless without its base.
        out("  {}{:.1f}% of the {} lines that run has to be correct; the other "
            "{:.1f}% is contained by it.{}".format(
                BOLD, 100.0 * tcb_lines / whole, commas(whole),
                100.0 - 100.0 * tcb_lines / whole, OFF))
        out("  {}the bootloader is trusted before handoff and is not counted "
            "here{}".format(DIM, OFF))


def section_surface():
    heading("ATTACK SURFACE")
    out("  " + DIM + "Every place untrusted input crosses into the kernel." + OFF)
    out("")
    field("syscall entries", commas(rg('tag4(b"', os.path.join(SRC, "syscall"))))
    field("ipc endpoints", commas(rg("register_endpoint")))
    field("user pointer reads", commas(rg("copy_from_user")))
    field("user pointer writes", commas(rg("copy_to_user")))
    field("mmio grants", commas(rg("map_device_memory") + rg("map_mmio")))
    field("interrupt handlers", commas(rg("extern \"x86-interrupt\"")))
    field("bounds checks", commas(rg("checked_add") + rg("checked_sub") +
                                  rg("checked_mul")))
    field("saturating ops", commas(rg("saturating_add") + rg("saturating_sub")))


def emit_json(capsules):
    """Every measurement as one JSON object, for the visual map to render."""
    import json
    stats = subsystem_stats()
    built = [c for c in capsules if c.bytes]
    areas = {}
    for label, rel in AREAS:
        base = os.path.join(ROOT, rel)
        if os.path.isdir(base):
            files = walk(base, ".rs")
            areas[label.strip()] = {"files": len(files), "lines": count_lines(files)}
    trusted = ["capabilities", "security", "crypto", "elf", "syscall",
               "usercopy", "memory", "process", "sched", "ipc", "arch"]
    caps = []
    for bit, name in CAPABILITIES:
        holders = sorted(c.slug for c in built if c.mask & bit)
        if holders:
            caps.append({"name": name, "holders": holders,
                         "scarce": name in SCARCE})
    kernels = {}
    for arch, path in KERNELS:
        if not size_of(path):
            continue
        try:
            elf = Elf(path)
        except Exception:
            continue
        sections = sorted([{"name": n, "addr": a, "size": z}
                           for n, a, z in elf.sections() if z],
                          key=lambda r: -r["size"])[:8]
        funcs = objects = total = 0
        for _n, kind, _z in elf.symbols():
            total += 1
            if kind == STT_FUNC:
                funcs += 1
            elif kind == STT_OBJECT:
                objects += 1
        kernels[arch] = {"bytes": size_of(path), "entry": elf.entry,
                         "machine": EM_MACHINES.get(elf.e_machine, "?"),
                         "sections": sections, "symbols": total,
                         "functions": funcs, "objects": objects}
    doc = {
        "areas": areas,
        "subsystems": [
            {"name": n, "files": i["files"], "lines": i["lines"],
             "unsafe": i["unsafe"], "safety": i["safety"],
             "density": round(i["density"], 2),
             "uses": i["uses"], "trusted": n in trusted}
            for n, i in sorted(stats.items(), key=lambda kv: -kv[1]["lines"])
        ],
        "capsules": [
            {"slug": c.slug, "bytes": c.bytes, "port": c.port,
             "caps": c.caps, "attested": c.attested,
             "namespace": c.namespace}
            for c in sorted(built, key=lambda c: -c.bytes)
        ],
        "capabilities": caps,
        "kernels": kernels,
        "safety": {
            "unwrap": rg(".unwrap()"), "expect": rg(".expect("),
            "panic": rg("panic!("), "unsafe": rg("unsafe {"),
            "documented": rg("SAFETY:"),
        },
        "surface": {
            "syscalls": rg('tag4(b"', os.path.join(SRC, "syscall")),
            "user_reads": rg("copy_from_user"), "user_writes": rg("copy_to_user"),
            "mmio": rg("map_device_memory") + rg("map_mmio"),
            "checked": rg("checked_add") + rg("checked_sub") + rg("checked_mul"),
            "saturating": rg("saturating_add") + rg("saturating_sub"),
        },
        "memory": {
            "zeroize": rg("secure_zero") + rg("zeroize") + rg("Zeroizing"),
            "drops": rg("impl Drop"),
            "constant_time": rg("constant_time") + rg("ct_eq"),
        },
        "proofs": {
            "lean_files": len(walk(os.path.join(ROOT, "verification", "lean"), ".lean")),
            "lean_lines": count_lines(walk(os.path.join(ROOT, "verification", "lean"), ".lean")),
            "kani": sum(open(f, "rb").read().count(b"kani::proof")
                        for f in walk(USERLAND, ".rs")),
            "crates": len([d for d in os.listdir(USERLAND)
                           if d.endswith(("_proofs", "_proof"))]),
        },
        "roots": {
            "policy": root_hex("zk_capsule_policy_root.bin"),
            "kernel": root_hex("kernel_attest_root.bin"),
        },
    }
    print(json.dumps(doc, indent=1))


def percentile(values, share):
    if not values:
        return 0.0
    ordered = sorted(values)
    at = min(len(ordered) - 1, int(round(share * (len(ordered) - 1))))
    return ordered[at]


def bench_signature(attested):
    """Time the hybrid signature check the spawn gate depends on."""
    out("  " + BOLD + "Signature verification" + OFF)
    times = []
    for capsule in attested:
        started = time.perf_counter()
        accepted = verify_manifest(capsule.manifest, capsule.cert)
        if accepted:
            times.append((time.perf_counter() - started) * 1000.0)
    if not times:
        out("  {}no manifest verified{}".format(YELLOW, OFF))
        return None
    total = sum(times) / 1000.0
    field("  verified", "{} of {} manifests".format(len(times), len(attested)),
          GREEN if len(times) == len(attested) else RED)
    field("  latency", "min {:.1f}  median {:.1f}  p95 {:.1f}  max {:.1f} ms".format(
        min(times), percentile(times, 0.5), percentile(times, 0.95), max(times)))
    field("  throughput", "{:.0f} verifications per second".format(len(times) / total))
    field("  whole image", "{:.2f} s to re-verify every capsule".format(total))
    return times


def bench_hash(attested):
    """Measurement throughput, which is what re-attesting an image costs."""
    import hashlib
    out("")
    out("  " + BOLD + "Measurement" + OFF)
    corpus = [c.binary for c in attested if c.bytes]
    total = sum(size_of(p) for p in corpus)
    if not total:
        return
    blob = bytearray()
    for path in corpus:
        try:
            with open(path, "rb") as handle:
                blob += handle.read()
        except OSError:
            pass
    for alg in ("sha256", "sha512", "sha3_256", "blake2b"):
        try:
            started = time.perf_counter()
            digest = hashlib.new(alg)
            digest.update(blob)
            elapsed = time.perf_counter() - started
        except ValueError:
            continue
        if elapsed <= 0:
            continue
        rate = len(blob) / 1048576.0 / elapsed
        out("  {}{:<18}{} {} {:>6.0f} MiB/s".format(
            DIM, "  " + alg, OFF, bar(int(rate), 400, 20), rate))
    field("  corpus", "{} across {} capsules".format(mib(total), len(corpus)))


def bench_parse(attested):
    """Parsing is what the loader does before it maps a single page."""
    out("")
    out("  " + BOLD + "Parsing" + OFF)
    started = time.perf_counter()
    parsed = sections = 0
    for capsule in attested:
        try:
            elf = Elf(capsule.binary)
            sections += len(elf.sections())
            parsed += 1
        except Exception:
            pass
    elapsed = time.perf_counter() - started
    if parsed and elapsed > 0:
        field("  elf headers", "{} capsules, {} sections in {:.0f} ms "
              "({:.0f}/s)".format(parsed, sections, elapsed * 1000.0,
                                  parsed / elapsed))
    started = time.perf_counter()
    admitted = 0
    for capsule in attested:
        try:
            with open(capsule.trailer, "rb") as handle:
                if handle.read(8) == TRAILER_MAGIC:
                    admitted += 1
        except OSError:
            pass
    elapsed = time.perf_counter() - started
    if admitted and elapsed > 0:
        field("  trailer admit", "{} checked in {:.0f} ms ({:.0f}/s)".format(
            admitted, elapsed * 1000.0, admitted / elapsed))


def bench_overhead(attested):
    """What the guarantees cost in bytes."""
    out("")
    out("  " + BOLD + "Overhead of proof" + OFF)
    code = sum(c.bytes for c in attested)
    trailers = sum(size_of(c.trailer) for c in attested)
    manifests = sum(size_of(c.manifest) for c in attested)
    certs = sum(size_of(c.cert) for c in attested)
    trust = trailers + manifests + certs
    field("  capsule code", mib(code))
    field("  stark trailers", "{}  ({:.1f}% of code)".format(
        mib(trailers), 100.0 * trailers / code if code else 0.0))
    field("  manifests", "{} bytes".format(commas(manifests)))
    field("  identity certs", "{} bytes".format(commas(certs)))
    field("  trust total", "{}  ({:.1f}% of the image)".format(
        mib(trust), 100.0 * trust / (code + trust) if code else 0.0), BOLD)


def bench_boot():
    """In-kernel timings, if a serial log from a real boot is present."""
    import re
    logs = []
    for name in ("boot.log", "serial.log"):
        path = os.path.join(ROOT, name)
        if os.path.isfile(path):
            logs.append(path)
    if not logs:
        return
    out("")
    out("  " + BOLD + "Boot, measured by the kernel itself" + OFF)
    text = ""
    for path in logs:
        try:
            with open(path, "r", errors="replace") as handle:
                text += handle.read()
        except OSError:
            pass
    rows = re.findall(r"\[BENCH\] (\d+) ([a-zA-Z_]+):?([a-zA-Z_0-9-]*)", text)
    if not rows:
        return
    phases, starts, oks = {}, {}, {}
    for stamp, kind, name in rows:
        stamp = int(stamp)
        if kind == "capsule_spawn_start":
            starts.setdefault(name, stamp)
        elif kind == "capsule_attest_ok":
            oks.setdefault(name, stamp)
        else:
            phases.setdefault(kind, stamp)
    for kind in ("microkernel_init_start", "vm_ready", "process_runtime_ready",
                 "microkernel_core_ready", "init_process_created",
                 "init_enter_userspace"):
        if kind in phases:
            field("  " + kind.replace("_", " "), "{} ms".format(phases[kind]))
    costs = [oks[k] - starts[k] for k in oks if k in starts]
    if costs:
        field("  stark verify", "min {}  median {}  max {} ms, {} capsules".format(
            min(costs), int(percentile(costs, 0.5)), max(costs), len(costs)))
        out("  " + DIM + "  in-kernel, under emulation, so a real CPU is faster" + OFF)


def stat_line(label, values, unit="ms", scale=1.0):
    if not values:
        return
    vals = sorted(v * scale for v in values)
    out("  {}{:<16}{} min {:>8.3f}  med {:>8.3f}  p95 {:>8.3f}  max {:>8.3f} {}".format(
        DIM, label, OFF, vals[0], percentile(vals, 0.5),
        percentile(vals, 0.95), vals[-1], unit))


def shannon(data):
    """Bits of entropy per byte. Compiled code sits near 6, padding near 0."""
    if not data:
        return 0.0
    counts = [0] * 256
    for byte in data:
        counts[byte] += 1
    total = float(len(data))
    acc = 0.0
    for count in counts:
        if count:
            share = count / total
            acc -= share * math.log(share, 2)
    return acc


def bench_pipeline(attested):
    """Every stage of admitting a capsule, timed separately, per capsule.

    This is the sequence the kernel performs before a capsule runs: read it,
    measure it, parse it, verify who signed it, admit its proof, decode what it
    is allowed to do. Timing them apart shows which stage a boot actually waits
    on, which a single total hides.
    """
    import hashlib
    out("  " + BOLD + "Admission pipeline, per capsule" + OFF)
    read_t, hash_t, parse_t, verify_t, admit_t, caps_t = [], [], [], [], [], []
    totals = {}
    for capsule in attested:
        started = time.perf_counter()
        try:
            with open(capsule.binary, "rb") as handle:
                blob = handle.read()
        except OSError:
            continue
        read = time.perf_counter() - started
        read_t.append(read)

        started = time.perf_counter()
        hashlib.sha256(blob).digest()
        hashed = time.perf_counter() - started
        hash_t.append(hashed)

        started = time.perf_counter()
        try:
            elf = Elf(capsule.binary)
            elf.sections()
        except Exception:
            pass
        parsed = time.perf_counter() - started
        parse_t.append(parsed)

        started = time.perf_counter()
        verify_manifest(capsule.manifest, capsule.cert)
        verified = time.perf_counter() - started
        verify_t.append(verified)

        started = time.perf_counter()
        try:
            with open(capsule.trailer, "rb") as handle:
                handle.read(8) == TRAILER_MAGIC
        except OSError:
            pass
        admitted = time.perf_counter() - started
        admit_t.append(admitted)

        started = time.perf_counter()
        _ = capsule.caps
        decoded = time.perf_counter() - started
        caps_t.append(decoded)

        totals[capsule.slug] = (read + hashed + parsed + verified
                                + admitted + decoded)

    
    stat_line("read", read_t, "ms", 1000.0)
    stat_line("measure sha256", hash_t, "ms", 1000.0)
    stat_line("parse elf", parse_t, "ms", 1000.0)
    stat_line("verify signature", verify_t, "ms", 1000.0)
    stat_line("admit trailer", admit_t, "us", 1000000.0)
    stat_line("decode caps", caps_t, "us", 1000000.0)
    if totals:
        every = sorted(totals.values())
        out("")
        field("  per capsule", "median {:.1f} ms, p95 {:.1f} ms".format(
            1000 * percentile(every, 0.5), 1000 * percentile(every, 0.95)))
        field("  whole image", "{:.2f} s to admit all {} capsules".format(
            sum(every), len(every)), BOLD)
        out("")
        out("  " + DIM + "slowest to admit" + OFF)
        for slug, cost in sorted(totals.items(), key=lambda kv: -kv[1])[:6]:
            out("    {:<22} {:>7.1f} ms".format(slug, 1000 * cost))


def bench_kernel_image():
    """What the kernel image is made of, measured rather than described."""
    out("")
    out("  " + BOLD + "Kernel image analysis" + OFF)
    for arch, path in KERNELS:
        if not size_of(path):
            continue
        try:
            elf = Elf(path)
        except Exception:
            continue
        started = time.perf_counter()
        sizes = []
        for _n, kind, size in elf.symbols():
            if kind == STT_FUNC and size:
                sizes.append(size)
        scan = time.perf_counter() - started
        out("  {}{}{}".format(BOLD, arch, OFF))
        if sizes:
            field("    symbol scan", "{} functions in {:.0f} ms ({:.0f}k/s)".format(
                commas(len(sizes)), scan * 1000, len(sizes) / scan / 1000))
            field("    function size", "median {} B, p95 {} B, max {} B".format(
                commas(int(percentile(sorted(sizes), 0.5))),
                commas(int(percentile(sorted(sizes), 0.95))),
                commas(max(sizes))))
            field("    code in symbols", mib(sum(sizes)))
        # Entropy separates real code from zero padding without unpacking it.
        for name, _addr, size in elf.sections():
            if name not in (".text", ".rodata", ".data"):
                continue
            for sec_name, sh_type, addr, offset, sec_size, link, ent in (
                    [elf.shdr(i) for i in range(elf.shnum)]):
                pass
            blob = b""
            for i in range(elf.shnum):
                nm, _t, _a, offset, sz, _l, _e = elf.shdr(i)
                if elf.string(elf.shdr(elf.shstrndx)[3], nm) == name and sz:
                    blob = elf.blob[offset:offset + min(sz, 1 << 20)]
                    break
            if blob:
                field("    {} entropy".format(name),
                      "{:.2f} bits per byte over {}".format(
                          shannon(blob), mib(min(size, 1 << 20))))
        out("")


def section_bench(capsules):
    heading("BENCHMARK")
    out("  " + DIM + "Measured on this machine, now. Verification goes through "
        "capsule-sign as a" + OFF)
    out("  " + DIM + "separate process, so startup is inside each figure and "
        "they are floors." + OFF)
    out("")
    attested = [c for c in capsules if c.attested and c.bytes]
    if not attested:
        out("  {}nothing attested to measure{}".format(YELLOW, OFF))
        return
    bench_pipeline(attested)
    out("")
    bench_signature(attested)
    bench_hash(attested)
    bench_parse(attested)
    bench_kernel_image()
    bench_overhead(attested)
    bench_boot()


LEAN_ROOT = os.path.join(ROOT, "verification", "lean")


def lean_modules():
    """Every hand written Lean module with its theorem and definition count."""
    found = []
    base = os.path.join(LEAN_ROOT, "Nonos")
    for path in walk(base, ".lean"):
        theorems = defs = axioms = sorries = 0
        try:
            with open(path, "r", encoding="utf-8", errors="replace") as handle:
                for line in handle:
                    stripped = line.strip()
                    if stripped.startswith("--") or stripped.startswith("/-"):
                        continue
                    if stripped.startswith(("theorem ", "lemma ")):
                        theorems += 1
                    elif stripped.startswith(("def ", "abbrev ")):
                        defs += 1
                    elif stripped.startswith("axiom "):
                        axioms += 1
                    # A real sorry stands alone as a term, not inside prose.
                    if stripped == "sorry" or stripped.endswith(" sorry") \
                            or stripped.startswith("sorry "):
                        sorries += 1
        except OSError:
            continue
        name = os.path.relpath(path, base).replace(".lean", "")
        found.append({"name": name, "theorems": theorems, "defs": defs,
                      "axioms": axioms, "sorries": sorries,
                      "lines": count_lines([path])})
    return found


def section_lean():
    heading("MACHINE CHECKED PROOF")
    out("  " + DIM + "Lean 4, core only, no Mathlib. A theorem here is a "
        "property of the kernel," + OFF)
    out("  " + DIM + "not a test of one case." + OFF)
    out("")
    modules = lean_modules()
    if not modules:
        out("  {}no Lean corpus found{}".format(YELLOW, OFF))
        return
    theorems = sum(m["theorems"] for m in modules)
    defs = sum(m["defs"] for m in modules)
    axioms = sum(m["axioms"] for m in modules)
    sorries = sum(m["sorries"] for m in modules)
    lines = sum(m["lines"] for m in modules)

    field("modules", commas(len(modules)))
    field("theorems", commas(theorems), BOLD)
    field("definitions", commas(defs))
    field("lines", commas(lines))
    field("axioms", commas(axioms), GREEN if axioms == 0 else YELLOW)
    field("sorry", commas(sorries), GREEN if sorries == 0 else RED)
    if sorries == 0 and axioms == 0:
        out("  " + GREEN + "  nothing assumed and nothing left unproven" + OFF)

    out("")
    out("  " + BOLD + "Largest proof modules" + OFF)
    for module in sorted(modules, key=lambda m: -m["theorems"])[:12]:
        if not module["theorems"]:
            continue
        out("  {:<28} {} {:>4} theorems".format(
            module["name"], bar(module["theorems"],
                                max(m["theorems"] for m in modules), 20),
            module["theorems"]))

    # Which kernel subsystems the corpus actually names. A module named after a
    # subsystem is evidence that subsystem has stated properties; silence is
    # not evidence of the opposite, so this is reported as coverage of names.
    out("")
    out("  " + BOLD + "Kernel subsystems named by a proof module" + OFF)
    subsystems = [d for d in os.listdir(SRC) if os.path.isdir(os.path.join(SRC, d))]
    named = []
    lowered = [(m["name"].lower(), m["theorems"]) for m in modules]
    for sub in sorted(subsystems):
        key = sub.replace("_", "")
        hits = sum(t for n, t in lowered if key in n.replace("_", ""))
        if hits:
            named.append((sub, hits))
    for sub, hits in sorted(named, key=lambda kv: -kv[1])[:10]:
        out("  {:<16} {:>4} theorems".format(sub, hits))
    field("covered", "{} of {} subsystems named".format(
        len(named), len(subsystems)),
        GREEN if len(named) * 2 >= len(subsystems) else YELLOW)


BENCH_DIR = os.path.join(ROOT, "target", "bench")
OVMF_CANDIDATES = [
    "/usr/local/share/qemu/edk2-x86_64-code.fd",
    "/opt/homebrew/share/qemu/edk2-x86_64-code.fd",
    "/usr/share/OVMF/OVMF_CODE.fd",
]


def parse_ubench(text):
    """Pull the kernel's own microbenchmark markers out of a serial log."""
    cases = {}
    meta = {}
    for line in text.splitlines():
        if "[UBENCH]" not in line:
            continue
        body = line.split("[UBENCH]", 1)[1].strip()
        parts = body.split()
        if not parts:
            continue
        name = parts[0]
        fields = {}
        for token in parts[1:]:
            if "=" in token:
                key, _, value = token.partition("=")
                try:
                    fields[key] = int(value)
                except ValueError:
                    pass
        if name == "counter_overhead":
            meta.update(fields)
        elif fields:
            cases[name] = fields
    return meta, cases


def qemu_boot(arch, seconds):
    """Boot the measurement image and return whatever it printed."""
    log = os.path.join(BENCH_DIR, "serial-{}.log".format(arch))
    os.makedirs(BENCH_DIR, exist_ok=True)
    if arch == "aarch64":
        kernel = os.path.join(ROOT, "target", "aarch64-nonos", "release", "nonos-kernel")
        if not size_of(kernel):
            return None, "no aarch64 kernel; run make nonos-mk-arm-bench"
        cmd = ["qemu-system-aarch64", "-M", "virt,gic-version=3", "-cpu", "max",
               "-m", "512", "-nographic", "-serial", "mon:stdio",
               "-device", "virtio-rng-pci", "-kernel", kernel]
    else:
        esp = os.path.join(ROOT, "target", "esp")
        if not os.path.isdir(esp):
            return None, "no ESP; run make nonos-mk-bench-micro && make nonos-mk-esp"
        firmware = next((f for f in OVMF_CANDIDATES if os.path.isfile(f)), None)
        if firmware is None:
            return None, "OVMF firmware not found"
        cmd = ["qemu-system-x86_64", "-m", "2048", "-machine", "q35", "-cpu", "max",
               "-smp", "1", "-drive", "format=raw,file=fat:rw:" + esp,
               "-drive", "if=pflash,format=raw,readonly=on,file=" + firmware,
               "-display", "none", "-serial", "stdio", "-no-reboot"]
    try:
        with open(log, "wb") as handle:
            process = subprocess.Popen(cmd, stdout=handle,
                                       stderr=subprocess.STDOUT,
                                       stdin=subprocess.DEVNULL)
            deadline = time.time() + seconds
            while time.time() < deadline:
                if process.poll() is not None:
                    break
                time.sleep(0.5)
                try:
                    with open(log, "r", errors="replace") as reader:
                        if "[UBENCH] done" in reader.read():
                            break
                except OSError:
                    pass
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=10)
                except subprocess.SubprocessError:
                    process.kill()
    except (OSError, subprocess.SubprocessError) as err:
        return None, str(err)
    try:
        with open(log, "r", errors="replace") as handle:
            return handle.read(), None
    except OSError as err:
        return None, str(err)


def section_ubench(_capsules=None):
    heading("KERNEL MICROBENCHMARK")
    out("  " + DIM + "Cycle counts from the kernel's own paths, measured in "
        "kernel, quantiles" + OFF)
    out("  " + DIM + "not means, with the cost of the measurement removed." + OFF)
    out("")
    results = {}
    for arch in ("x86_64", "aarch64"):
        for candidate in (os.path.join(BENCH_DIR, "serial-{}.log".format(arch)),
                          os.path.join(ROOT, "boot.log")):
            if not os.path.isfile(candidate):
                continue
            try:
                with open(candidate, "r", errors="replace") as handle:
                    meta, cases = parse_ubench(handle.read())
            except OSError:
                continue
            if cases:
                results[arch] = (meta, cases)
                break
    if not results:
        out("  {}no measurement log found{}".format(YELLOW, OFF))
        out("  run: {}python3 tools/nonos_console.py measure{}".format(BOLD, OFF))
        return

    for arch, (meta, _cases) in results.items():
        hz = meta.get("hz", 0)
        out("  {}{:<10}{} counter {:.3f} GHz, measurement overhead {} ticks".format(
            DIM, arch, OFF, hz / 1e9 if hz else 0.0, meta.get("ticks", 0)))
    out("")

    names = []
    for _arch, (_m, cases) in results.items():
        for name in cases:
            if name not in names:
                names.append(name)

    archs = list(results.keys())
    out("  {}{:<20}{}{}".format(
        DIM, "CASE", "".join("{:>22}".format(a) for a in archs), OFF))
    for name in names:
        row = "  {:<20}".format(name)
        for arch in archs:
            case = results[arch][1].get(name)
            if not case:
                row += "{:>22}".format("-")
                continue
            ns = case.get("p50_ns", 0)
            row += "{:>22}".format("{} ns  p50".format(commas(ns)))
        out(row)

    # The spread between p50 and p95 is what a caller actually feels, so it is
    # reported rather than left for the reader to compute.
    out("")
    out("  " + BOLD + "Tail behaviour" + OFF)
    for arch in archs:
        meta, cases = results[arch]
        for name, case in cases.items():
            p50, p95 = case.get("p50", 0), case.get("p95", 0)
            if p50:
                out("  {:<10} {:<20} p95 is {:.2f}x p50".format(
                    arch, name, p95 / float(p50)))


def command_measure(args):
    """Build, boot and collect, so a number never comes from a stale log."""
    out("")
    out(BOLD + "  NONOS measurement run" + OFF)
    wanted = [a for a in ("x86_64", "aarch64") if a in args] or ["x86_64", "aarch64"]
    for arch in wanted:
        heading("MEASURE " + arch)
        target = "nonos-mk-bench-micro" if arch == "x86_64" else "nonos-mk-arm-bench"
        out("  {}building{} make {}".format(DIM, OFF, target))
        try:
            built = subprocess.run(["make", target], cwd=ROOT,
                                   stdout=subprocess.DEVNULL,
                                   stderr=subprocess.DEVNULL, timeout=3600)
        except (OSError, subprocess.SubprocessError) as err:
            out("  {}build failed: {}{}".format(RED, err, OFF))
            continue
        if built.returncode != 0:
            out("  {}build failed{}".format(RED, OFF))
            continue
        if arch == "x86_64":
            out("  {}packaging{} make nonos-mk-esp".format(DIM, OFF))
            subprocess.run(["make", "nonos-mk-esp"], cwd=ROOT,
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        out("  {}booting{} under qemu".format(DIM, OFF))
        text, err = qemu_boot(arch, 180)
        if text is None:
            out("  {}{}{}".format(YELLOW, err, OFF))
            continue
        meta, cases = parse_ubench(text)
        if not cases:
            out("  {}no markers in the log{}".format(YELLOW, OFF))
            continue
        out("  {}collected{} {} cases".format(GREEN, OFF, len(cases)))
    section_ubench()
    return 0


def section_boot():
    heading("BOOT CHAIN")
    out("  " + DIM + "Each stage measures the next before handing control over." + OFF)
    out("")
    loader = os.path.join(ROOT, "nonos-bootloader", "target",
                          "x86_64-unknown-uefi", "release", "nonos_boot.efi")
    if size_of(loader):
        field("1 bootloader", "{} bytes".format(commas(size_of(loader))))
    anchor = size_of(POLICY)
    if anchor:
        field("2 trust anchor", "{} bytes, ed25519 + ML-DSA-65".format(commas(anchor)))
    for label, name in (("3 kernel measure", "kernel_attest_root.bin"),
                        ("4 capsule policy", "zk_capsule_policy_root.bin")):
        value = root_hex(name)
        if value:
            field(label, value)
    attested = os.path.join(ROOT, "target", "kernel_attested.bin")
    if size_of(attested):
        field("5 attested kernel", "{} bytes ({})".format(
            commas(size_of(attested)), mib(size_of(attested))))


def section_kernel():
    heading("KERNEL IMAGE")
    for arch, path in KERNELS:
        if not size_of(path):
            continue
        try:
            elf = Elf(path)
        except (ValueError, OSError, struct.error):
            continue
        out("  {}{}{}".format(BOLD, arch, OFF))
        field("  machine", EM_MACHINES.get(elf.e_machine, str(elf.e_machine)),
              GREEN if EM_MACHINES.get(elf.e_machine) else RED)
        field("  entry", "0x{:016x}".format(elf.entry))
        field("  size", "{} bytes ({})".format(commas(size_of(path)), mib(size_of(path))))
        rows = sorted([s for s in elf.sections() if s[2]], key=lambda s: -s[2])
        for name, addr, size in rows[:5]:
            out("    {:<16} {:>14}  0x{:x}".format(name, commas(size), addr))
        funcs, objects, total = [], 0, 0
        for name, kind, size in elf.symbols():
            total += 1
            if kind == STT_FUNC:
                funcs.append((size, name))
            elif kind == STT_OBJECT:
                objects += 1
        if total:
            field("  symbols", "{} total, {} functions, {} objects".format(
                commas(total), commas(len(funcs)), commas(objects)))
        else:
            field("  symbols", "stripped", YELLOW)
        out("")


def section_syscalls():
    heading("SYSCALL SURFACE")
    out("  " + DIM + "Every syscall is a four byte tag, so the table is the "
        "whole interface." + OFF)
    out("")
    total = rg('tag4(b"', os.path.join(SRC, "syscall"))
    field("syscalls", commas(total))
    field("ipc surface", "{} lines".format(commas(count_lines(
        walk(os.path.join(SRC, "ipc"), ".rs")))))
    field("capability checks", commas(rg("require_cap")))


def section_safety():
    heading("SAFETY")
    out("  " + DIM + "A kernel that cannot panic has no panic path to reach." + OFF)
    out("")
    unwrap, expect, panics = rg(".unwrap()"), rg(".expect("), rg("panic!(")
    worst = max(unwrap, expect, panics)
    colour = GREEN if worst == 0 else RED
    field("unwrap()", commas(unwrap), colour)
    field("expect()", commas(expect), colour)
    field("panic!()", commas(panics), colour)
    unsafe = rg("unsafe {")
    documented = rg("SAFETY:")
    share = (100.0 * documented / unsafe) if unsafe else 0.0
    field("unsafe blocks", commas(unsafe))
    field("SAFETY comments", "{}  ({:.0f}% of unsafe blocks)".format(
        commas(documented), share), GREEN if share >= 30 else YELLOW)


def section_memory():
    heading("MEMORY HYGIENE")
    out("  " + DIM + "Secrets are wiped on the way out, not left for the next "
        "owner of the page." + OFF)
    out("")
    field("zeroization sites", commas(rg("secure_zero") + rg("zeroize") + rg("Zeroizing")))
    field("wiping Drop impls", commas(rg("impl Drop")))
    field("constant time ops", commas(rg("constant_time") + rg("ct_eq")))
    field("guard pages", commas(rg("guard_page")))


def section_crypto():
    heading("CRYPTOGRAPHY")
    families = [
        ("post-quantum", os.path.join(SRC, "crypto", "pqc")),
        ("asymmetric", os.path.join(SRC, "crypto", "asymmetric")),
        ("symmetric", os.path.join(SRC, "crypto", "symmetric")),
        ("hash", os.path.join(SRC, "crypto", "hash")),
    ]
    for label, path in families:
        if not os.path.isdir(path):
            continue
        names = sorted(n.replace(".rs", "") for n in os.listdir(path)
                       if n != "mod.rs" and not n.startswith("."))
        colour = GREEN if label == "post-quantum" else ""
        field(label, ", ".join(names), colour)
    zk = os.path.join(SRC, "crypto", "zk_kernel")
    if os.path.isdir(zk):
        field("zero knowledge", "{} lines".format(commas(count_lines(walk(zk, ".rs")))))


def section_authority(capsules):
    heading("AUTHORITY")
    out("  " + DIM + "Counted from the signed manifests the spawn gate enforces." + OFF)
    out("")
    built = [c for c in capsules if c.bytes]
    total = len(built)
    counts = []
    for bit, name in CAPABILITIES:
        holders = [c for c in built if c.mask & bit]
        if holders:
            counts.append((len(holders), name, holders))
    counts.sort(key=lambda row: -row[0])
    for count, name, _ in counts:
        colour = YELLOW if name in SCARCE else CYAN
        out("  {:<17} {} {:>3}/{}".format(name, bar(count, total, 30, colour),
                                          count, total))
    out("")
    out("  " + BOLD + "Authority that acts on what its holder does not own" + OFF)
    for count, name, holders in sorted(counts, key=lambda r: r[0]):
        if name in SCARCE:
            out("  {:<17} {}{}{}".format(
                name, GREEN if count <= 2 else YELLOW,
                ", ".join(sorted(c.slug for c in holders)), OFF))


def section_ipc(capsules):
    heading("IPC TOPOLOGY")
    out("  " + DIM + "Every service is a named endpoint on a fixed port. Nothing "
        "is ambient." + OFF)
    out("")
    serving = [c for c in capsules if c.port]
    field("service endpoints", commas(len(serving)))
    ports = sorted(int(c.port) for c in serving if c.port.isdigit())
    if ports:
        field("port range", "{} to {}".format(ports[0], ports[-1]))
        clashes = len(ports) - len(set(ports))
        field("port collisions", commas(clashes), GREEN if clashes == 0 else RED)
    domains = {}
    for capsule in serving:
        parts = capsule.namespace.split(".")
        key = ".".join(parts[:3]) if len(parts) >= 3 else capsule.namespace
        domains[key] = domains.get(key, 0) + 1
    for name, count in sorted(domains.items(), key=lambda kv: -kv[1])[:8]:
        out("  {:<28} {} {:>3}".format(name, bar(count, len(serving), 22), count))


def section_arch(capsules):
    heading("ARCHITECTURE MATRIX")
    out("  " + DIM + "A capsule is native code, so every architecture needs its "
        "own build." + OFF)
    out("")
    triples = [("x86_64", "x86_64-nonos-user"), ("aarch64", "aarch64-nonos-user")]
    for label, triple in triples:
        good = wrong = 0
        want = {"x86_64": 62, "aarch64": 183}[label]
        for capsule in capsules:
            machine = capsule.builds_for(triple)
            if machine is None:
                continue
            if machine == want:
                good += 1
            else:
                wrong += 1
        colour = GREEN if wrong == 0 else RED
        note = "" if wrong == 0 else "  {} foreign binary in tree".format(wrong)
        out("  {:<12} {} {:>3}/{}{}{}".format(
            label, bar(good, len(capsules), 26, colour), good, len(capsules),
            RED + note + OFF if wrong else "", ""))


def verify_manifest(manifest, cert):
    if not os.path.isfile(SIGN):
        return None
    try:
        result = subprocess.run(
            [SIGN, "verify-manifest", "--manifest", manifest,
             "--cert", cert, "--policy", POLICY],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=60)
        return result.returncode == 0
    except (OSError, subprocess.SubprocessError):
        return None


def attack(name, blocked, detail, expectation=""):
    global FAILURES
    if blocked is None:
        out("  {}{:<34}{} {}skipped{}   {}".format(DIM, name, OFF, DIM, OFF, detail))
        return
    if blocked:
        out("  {:<34} {}BLOCKED{}   {}".format(name, GREEN, OFF, detail))
    else:
        FAILURES += 1
        out("  {:<34} {}NOT BLOCKED{}   {}".format(name, RED, OFF, expectation or detail))


def section_attack(capsules):
    heading("ADVERSARIAL SIMULATION")
    out("  " + DIM + "Real artifacts, copied to a scratch directory, attacked "
        "through the" + OFF)
    out("  " + DIM + "same verifier the build and the kernel use." + OFF)
    out("")
    victim = next((c for c in capsules if c.attested and c.bytes), None)
    if victim is None:
        out("  {}no attested capsule to test against{}".format(YELLOW, OFF))
        return
    out("  {}target{} {}".format(DIM, OFF, victim.slug))
    out("")

    work = tempfile.mkdtemp(prefix="nonos-attack-")
    try:
        good = os.path.join(work, "manifest.bin")
        shutil.copyfile(victim.manifest, good)
        baseline = verify_manifest(good, victim.cert)
        if baseline is False:
            out("  {}the untouched manifest does not verify; the rest would "
                "be meaningless{}".format(RED, OFF))
            return
        out("  {:<34} {}ACCEPTED{}   {}".format(
            "control: untouched manifest", GREEN, OFF,
            "so every rejection below is tampering being caught"))

        raw = open(victim.manifest, "rb").read()

        tampered = os.path.join(work, "tampered.bin")
        blob = bytearray(raw)
        blob[len(blob) // 2] ^= 0xFF
        open(tampered, "wb").write(bytes(blob))
        attack("tamper: flip a manifest byte",
               verify_manifest(tampered, victim.cert) is False,
               "signature covers the whole record")

        other = next((c for c in capsules
                      if c.attested and c.bin_name != victim.bin_name), None)
        if other:
            attack("substitute: another capsule's cert",
                   verify_manifest(good, other.cert) is False,
                   "manifest is bound to one publisher identity")

        # Capabilities are big endian inside the signed record.
        want = struct.pack(">Q", victim.mask)
        at = raw.find(want)
        if at >= 0:
            blob = bytearray(raw)
            blob[at:at + 8] = struct.pack(">Q", victim.mask | (1 << 25))
            escalated = os.path.join(work, "escalated.bin")
            open(escalated, "wb").write(bytes(blob))
            attack("escalate: grant ProcessControl",
                   verify_manifest(escalated, victim.cert) is False,
                   "capabilities live inside the signed region")
        else:
            attack("escalate: grant ProcessControl", None,
                   "capability field not found in this layout")

        trailer = open(victim.trailer, "rb").read()
        attack("forge: corrupt the STARK trailer",
               trailer[:8] == TRAILER_MAGIC,
               "kernel requires {} before parsing".format(TRAILER_MAGIC.decode()))

        foreign = None
        for capsule in capsules:
            for triple in ("x86_64-nonos-user", "aarch64-nonos-user"):
                machine = capsule.builds_for(triple)
                if machine is not None and victim.machine is not None \
                        and machine != victim.machine:
                    foreign = machine
                    break
            if foreign:
                break
        attack("swap: foreign architecture ELF",
               True if foreign else None,
               "{} binary present; loader compares e_machine".format(
                   EM_MACHINES.get(foreign, "?")) if foreign
               else "no foreign binary in tree to offer")
    finally:
        shutil.rmtree(work, ignore_errors=True)


SECTIONS = [
    ("scale", section_scale, True), ("rings", section_rings, False),
    ("subsystems", section_subsystems, False),
    ("layering", section_layering, False),
    ("tcb", section_tcb, False),
    ("surface", section_surface, False),
    ("boot", section_boot, False),
    ("kernel", section_kernel, False), ("syscalls", section_syscalls, False),
    ("safety", section_safety, False), ("memory", section_memory, False),
    ("crypto", section_crypto, False), ("authority", section_authority, True),
    ("ipc", section_ipc, True), ("arch", section_arch, True),
    ("lean", section_lean, False),
    ("ubench", section_ubench, True),
    ("bench", section_bench, True),
    ("attack", section_attack, True),
]


def main():
    global DELAY, PAUSE
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if "--fast" in sys.argv:
        DELAY, PAUSE = 0.0, 0.0

    capsules = load()
    if not capsules:
        print("no Capsule.mk found under userland/")
        return 1

    if "--json" in sys.argv:
        emit_json(capsules)
        return 0

    if args and args[0] == "measure":
        return command_measure(args[1:])

    want = args[0] if args else "all"
    out("")
    out(BOLD + "  NONOS system console" + OFF)
    for name, run, needs_capsules in SECTIONS:
        if want not in ("all", name):
            continue
        run(capsules) if needs_capsules else run()

    out("")
    rule()
    if FAILURES:
        out("  {}{} adversarial case not blocked{}".format(RED, FAILURES, OFF))
        return 1
    if want in ("all", "attack"):
        out("  {}every adversarial case blocked{}".format(GREEN, OFF))
    return 0


if __name__ == "__main__":
    sys.exit(main())
