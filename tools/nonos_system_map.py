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
"""What authority every capsule in this image actually holds.

Reads the Capsule.mk that declared each capsule and the signed artifacts the
build produced from it, so the answer is the one the spawn gate enforces
rather than a description of it. Nothing here is asserted: every number is
counted from a file on disk.

    python3 tools/nonos_system_map.py           the whole map
    python3 tools/nonos_system_map.py --fast    no typing delay
    python3 tools/nonos_system_map.py --tsv     one capsule per line
    python3 tools/nonos_system_map.py --rare    only the scarce authority
"""

import os
import struct
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.basename(ROOT)
TRUST = os.path.join(ROOT, "nonos-data", "trust")
USERLAND = os.path.join(ROOT, "userland")
TARGET = os.environ.get("NONOS_USER_TARGET", "x86_64-nonos-user")

EM_MACHINES = {62: "x86-64", 183: "AArch64", 243: "RISC-V"}

# Mirrors Capability::bit in src/capabilities/types/bit.rs. A bit the kernel
# knows and this does not is reported as its number rather than dropped, so the
# two cannot drift apart quietly.
CAPABILITIES = [
    (1 << 0, "CoreExec"), (1 << 1, "IO"), (1 << 2, "Network"), (1 << 3, "IPC"),
    (1 << 4, "Memory"), (1 << 5, "Crypto"), (1 << 6, "FileSystem"),
    (1 << 7, "Hardware"), (1 << 8, "Debug"), (1 << 9, "Admin"),
    (1 << 10, "RegisterService"), (1 << 11, "GfxQuery"), (1 << 12, "GfxCreate"),
    (1 << 13, "GfxMap"), (1 << 14, "GfxPresent"), (1 << 15, "DeviceEnum"),
    (1 << 16, "Driver"), (1 << 17, "Mmio"), (1 << 18, "Irq"), (1 << 19, "Dma"),
    (1 << 20, "Pio"), (1 << 21, "InputSource"), (1 << 22, "TimeSet"),
    (1 << 23, "SpawnBroker"), (1 << 24, "SpawnWindow"), (1 << 25, "ProcessControl"),
    (1 << 26, "StoreWrite"), (1 << 27, "EnrolDevRoot"), (1 << 28, "Keyring"),
    (1 << 29, "Entropy"), (1 << 30, "AppInstall"),
]

# Authority that lets its holder act on something it does not own: put pixels
# on the screen, start or stop another process, mint a service name, move the
# clock. A second holder appearing against any of these is a change that wants
# explaining, which is the whole reason to print them separately.
SCARCE = {
    "Admin", "RegisterService", "GfxPresent", "TimeSet",
    "SpawnBroker", "SpawnWindow", "ProcessControl", "Pio",
}

BOLD = "\033[1m"
DIM = "\033[2m"
CYAN = "\033[36m"
GREEN = "\033[32m"
YELLOW = "\033[33m"
RED = "\033[31m"
OFF = "\033[0m"

DELAY = 0.006
PAUSE = 0.35


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
    rule()
    out(BOLD + CYAN + "  " + title + OFF)
    rule()


def commas(n):
    return "{:,}".format(n)


def mib(n):
    return "{:.2f} MiB".format(n / 1048576.0)


def decode(mask):
    """Capability names for a mask, with any unknown bits kept visible."""
    names = []
    seen = 0
    for bit, name in CAPABILITIES:
        if mask & bit:
            names.append(name)
            seen |= bit
    leftover = mask & ~seen
    if leftover:
        names.append("bit:0x{:x}".format(leftover))
    return names


def field_of(path, key):
    """First value of a `KEY := value` line in a Capsule.mk."""
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as handle:
            for line in handle:
                if line.startswith(key):
                    parts = line.split(":=", 1)
                    if len(parts) == 2:
                        return parts[1].strip()
    except OSError:
        pass
    return ""


def machine_of(path):
    try:
        with open(path, "rb") as handle:
            head = handle.read(20)
    except OSError:
        return None
    if len(head) < 20 or head[:4] != b"\x7fELF":
        return None
    return struct.unpack_from("<H", head, 18)[0]


def size_of(path):
    try:
        return os.path.getsize(path)
    except OSError:
        return 0


class Capsule(object):
    def __init__(self, mk):
        self.dir = os.path.dirname(mk)
        self.slug = field_of(mk, "CAPSULE_SLUG")
        # The signed artifacts are named by the binary, not the slug. Getting
        # that wrong reports a fully signed capsule as unsigned, which is worse
        # than reporting nothing.
        self.bin_name = field_of(mk, "CAPSULE_BIN_NAME") or self.slug
        self.namespace = field_of(mk, "CAPSULE_NAMESPACE")
        self.service = field_of(mk, "CAPSULE_SERVICE_ENDPOINT") or "none"
        raw = field_of(mk, "CAPSULE_REQUIRED_CAPS") or "0"
        try:
            self.mask = int(raw, 0)
        except ValueError:
            self.mask = 0
        self.binary = os.path.join(self.dir, "target", TARGET, "release", self.bin_name)
        self.bytes = size_of(self.binary)
        self.machine = machine_of(self.binary)
        self.artifacts = {}
        for kind, ext in (
            ("cert", "nonos_id_cert.bin"),
            ("manifest", "manifest.bin"),
            ("trailer", "zk_trailer.bin"),
        ):
            self.artifacts[kind] = size_of(
                os.path.join(TRUST, "capsules", "{}.{}".format(self.bin_name, ext))
            )

    @property
    def caps(self):
        return decode(self.mask)

    @property
    def attested(self):
        return all(size > 0 for size in self.artifacts.values())

    @property
    def port(self):
        parts = self.service.split(":")
        return parts[1] if len(parts) >= 2 else ""


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


def root_hex(name):
    path = os.path.join(TRUST, "policy", name)
    try:
        with open(path, "rb") as handle:
            return handle.read().hex()
    except OSError:
        return ""


def report_roots(capsules):
    heading("IMAGE")
    built = [c for c in capsules if c.bytes]
    total = sum(c.bytes for c in built)
    attested = [c for c in built if c.attested]
    out("  {}{:<16}{} {}".format(DIM, "target", OFF, TARGET))
    out("  {}{:<16}{} {} declared, {} built, {} attested".format(
        DIM, "capsules", OFF, len(capsules), len(built), len(attested)))
    out("  {}{:<16}{} {} bytes  ({})".format(
        DIM, "capsule bytes", OFF, commas(total), mib(total)))
    machines = sorted({EM_MACHINES.get(c.machine, str(c.machine))
                       for c in built if c.machine is not None})
    colour = GREEN if len(machines) == 1 else RED
    out("  {}{:<16}{} {}{}{}".format(DIM, "architecture", OFF, colour,
                                     ", ".join(machines) or "none", OFF))
    for label, name in (("policy root", "zk_capsule_policy_root.bin"),
                        ("kernel attest", "kernel_attest_root.bin")):
        value = root_hex(name)
        if value:
            out("  {}{:<16}{} {}".format(DIM, label, OFF, value))


def report_table(capsules):
    heading("CAPSULES")
    out("  {}{:<22} {:>10} {:>6}  {}{}".format(DIM, "NAME", "BYTES", "PORT",
                                               "AUTHORITY", OFF))
    for capsule in sorted(capsules, key=lambda c: c.slug):
        if not capsule.bytes:
            continue
        mark = "" if capsule.attested else YELLOW + "  [not attested]" + OFF
        out("  {:<22} {:>10} {:>6}  {}{}".format(
            capsule.slug, commas(capsule.bytes), capsule.port,
            " ".join(capsule.caps), mark))


def report_distribution(capsules):
    heading("AUTHORITY DISTRIBUTION")
    built = [c for c in capsules if c.bytes]
    total = len(built)
    counts = []
    for bit, name in CAPABILITIES:
        holders = [c for c in built if c.mask & bit]
        if holders:
            counts.append((len(holders), name))
    counts.sort(reverse=True)
    width = 34
    for count, name in counts:
        filled = max(1, int(round(width * count / total))) if total else 0
        colour = YELLOW if name in SCARCE else CYAN
        bar = colour + "#" * filled + OFF + DIM + "." * (width - filled) + OFF
        out("  {:<17} {} {:>3} of {}".format(name, bar, count, total))


def report_scarce(capsules):
    heading("SCARCE AUTHORITY")
    out("  " + DIM + "Authority that acts on something its holder does not own." + OFF)
    out("")
    built = [c for c in capsules if c.bytes]
    for bit, name in CAPABILITIES:
        if name not in SCARCE:
            continue
        holders = sorted(c.slug for c in built if c.mask & bit)
        if not holders:
            continue
        colour = GREEN if len(holders) <= 2 else YELLOW
        out("  {:<17} {}{}{}".format(name, colour, ", ".join(holders), OFF))


def report_tsv(capsules):
    print("\t".join(["slug", "namespace", "bin", "caps_mask", "service",
                     "bytes", "machine", "attested", "capabilities"]))
    for capsule in sorted(capsules, key=lambda c: c.slug):
        print("\t".join([
            capsule.slug,
            capsule.namespace,
            capsule.bin_name,
            hex(capsule.mask),
            capsule.service,
            str(capsule.bytes),
            EM_MACHINES.get(capsule.machine, str(capsule.machine)),
            "yes" if capsule.attested else "no",
            " ".join(capsule.caps),
        ]))


def main():
    global DELAY, PAUSE
    args = sys.argv[1:]
    if "--fast" in args or "--tsv" in args:
        DELAY = 0.0
        PAUSE = 0.0

    capsules = load()
    if not capsules:
        print("no Capsule.mk found under {}/userland".format(REPO))
        return 1

    if "--tsv" in args:
        report_tsv(capsules)
        return 0

    out()
    out(BOLD + "  NONOS system map" + OFF)
    if "--rare" in args:
        report_scarce(capsules)
    else:
        report_roots(capsules)
        report_table(capsules)
        report_distribution(capsules)
        report_scarce(capsules)
    time.sleep(PAUSE)
    rule()
    return 0


if __name__ == "__main__":
    sys.exit(main())
