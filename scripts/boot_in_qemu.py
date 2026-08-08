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

"""Download NONOS and boot it under QEMU, showing every step and its output.

Run it with no arguments and it will fetch the current release, check it
against the published checksums, work out the right QEMU invocation for the
machine it is on, and boot it with the serial console on screen.

    python3 boot_in_qemu.py

Nothing is installed and nothing is written outside the working directory.
The image is read-only to the virtual machine: NONOS is RAM resident, so a
session leaves nothing behind when the window closes.

Useful switches:

    --dir PATH        where to keep the download (default ./nonos)
    --iso PATH        boot an image you already have, skip the download
    --no-verify       boot without checking the checksum. Not advised
    --headless        no window, serial only, for a machine with no display
    --memory 4G       how much RAM to give it (default 4G)
    --serial FILE     also write the serial console to a file
"""

import argparse
import hashlib
import os
import platform
import shutil
import subprocess
import sys
import time
import urllib.request

RELEASE = "v0.9.1"
BASE = f"https://nonos.software/releases/{RELEASE}"
ISO = f"nonos-{RELEASE.lstrip('v')}.iso"
SUMS = "SHA256SUMS"

# Firmware. NONOS is UEFI only, so QEMU needs an OVMF build. These are where
# the common package managers put one; the first that exists wins.
OVMF_CANDIDATES = [
    "/opt/homebrew/share/qemu/edk2-x86_64-code.fd",
    "/usr/local/share/qemu/edk2-x86_64-code.fd",
    "/usr/share/OVMF/OVMF_CODE.fd",
    "/usr/share/ovmf/OVMF.fd",
    "/usr/share/edk2/x64/OVMF_CODE.fd",
    "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd",
]

BOLD, DIM, GREEN, RED, YELLOW, RESET = (
    "\033[1m",
    "\033[2m",
    "\033[32m",
    "\033[31m",
    "\033[33m",
    "\033[0m",
)


def say(text=""):
    print(text, flush=True)


def step(n, total, title):
    say()
    say(f"{BOLD}[{n}/{total}] {title}{RESET}")


def detail(text):
    say(f"      {DIM}{text}{RESET}")


def good(text):
    say(f"      {GREEN}ok{RESET}  {text}")


def warn(text):
    say(f"      {YELLOW}note{RESET}  {text}")


def die(text, hint=None):
    say()
    say(f"{RED}stopped{RESET}  {text}")
    if hint:
        say(f"         {hint}")
    sys.exit(1)


def find_qemu():
    exe = shutil.which("qemu-system-x86_64")
    if exe:
        return exe
    hint = {
        "Darwin": "brew install qemu",
        "Linux": "apt install qemu-system-x86  (or the equivalent for your distribution)",
    }.get(platform.system(), "install QEMU from https://www.qemu.org/download/")
    die("no qemu-system-x86_64 on this machine", hint)


def find_ovmf():
    for path in OVMF_CANDIDATES:
        if os.path.exists(path):
            return path
    die(
        "no UEFI firmware found for QEMU",
        "NONOS is UEFI only. Install an OVMF build: brew install qemu, "
        "or apt install ovmf, and re-run.",
    )


def download(url, dest, label):
    """Fetch with a progress line, so a slow link looks like progress."""
    detail(f"{url}")
    started = time.time()

    def report(count, block, total):
        if total <= 0:
            return
        done = min(count * block, total)
        pct = done * 100 // total
        mb, mbtotal = done / 1048576, total / 1048576
        secs = max(time.time() - started, 0.001)
        sys.stdout.write(
            f"\r      {label}  {pct:3d}%  {mb:6.1f} / {mbtotal:.1f} MB  "
            f"{mb / secs:5.1f} MB/s "
        )
        sys.stdout.flush()

    urllib.request.urlretrieve(url, dest, reporthook=report)
    sys.stdout.write("\r" + " " * 70 + "\r")
    sys.stdout.flush()


def sha256(path, label="hashing"):
    total = os.path.getsize(path)
    h = hashlib.sha256()
    read = 0
    with open(path, "rb") as f:
        while chunk := f.read(1024 * 1024):
            h.update(chunk)
            read += len(chunk)
            pct = read * 100 // total
            sys.stdout.write(f"\r      {label}  {pct:3d}%")
            sys.stdout.flush()
    sys.stdout.write("\r" + " " * 40 + "\r")
    sys.stdout.flush()
    return h.hexdigest()


def accel_for_host():
    """The acceleration this host can offer, and a word about what it means."""
    system = platform.system()
    if system == "Darwin":
        return ["-accel", "hvf", "-cpu", "host"], "hvf, the macOS hypervisor"
    if system == "Linux" and os.path.exists("/dev/kvm") and os.access("/dev/kvm", os.W_OK):
        return ["-accel", "kvm", "-cpu", "host"], "kvm"
    if system == "Linux" and os.path.exists("/dev/kvm"):
        return [], "none: /dev/kvm exists but is not writable, add yourself to the kvm group"
    return [], "none: this will boot, slowly"


def main():
    p = argparse.ArgumentParser(add_help=True, description=__doc__)
    p.add_argument("--dir", default="nonos")
    p.add_argument("--iso")
    p.add_argument("--no-verify", action="store_true")
    p.add_argument("--headless", action="store_true")
    p.add_argument("--memory", default="4G")
    p.add_argument("--serial")
    args = p.parse_args()

    total = 5
    say()
    say(f"{BOLD}NONOS {RELEASE} under QEMU{RESET}")
    say(f"{DIM}Nothing is installed. The image is read only to the machine, and{RESET}")
    say(f"{DIM}NONOS keeps nothing on disk, so closing the window ends the session.{RESET}")

    step(1, total, "Looking for what this needs")
    qemu = find_qemu()
    good(f"qemu at {qemu}")
    version = subprocess.run([qemu, "--version"], capture_output=True, text=True)
    detail(version.stdout.splitlines()[0] if version.stdout else "")
    ovmf = find_ovmf()
    good(f"UEFI firmware at {ovmf}")
    accel, accel_note = accel_for_host()
    good(f"acceleration: {accel_note}")

    step(2, total, "Fetching the image")
    if args.iso:
        iso = os.path.abspath(args.iso)
        if not os.path.exists(iso):
            die(f"{iso} does not exist")
        good(f"using {iso}")
        sums_path = None
    else:
        os.makedirs(args.dir, exist_ok=True)
        iso = os.path.join(args.dir, ISO)
        if os.path.exists(iso):
            good(f"already downloaded: {iso}")
        else:
            download(f"{BASE}/{ISO}", iso, "downloading")
            good(f"{os.path.getsize(iso) / 1048576:.1f} MB to {iso}")
        sums_path = os.path.join(args.dir, SUMS)
        download(f"{BASE}/{SUMS}", sums_path, "checksums ")
        good("published checksums fetched")

    step(3, total, "Checking it is the image we published")
    if args.no_verify or not sums_path:
        warn("skipped. A tampered image would boot and you would not know")
    else:
        expected = None
        for line in open(sums_path):
            digest, _, name = line.strip().partition("  ")
            if name == os.path.basename(iso):
                expected = digest
        if not expected:
            die(f"{SUMS} does not mention {os.path.basename(iso)}")
        detail(f"expected  {expected}")
        actual = sha256(iso)
        detail(f"actual    {actual}")
        if actual != expected:
            die(
                "the download does not match the published checksum",
                "Delete it and fetch again. If it fails twice, say so publicly.",
            )
        good("checksum matches")
        detail("This says the bytes are ours. The image also carries signatures")
        detail("and a proof for every capsule, which the bootloader checks itself.")

    step(4, total, "Building the QEMU command")
    cmd = [
        qemu,
        "-m", args.memory,
        "-smp", "2",
        "-machine", "q35",
        *accel,
        "-bios", ovmf,
        "-cdrom", iso,
        "-device", "virtio-net-pci,netdev=n0",
        "-netdev", "user,id=n0",
        "-device", "virtio-rng-pci",
        "-no-reboot",
    ]
    if args.headless:
        cmd += ["-display", "none", "-serial", "mon:stdio"]
    elif args.serial:
        cmd += ["-serial", f"file:{args.serial}"]
    else:
        cmd += ["-serial", "mon:stdio"]
    say()
    say("      " + " \\\n        ".join(cmd))
    say()
    detail("q35 because the image expects a modern chipset")
    detail("virtio-net for the network, which is the card the drivers know best")
    detail("virtio-rng so the guest has a real entropy source")
    detail("the serial console is printed below, which is where boot says what it did")

    step(5, total, "Booting")
    if args.serial:
        detail(f"serial console going to {args.serial}")
    say(f"{DIM}      Verified boot runs first: it checks the kernel's Ed25519 and{RESET}")
    say(f"{DIM}      ML-DSA-65 signatures, then every capsule is attested as it{RESET}")
    say(f"{DIM}      spawns. On a slow machine that takes a while and the splash{RESET}")
    say(f"{DIM}      sits at 100% while it works. Give it a couple of minutes.{RESET}")
    say(f"{DIM}      Quit with Ctrl+A then X.{RESET}")
    say()
    try:
        raise SystemExit(subprocess.call(cmd))
    except KeyboardInterrupt:
        say()
        say("      stopped")
        return 0


if __name__ == "__main__":
    sys.exit(main())
