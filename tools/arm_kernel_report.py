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
"""Report what the aarch64 kernel image actually is.

Reads the ELF directly rather than shelling out to binutils, so it runs on a
machine with no cross toolchain installed. Every path it prints is relative to
the repository root.

    python3 tools/arm_kernel_report.py [--fast] [--delay MS]
"""

import hashlib
import os
import struct
import sys
import time

EM_AARCH64 = 183
EM_X86_64 = 62
ET_NAMES = {1: "Relocatable", 2: "Executable", 3: "Shared object", 4: "Core"}
MACHINES = {EM_AARCH64: "AArch64", EM_X86_64: "x86-64", 243: "RISC-V"}
PT_NAMES = {1: "LOAD", 2: "DYNAMIC", 4: "NOTE", 6: "PHDR", 7: "TLS", 0x6474E552: "RELRO"}
SHT_SYMTAB = 2
STT_FUNC = 2
STT_OBJECT = 1

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.basename(ROOT)
KERNEL = "target/aarch64-nonos/release/nonos-kernel"
CAPSULE_TARGET = "aarch64-nonos-user"

BOLD = "\033[1m"
DIM = "\033[2m"
CYAN = "\033[36m"
GREEN = "\033[32m"
YELLOW = "\033[33m"
RED = "\033[31m"
OFF = "\033[0m"

DELAY = 0.010
PAUSE = 0.40


def out(text="", delay=None):
    """Type one line out a character at a time."""
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
    out(DIM + "-" * 66 + OFF, delay=0)


def heading(title):
    time.sleep(PAUSE)
    rule()
    out(BOLD + CYAN + "  " + title + OFF)
    rule()


def field(name, value, colour=""):
    tail = OFF if colour else ""
    out("  " + DIM + name.ljust(12) + OFF + " " + colour + str(value) + tail)


def commas(n):
    return "{:,}".format(n)


def mib(n):
    return "{:.2f} MiB".format(n / 1048576.0)


class Elf(object):
    """Just enough ELF64 to describe an image honestly."""

    def __init__(self, path):
        with open(path, "rb") as handle:
            self.blob = handle.read()
        if self.blob[:4] != b"\x7fELF":
            raise ValueError("not an ELF file")
        self.bits = 64 if self.blob[4] == 2 else 32
        self.little = self.blob[5] == 1
        end = "<" if self.little else ">"
        self.end = end
        self.e_type, self.e_machine = struct.unpack_from(end + "HH", self.blob, 16)
        self.entry, self.phoff, self.shoff = struct.unpack_from(end + "QQQ", self.blob, 24)
        self.phentsize, self.phnum = struct.unpack_from(end + "HH", self.blob, 54)
        self.shentsize, self.shnum, self.shstrndx = struct.unpack_from(end + "HHH", self.blob, 58)

    def segments(self):
        for i in range(self.phnum):
            base = self.phoff + i * self.phentsize
            p_type, p_flags = struct.unpack_from(self.end + "II", self.blob, base)
            p_vaddr = struct.unpack_from(self.end + "Q", self.blob, base + 16)[0]
            p_filesz, p_memsz = struct.unpack_from(self.end + "QQ", self.blob, base + 32)
            yield p_type, p_flags, p_vaddr, p_filesz, p_memsz

    def shdr(self, i):
        base = self.shoff + i * self.shentsize
        name, sh_type = struct.unpack_from(self.end + "II", self.blob, base)
        flags, addr, offset, size = struct.unpack_from(self.end + "QQQQ", self.blob, base + 8)
        link, info = struct.unpack_from(self.end + "II", self.blob, base + 40)
        entsize = struct.unpack_from(self.end + "Q", self.blob, base + 56)[0]
        return name, sh_type, flags, addr, offset, size, link, info, entsize

    def string(self, table_offset, index):
        stop = self.blob.index(b"\x00", table_offset + index)
        return self.blob[table_offset + index:stop].decode("utf-8", "replace")

    def sections(self):
        if self.shnum == 0 or self.shstrndx >= self.shnum:
            return []
        strtab = self.shdr(self.shstrndx)[4]
        rows = []
        for i in range(self.shnum):
            name, _t, _f, addr, _o, size, _l, _i, _e = self.shdr(i)
            rows.append((self.string(strtab, name), addr, size))
        return rows

    def symbols(self):
        """Yield (name, type, value, size) from .symtab when it is present."""
        table = None
        for i in range(self.shnum):
            _n, sh_type, _f, _a, offset, size, link, _i, entsize = self.shdr(i)
            if sh_type == SHT_SYMTAB and entsize:
                table = (offset, size, link, entsize)
                break
        if table is None:
            return
        offset, size, link, entsize = table
        strtab = self.shdr(link)[4]
        for pos in range(offset, offset + size, entsize):
            st_name = struct.unpack_from(self.end + "I", self.blob, pos)[0]
            st_info = self.blob[pos + 4]
            st_value, st_size = struct.unpack_from(self.end + "QQ", self.blob, pos + 8)
            if st_name == 0:
                continue
            yield self.string(strtab, st_name), st_info & 0xF, st_value, st_size


def tidy(symbol):
    """Trim a mangled Rust name down to something readable on screen."""
    name = symbol
    if name.startswith("_RN"):
        parts = []
        digits = ""
        i = 3
        while i < len(name):
            ch = name[i]
            if ch.isdigit():
                digits += ch
                i += 1
                continue
            if digits:
                length = int(digits)
                piece = name[i:i + length]
                if piece and not piece.startswith("_"):
                    parts.append(piece)
                i += length
                digits = ""
                continue
            i += 1
        if parts:
            name = "::".join(parts[1:] if len(parts) > 1 else parts)
    if "." in name:
        name = name.split(".")[0]
    return name[:52]


def machine_of(path):
    try:
        with open(path, "rb") as handle:
            head = handle.read(20)
    except (OSError, IOError):
        return None
    if len(head) < 20 or head[:4] != b"\x7fELF":
        return None
    return struct.unpack_from("<H", head, 18)[0]


def report_image(path):
    heading("IMAGE")
    size = os.path.getsize(path)
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        while True:
            chunk = handle.read(1 << 20)
            if not chunk:
                break
            digest.update(chunk)
    field("path", REPO + "/" + KERNEL)
    field("size", commas(size) + " bytes  (" + mib(size) + ")", BOLD)
    field("sha256", digest.hexdigest())
    field("built", time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(path))))


def report_header(elf):
    heading("ELF HEADER")
    field("class", str(elf.bits) + "-bit")
    field("encoding", "little endian" if elf.little else "big endian")
    machine = MACHINES.get(elf.e_machine, "unknown (" + str(elf.e_machine) + ")")
    field("machine", machine, GREEN if elf.e_machine == EM_AARCH64 else RED)
    field("type", ET_NAMES.get(elf.e_type, str(elf.e_type)))
    field("entry", "0x{:016x}".format(elf.entry))
    field("segments", str(elf.phnum))
    field("sections", str(elf.shnum))


def report_segments(elf):
    heading("SEGMENTS")
    out("  " + DIM + "TYPE".ljust(8) + " " + "VADDR".ljust(18) + " "
        + "FILESZ".rjust(13) + " " + "MEMSZ".rjust(13) + "  PERM" + OFF)
    for p_type, p_flags, vaddr, filesz, memsz in elf.segments():
        if p_type not in PT_NAMES:
            continue
        perm = ("r" if p_flags & 4 else "-") + ("w" if p_flags & 2 else "-") \
            + ("x" if p_flags & 1 else "-")
        out("  " + PT_NAMES[p_type].ljust(8) + " " + "0x{:016x}".format(vaddr)
            + " " + commas(filesz).rjust(13) + " " + commas(memsz).rjust(13) + "  " + perm)


def report_sections(elf):
    heading("SECTION BUDGET")
    rows = [(name, size, addr) for name, addr, size in elf.sections() if size]
    total = sum(row[1] for row in rows)
    rows.sort(key=lambda row: row[1], reverse=True)
    out("  " + DIM + "SECTION".ljust(22) + " " + "BYTES".rjust(14) + " "
        + "SHARE".rjust(8) + "  ADDR" + OFF)
    for name, size, addr in rows[:10]:
        share = (100.0 * size / total) if total else 0.0
        out("  " + name.ljust(22) + " " + commas(size).rjust(14) + " "
            + "{:6.1f}%".format(share).rjust(8) + "  0x{:x}".format(addr))
    out("  " + BOLD + "total".ljust(22) + " " + commas(total).rjust(14) + OFF)


def report_symbols(elf):
    heading("SYMBOLS")
    funcs = []
    objects = 0
    total = 0
    for name, kind, _value, size in elf.symbols():
        total += 1
        if kind == STT_FUNC:
            funcs.append((size, name))
        elif kind == STT_OBJECT:
            objects += 1
    if total == 0:
        field("symtab", "stripped", YELLOW)
        return
    field("total", commas(total))
    field("functions", commas(len(funcs)))
    field("objects", commas(objects))
    funcs.sort(reverse=True)
    out()
    out("  " + DIM + "largest functions" + OFF)
    for size, name in funcs[:8]:
        out("  " + commas(size).rjust(9) + "  " + tidy(name))


def report_entry_path(elf):
    heading("AARCH64 ENTRY PATH")
    wanted = ["_start", "kernel_entry", "microkernel_init", "microkernel_main"]
    found = {}
    for name, kind, value, _size in elf.symbols():
        if kind != STT_FUNC:
            continue
        for want in wanted:
            if name == want or name.endswith(want):
                found.setdefault(want, value)
    for want in wanted:
        if want in found:
            out("  " + want.ljust(20) + " " + GREEN + "0x{:016x}".format(found[want]) + OFF)
        else:
            out("  " + want.ljust(20) + " " + YELLOW + "absent" + OFF)


def report_capsules():
    heading("CAPSULES BUILT FOR AARCH64")
    userland = os.path.join(ROOT, "userland")
    good = []
    foreign = []
    if os.path.isdir(userland):
        for crate in sorted(os.listdir(userland)):
            release = os.path.join(userland, crate, "target", CAPSULE_TARGET, "release")
            if not os.path.isdir(release):
                continue
            for entry in sorted(os.listdir(release)):
                path = os.path.join(release, entry)
                if "." in entry or not os.path.isfile(path):
                    continue
                machine = machine_of(path)
                if machine is None:
                    continue
                size = os.path.getsize(path)
                # Checked per file rather than trusted from the directory name.
                # A capsule that failed to cross compile leaves its earlier
                # x86_64 build sitting here, and counting that would overstate
                # the port.
                if machine == EM_AARCH64:
                    good.append((size, entry))
                else:
                    foreign.append((size, entry, machine))
    total = sum(size for size, _n in good)
    field("target", CAPSULE_TARGET)
    field("aarch64", commas(len(good)), GREEN)
    field("total", commas(total) + " bytes  (" + mib(total) + ")")
    if foreign:
        field("not aarch64", str(len(foreign)) + " (excluded)", YELLOW)
        for size, name, machine in sorted(foreign, reverse=True):
            label = MACHINES.get(machine, str(machine))
            out("  " + YELLOW + commas(size).rjust(13) + "  " + name + "  [" + label + "]" + OFF)
    good.sort(reverse=True)
    out()
    out("  " + DIM + "largest" + OFF)
    for size, name in good[:8]:
        out("  " + commas(size).rjust(13) + "  " + name)


def report_trust():
    heading("SIGNED ARTIFACTS")
    trust = os.path.join(ROOT, "nonos-data", "trust", "capsules")
    if not os.path.isdir(trust):
        field("trust dir", "absent", YELLOW)
        return
    names = os.listdir(trust)
    field("id certs", commas(len([n for n in names if n.endswith(".nonos_id_cert.bin")])))
    field("manifests", commas(len([n for n in names if n.endswith(".manifest.bin")])))
    field("zk trailers", commas(len([n for n in names if n.endswith(".zk_trailer.bin")])))


def main():
    global DELAY, PAUSE
    args = sys.argv[1:]
    if "--fast" in args:
        DELAY = 0.0
        PAUSE = 0.0
    if "--delay" in args:
        index = args.index("--delay") + 1
        if index < len(args):
            DELAY = float(args[index]) / 1000.0

    os.chdir(ROOT)
    out()
    out(BOLD + "  NONOS aarch64 kernel report" + OFF)

    if not os.path.exists(KERNEL):
        out()
        out("  " + RED + "no aarch64 kernel at " + REPO + "/" + KERNEL + OFF)
        out("  build one with: make nonos-mk-arm")
        return 1

    elf = Elf(KERNEL)
    report_image(KERNEL)
    report_header(elf)
    report_segments(elf)
    report_sections(elf)
    report_symbols(elf)
    report_entry_path(elf)
    report_capsules()
    report_trust()
    time.sleep(PAUSE)
    rule()
    return 0


if __name__ == "__main__":
    sys.exit(main())
