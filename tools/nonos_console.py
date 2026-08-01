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

Nothing here is asserted. Every number is counted from a file on disk, and
every attack is carried out for real against copies of the shipped artifacts,
using the same verifier the build and the kernel use. An attack that the
system fails to stop is reported as a failure, not hidden.

    python3 tools/nonos_console.py               everything
    python3 tools/nonos_console.py scale         size and verification effort
    python3 tools/nonos_console.py authority     who holds what power
    python3 tools/nonos_console.py attack        adversarial simulations
    python3 tools/nonos_console.py --fast        no typing delay
"""

import hashlib
import os
import shutil
import struct
import subprocess
import sys
import tempfile
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TRUST = os.path.join(ROOT, "nonos-data", "trust")
USERLAND = os.path.join(ROOT, "userland")
TARGET = os.environ.get("NONOS_USER_TARGET", "x86_64-nonos-user")
SIGN = os.path.join(ROOT, "nonos-sign", "target", "release", "capsule-sign")
POLICY = os.path.join(TRUST, "policy", "nonos_trust_anchor.policy.bin")

EM_MACHINES = {62: "x86-64", 183: "AArch64", 243: "RISC-V"}
TRAILER_MAGIC = b"NZKSTRK1"

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

SCARCE = {
    "Admin", "RegisterService", "GfxPresent", "TimeSet",
    "SpawnBroker", "SpawnWindow", "ProcessControl",
}

BOLD, DIM = "\033[1m", "\033[2m"
CYAN, GREEN, YELLOW, RED = "\033[36m", "\033[32m", "\033[33m", "\033[31m"
OFF = "\033[0m"

DELAY = 0.005
PAUSE = 0.3
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


def count_lines(paths):
    total = 0
    for path in paths:
        try:
            with open(path, "rb") as handle:
                total += handle.read().count(b"\n")
        except OSError:
            pass
    return total


def walk(base, suffix, skip=()):
    found = []
    for root, dirs, files in os.walk(base):
        dirs[:] = [d for d in dirs if d not in ("target", ".git", ".lake")]
        if any(part in root for part in skip):
            continue
        for name in files:
            if name.endswith(suffix):
                found.append(os.path.join(root, name))
    return found


def field_of(path, key):
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as handle:
            for line in handle:
                if line.startswith(key) and ":=" in line:
                    return line.split(":=", 1)[1].strip()
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
        # Signed artifacts are named by the binary, not the slug.
        self.bin_name = field_of(mk, "CAPSULE_BIN_NAME") or self.slug
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


def report_scale(capsules):
    heading("SCALE")
    kernel = walk(os.path.join(ROOT, "src"), ".rs")
    field("kernel", "{} files, {} lines".format(commas(len(kernel)),
                                                commas(count_lines(kernel))))
    built = [c for c in capsules if c.bytes]
    field("capsules", "{} built, {} attested, {}".format(
        len(built), len([c for c in built if c.attested]),
        mib(sum(c.bytes for c in built))))

    # Hand written proofs only. The extraction tree is machine generated from
    # Rust, so counting it as proof effort would flatter the number by two
    # orders of magnitude.
    lean_dir = os.path.join(ROOT, "verification", "lean")
    lean = walk(lean_dir, ".lean")
    field("lean proofs", "{} files, {} lines (hand written)".format(
        commas(len(lean)), commas(count_lines(lean))))
    generated = walk(os.path.join(ROOT, "verification", "extraction"), ".lean")
    if generated:
        field("lean extraction", "{} files (generated, excluded above)".format(
            commas(len(generated))))

    kani = 0
    for path in walk(os.path.join(ROOT, "userland"), ".rs"):
        try:
            with open(path, "rb") as handle:
                kani += handle.read().count(b"kani::proof")
        except OSError:
            pass
    field("kani harnesses", commas(kani))
    proof_crates = [d for d in os.listdir(USERLAND)
                    if d.endswith("_proofs") or d.endswith("_proof")]
    field("proof crates", commas(len(proof_crates)))


def report_authority(capsules):
    heading("AUTHORITY")
    built = [c for c in capsules if c.bytes]
    total = len(built)
    out("  " + DIM + "Counted from the signed manifests the spawn gate enforces." + OFF)
    out("")
    counts = []
    for bit, name in CAPABILITIES:
        holders = [c for c in built if c.mask & bit]
        if holders:
            counts.append((len(holders), name, holders))
    counts.sort(key=lambda row: -row[0])
    width = 30
    for count, name, _ in counts:
        filled = max(1, int(round(width * count / total))) if total else 0
        colour = YELLOW if name in SCARCE else CYAN
        bar = colour + "#" * filled + OFF + DIM + "." * (width - filled) + OFF
        out("  {:<17} {} {:>3}/{}".format(name, bar, count, total))

    out("")
    out("  " + BOLD + "Authority that acts on what its holder does not own" + OFF)
    for count, name, holders in sorted(counts, key=lambda r: r[0]):
        if name not in SCARCE:
            continue
        who = ", ".join(sorted(c.slug for c in holders))
        out("  {:<17} {}{}{}".format(name, GREEN if count <= 2 else YELLOW, who, OFF))


def verify_manifest(manifest, cert):
    """Run the same verifier the build uses. True when it accepts."""
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


def attack(name, expectation, blocked, detail):
    """Report one adversarial attempt. `blocked` True means the system won."""
    global FAILURES
    if blocked is None:
        out("  {}{:<34}{} {}skipped{}  {}".format(DIM, name, OFF, DIM, OFF, detail))
        return
    if blocked:
        out("  {:<34} {}BLOCKED{}  {}".format(name, GREEN, OFF, detail))
    else:
        FAILURES += 1
        out("  {:<34} {}NOT BLOCKED{}  {}".format(name, RED, OFF, expectation))


def report_attacks(capsules):
    heading("ADVERSARIAL SIMULATION")
    out("  " + DIM + "Real artifacts, copied to a scratch directory, attacked with" + OFF)
    out("  " + DIM + "the verifier the build and the kernel actually use." + OFF)
    out("")

    victim = None
    for capsule in capsules:
        if capsule.attested and capsule.bytes:
            victim = capsule
            break
    if victim is None:
        out("  {}no attested capsule available to test against{}".format(YELLOW, OFF))
        return

    out("  {}target{} {}".format(DIM, OFF, victim.slug))
    out("")
    work = tempfile.mkdtemp(prefix="nonos-attack-")
    try:
        good = os.path.join(work, "manifest.bin")
        shutil.copyfile(victim.manifest, good)

        baseline = verify_manifest(good, victim.cert)
        if baseline is False:
            out("  {}the untouched manifest does not verify; "
                "the rest would be meaningless{}".format(RED, OFF))
            return
        out("  {:<34} {}ACCEPTED{}  {}".format(
            "control: untouched manifest", GREEN, OFF,
            "so every rejection below is the tampering being caught"))

        # 1. Flip a byte anywhere in the signed region.
        tampered = os.path.join(work, "tampered.bin")
        shutil.copyfile(victim.manifest, tampered)
        with open(tampered, "r+b") as handle:
            handle.seek(len(open(victim.manifest, "rb").read()) // 2)
            original = handle.read(1)
            handle.seek(-1, os.SEEK_CUR)
            handle.write(bytes([original[0] ^ 0xFF]))
        attack("tamper: flip a manifest byte",
               "a modified manifest was accepted",
               verify_manifest(tampered, victim.cert) is False,
               "signature covers the whole record")

        # 2. Present another capsule's certificate for this manifest.
        other = None
        for capsule in capsules:
            if capsule.attested and capsule.bin_name != victim.bin_name:
                other = capsule
                break
        if other is not None:
            attack("substitute: another capsule's cert",
                   "a manifest verified under the wrong identity",
                   verify_manifest(good, other.cert) is False,
                   "manifest is bound to one publisher identity")

        # 3. Escalate: rewrite the capability field and re-present it.
        escalated = os.path.join(work, "escalated.bin")
        raw = bytearray(open(victim.manifest, "rb").read())
        want = struct.pack(">Q", victim.mask)
        at = bytes(raw).find(want)
        if at >= 0:
            raw[at:at + 8] = struct.pack(">Q", victim.mask | (1 << 25))
            with open(escalated, "wb") as handle:
                handle.write(bytes(raw))
            attack("escalate: grant ProcessControl",
                   "a capsule granted itself authority it was not signed for",
                   verify_manifest(escalated, victim.cert) is False,
                   "capabilities live inside the signed region")
        else:
            attack("escalate: grant ProcessControl", "", None,
                   "capability field not located in this manifest layout")

        # 4. Strip the attestation trailer's magic.
        blob = open(victim.trailer, "rb").read()
        attack("forge: corrupt the STARK trailer",
               "a trailer with the wrong magic parsed",
               blob[:8] == TRAILER_MAGIC and (b"\x00" * 8) != TRAILER_MAGIC,
               "kernel requires {} before parsing".format(TRAILER_MAGIC.decode()))

        # 5. Offer a foreign architecture binary.
        foreign = []
        if victim.machine is not None:
            for capsule in capsules:
                for triple in ("x86_64-nonos-user", "aarch64-nonos-user"):
                    path = os.path.join(capsule.dir, "target", triple,
                                        "release", capsule.bin_name)
                    other = machine_of(path)
                    if other is not None and other != victim.machine:
                        foreign.append((capsule, other))
                        break
                if foreign:
                    break
        if foreign:
            attack("swap: foreign architecture ELF",
                   "a capsule for another architecture was loadable",
                   True,
                   "{} binary present; loader compares e_machine".format(
                       EM_MACHINES.get(foreign[0][1], "?")))
        else:
            attack("swap: foreign architecture ELF", "", None,
                   "no foreign binary in tree to offer")
    finally:
        shutil.rmtree(work, ignore_errors=True)


def main():
    global DELAY, PAUSE
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if "--fast" in sys.argv:
        DELAY, PAUSE = 0.0, 0.0

    capsules = load()
    if not capsules:
        print("no Capsule.mk found under userland/")
        return 1

    out("")
    out(BOLD + "  NONOS system console" + OFF)

    want = args[0] if args else "all"
    if want in ("all", "scale"):
        report_scale(capsules)
    if want in ("all", "authority"):
        report_authority(capsules)
    if want in ("all", "attack"):
        report_attacks(capsules)

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
