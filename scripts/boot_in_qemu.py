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

"""Download NONOS and boot it under QEMU.

    python3 boot_in_qemu.py

It finds QEMU and the UEFI firmware, tells you how to install either if it
is missing, downloads the current release, checks it against the published
checksums, explains the machine it is about to build and why each part of it
is there, then boots it with the serial console on screen.

Nothing is installed on your machine and nothing is written outside the
working directory. The image is read only to the virtual machine, and NONOS
is RAM resident, so closing the window ends the session and leaves nothing.

    --dir PATH      where to keep the download          (default ./nonos)
    --iso PATH      boot an image you already have
    --memory SIZE   RAM for the machine                 (default 4G)
    --cpus N        processors for the machine          (default 2)
    --window        open a window instead of serial only
    --serial FILE   also write the serial console to a file
    --no-verify     skip the checksum check. Not advised
    --quick         no pauses, for when you are not watching
"""

import argparse
import hashlib
import os
import platform
import shutil
import subprocess
import sys
import tempfile
import textwrap
import time
import urllib.error
import urllib.request

RELEASE = "v0.9.1"
BASE = f"https://nonos.software/releases/{RELEASE}"
ISO = f"nonos-{RELEASE.lstrip('v')}.iso"
SUMS = "SHA256SUMS"

# NONOS is UEFI only, so QEMU has to be given an OVMF build. These are where
# the usual package managers leave one. The first that exists is used.
#
# Most packages now ship the firmware in two pieces: a read-only code image and
# a writable variables image holding the boot entries. A split build cannot be
# passed with -bios, which fails with "could not load PC BIOS" and no
# explanation, so each entry names its variables image and the pair is attached
# as flash instead. Entries with no variables image are whole firmware files and
# still go through -bios.
FIRMWARE_PATHS = [
    ("/opt/homebrew/share/qemu/edk2-x86_64-code.fd", "/opt/homebrew/share/qemu/edk2-i386-vars.fd"),
    ("/usr/local/share/qemu/edk2-x86_64-code.fd", "/usr/local/share/qemu/edk2-i386-vars.fd"),
    ("/usr/share/OVMF/OVMF_CODE_4M.fd", "/usr/share/OVMF/OVMF_VARS_4M.fd"),
    ("/usr/share/OVMF/OVMF_CODE.fd", "/usr/share/OVMF/OVMF_VARS.fd"),
    ("/usr/share/edk2/x64/OVMF_CODE.fd", "/usr/share/edk2/x64/OVMF_VARS.fd"),
    ("/usr/share/edk2-ovmf/x64/OVMF_CODE.fd", "/usr/share/edk2-ovmf/x64/OVMF_VARS.fd"),
    ("/usr/share/ovmf/OVMF.fd", None),
    ("/usr/share/qemu/OVMF.fd", None),
]

INSTALL_HELP = {
    "Darwin": "brew install qemu",
    "Linux": "sudo apt install qemu-system-x86 ovmf\n"
    "        sudo dnf install qemu-system-x86 edk2-ovmf\n"
    "        sudo pacman -S qemu-full edk2-ovmf",
    "Windows": "winget install SoftwareFreedomConservancy.QEMU",
}

B, D, G, R, Y, C, X = (
    "\033[1m",
    "\033[2m",
    "\033[32m",
    "\033[31m",
    "\033[33m",
    "\033[36m",
    "\033[0m",
)

PACE = 0.5


def out(text="", pause=0.0):
    print(text, flush=True)
    if PACE and pause:
        time.sleep(pause * PACE)


def rule():
    out(f"{D}{'─' * 68}{X}")


def heading(number, title):
    out()
    rule()
    out(f"  {B}{number}. {title}{X}")
    rule()
    out(pause=0.3)


def note(text, pause=0.25):
    for line in textwrap.wrap(text, 64):
        out(f"    {D}{line}{X}")
    if pause:
        time.sleep(pause * PACE)


def found(text):
    out(f"    {G}found{X}   {text}", pause=0.2)


def done(text):
    out(f"    {G}done{X}    {text}", pause=0.2)


def field(label, value):
    out(f"    {label:<10}{value}", pause=0.15)


def stop(problem, remedy=None):
    out()
    out(f"    {R}stopped{X}  {problem}")
    if remedy:
        out()
        for line in remedy.splitlines():
            out(f"      {C}{line}{X}")
    out()
    sys.exit(1)


def locate_qemu():
    exe = shutil.which("qemu-system-x86_64")
    if exe:
        return exe
    stop(
        "QEMU is not installed, or not on your PATH",
        INSTALL_HELP.get(platform.system(), "https://www.qemu.org/download/"),
    )


def locate_firmware():
    """Return the firmware code image and its variables image, if it has one."""
    for code, variables in FIRMWARE_PATHS:
        if os.path.exists(code):
            if variables and os.path.exists(variables):
                return code, variables
            if not variables:
                return code, None
    stop(
        "no UEFI firmware found for QEMU",
        "NONOS boots UEFI only, so QEMU needs an OVMF build.\n"
        + INSTALL_HELP.get(platform.system(), ""),
    )


def firmware_arguments(code, variables, workdir):
    """How to hand the firmware to QEMU.

    A split build has to be attached as two flash devices, and the variables
    half has to be writable, so it is copied next to the image first. Passing a
    code-only image with -bios fails with "could not load PC BIOS" and nothing
    else, which is a miserable first thing to meet.
    """
    if not variables:
        return ["-bios", code], "one whole firmware image, given with -bios"
    local = os.path.join(workdir, "ovmf-vars.fd")
    if not os.path.exists(local):
        shutil.copyfile(variables, local)
    return (
        [
            "-drive", f"if=pflash,format=raw,readonly=on,file={code}",
            "-drive", f"if=pflash,format=raw,file={local}",
        ],
        "split into code and variables, attached as flash",
    )


def start_tpm(workdir):
    """Give the machine a software TPM, or return nothing if there is none.

    Without one the boot chain measures nothing and says so: SecureBoot not
    enabled, TPM not available, PlatformKey not verified. Those warnings are
    honest, and they are also avoidable, because a virtual machine can be given
    a real TPM. An emulated part ships with no endorsement key, and a TPM with
    no endorsement key answers every EK read with an error, so it has to be
    provisioned before it is any use.
    """
    if not shutil.which("swtpm") or not shutil.which("swtpm_setup"):
        return [], None, None
    state = os.path.abspath(os.path.join(workdir, "tpm"))
    os.makedirs(state, exist_ok=True)
    # The socket does not live beside the state. A unix socket path is capped
    # at around 104 bytes, and putting it under the working directory puts the
    # whole path of wherever the reader happened to run this inside that
    # budget. One directory deeper and the socket silently fails to appear,
    # which reads as "this machine has no TPM" and sends the reader looking in
    # entirely the wrong place.
    socket_path = os.path.join(
        tempfile.gettempdir() if len(tempfile.gettempdir()) < 20 else "/tmp",
        f"nonos-tpm-{os.getpid()}.sock",
    )
    # An endorsement key certificate needs a local certificate authority on the
    # host. swtpm can write itself one, and without it the boot chain reports
    # the endorsement key as unavailable: measurement works but the hardware
    # identity it is meant to bind to does not. Ask for the authority first,
    # then provision with certificates, and settle for a bare key if the host
    # will not have it.
    subprocess.run(
        ["swtpm_setup", "--create-config-files", "skip-if-exists"],
        capture_output=True, text=True,
    )
    with_certs = subprocess.run(
        ["swtpm_setup", "--tpm2", "--create-ek-cert", "--create-platform-cert",
         "--tpmstate", state, "--overwrite"],
        capture_output=True, text=True,
    )
    if with_certs.returncode != 0:
        bare = subprocess.run(
            ["swtpm_setup", "--tpm2", "--tpmstate", state,
             "--config", "/dev/null", "--overwrite"],
            capture_output=True, text=True,
        )
        if bare.returncode != 0:
            return [], None, None
    process = subprocess.Popen(
        ["swtpm", "socket", "--tpm2", "--tpmstate", f"dir={state}",
         "--ctrl", f"type=unixio,path={socket_path}", "--flags", "startup-clear"],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    for _ in range(50):
        if os.path.exists(socket_path):
            break
        time.sleep(0.1)
    else:
        process.terminate()
        return [], None, None
    return (
        ["-chardev", f"socket,id=chrtpm,path={socket_path}",
         "-tpmdev", "emulator,id=tpm0,chardev=chrtpm",
         "-device", "tpm-crb,tpmdev=tpm0"],
        process,
        socket_path,
    )


def acceleration():
    """What this host can offer, and what it means in plain terms."""
    system = platform.system()
    if system == "Darwin":
        return ["-accel", "hvf", "-cpu", "host"], "hvf, the hypervisor built into macOS"
    if system == "Linux" and os.access("/dev/kvm", os.W_OK):
        return ["-accel", "kvm", "-cpu", "host"], "kvm, the hypervisor built into Linux"
    if system == "Linux" and os.path.exists("/dev/kvm"):
        return [], "none. /dev/kvm exists but you cannot write to it: add yourself to the kvm group"
    if system == "Windows":
        return ["-accel", "whpx"], "whpx, the Windows hypervisor platform"
    return [], "none. It will still boot, just slowly"


def fetch(url, dest, label):
    """Download with a progress bar.

    The user agent is set deliberately. The site sits behind a CDN that
    answers Python's default agent with 403, so a download that works in any
    browser fails here for a reason that has nothing to do with the file.
    """
    request = urllib.request.Request(
        url, headers={"User-Agent": f"nonos-boot/{RELEASE} (python-urllib)"}
    )
    started = time.time()
    try:
        with urllib.request.urlopen(request) as response, open(dest, "wb") as f:
            total = int(response.headers.get("Content-Length") or 0)
            got = 0
            while chunk := response.read(1 << 18):
                f.write(chunk)
                got += len(chunk)
                secs = max(time.time() - started, 0.001)
                if total:
                    pct = got * 100 // total
                    bar = "█" * (pct * 28 // 100)
                    sys.stdout.write(
                        f"\r    {label} {bar:<28} {pct:3d}%  "
                        f"{got / 1048576:6.1f} MB  {got / 1048576 / secs:5.1f} MB/s"
                    )
                else:
                    sys.stdout.write(f"\r    {label} {got / 1048576:6.1f} MB")
                sys.stdout.flush()
    except urllib.error.HTTPError as e:
        stop(f"the server answered {e.code} for {url}", str(e.reason))
    except urllib.error.URLError as e:
        stop(f"could not reach {url}", str(e.reason))
    sys.stdout.write("\r" + " " * 78 + "\r")
    sys.stdout.flush()


def digest(path):
    size = os.path.getsize(path)
    h, read = hashlib.sha256(), 0
    with open(path, "rb") as f:
        while chunk := f.read(1 << 20):
            h.update(chunk)
            read += len(chunk)
            pct = read * 100 // size
            bar = "█" * (pct * 28 // 100)
            sys.stdout.write(f"\r    hashing  {bar:<28} {pct:3d}%")
            sys.stdout.flush()
    sys.stdout.write("\r" + " " * 78 + "\r")
    sys.stdout.flush()
    return h.hexdigest()


def main():
    global PACE
    p = argparse.ArgumentParser(add_help=True)
    p.add_argument("--dir", default="nonos")
    p.add_argument("--iso")
    p.add_argument("--memory", default="4G")
    p.add_argument("--cpus", default="2")
    p.add_argument("--window", action="store_true")
    p.add_argument("--serial")
    p.add_argument("--no-verify", action="store_true")
    p.add_argument("--quick", action="store_true")
    p.add_argument("--line-delay", type=float, default=0.02,
                   help="seconds between serial lines, so a boot can be read")
    p.add_argument("--no-tpm", action="store_true")
    args = p.parse_args()
    if args.quick:
        PACE = 0.0

    out()
    out(f"  {B}NONOS {RELEASE}{X}")
    out(f"  {D}A RAM resident, capability based operating system, booted in a{X}")
    out(f"  {D}virtual machine. Nothing is installed and nothing is kept.{X}", pause=0.6)

    heading(1, "What this needs")
    note(
        "QEMU runs the machine. OVMF is the UEFI firmware inside it, which "
        "NONOS requires because there is no legacy boot path."
    )
    out()
    qemu = locate_qemu()
    found(f"QEMU        {qemu}")
    banner = subprocess.run([qemu, "--version"], capture_output=True, text=True)
    if banner.stdout:
        out(f"    {D}            {banner.stdout.splitlines()[0]}{X}", pause=0.2)
    code, variables = locate_firmware()
    found(f"firmware    {code}")
    if variables:
        out(f"    {D}            variables   {variables}{X}", pause=0.2)
    accel, accel_note = acceleration()
    found(f"speed       {accel_note}")

    heading(2, "The image")
    if args.iso:
        iso, sums = os.path.abspath(args.iso), None
        if not os.path.exists(iso):
            stop(f"{iso} does not exist")
        found(iso)
    else:
        os.makedirs(args.dir, exist_ok=True)
        iso = os.path.join(args.dir, ISO)
        sums = os.path.join(args.dir, SUMS)
        note(f"Downloading from {BASE}", pause=0.3)
        out()
        if os.path.exists(iso):
            found(f"already here  {iso}")
        else:
            fetch(f"{BASE}/{ISO}", iso, "image    ")
            done(f"{os.path.getsize(iso) / 1048576:.0f} MB  {iso}")
        fetch(f"{BASE}/{SUMS}", sums, "checksums")
        done("published checksums")

    heading(3, "Checking it is ours")
    if args.no_verify or not sums:
        out(f"    {Y}skipped{X}  a tampered image would boot and you would not know")
    else:
        expected = None
        for line in open(sums):
            value, _, name = line.strip().partition("  ")
            if name == os.path.basename(iso):
                expected = value
        if not expected:
            stop(f"{SUMS} does not list {os.path.basename(iso)}")
        note("Hashing the file you downloaded and comparing it with the "
             "checksum published beside it.")
        out()
        actual = digest(iso)
        field("published", expected)
        field("yours", actual)
        out()
        if actual != expected:
            stop(
                "this file is not the one we published",
                "Delete it, download it again, and if it fails twice, say so publicly.",
            )
        done("they match")
        note(
            "That proves the bytes are the ones we released. It is the weaker "
            "of the two checks: the image also carries an Ed25519 and an "
            "ML-DSA-65 signature over the kernel, and a proof for every capsule "
            "in it. The bootloader checks those itself, in a moment, and refuses "
            "to run anything that fails.", pause=0.6,
        )

    heading(4, "The machine")
    workdir = args.dir if not args.iso else (os.path.dirname(iso) or ".")
    os.makedirs(workdir, exist_ok=True)
    tpm_args, tpm_process, tpm_socket = ([], None, None) if args.no_tpm else start_tpm(workdir)
    if tpm_args:
        found("TPM         a software TPM, provisioned with an endorsement key")
    else:
        out(f"    {Y}no TPM{X}  install swtpm for measured boot, or pass --no-tpm to silence this")
    firmware_args, firmware_note = firmware_arguments(
        code, variables, workdir
    )
    command = [
        qemu,
        "-m", args.memory,
        "-smp", args.cpus,
        "-machine", "q35",
        *accel,
        *firmware_args,
        "-cdrom", iso,
        "-device", "virtio-net-pci,netdev=n0",
        "-netdev", "user,id=n0",
        "-device", "virtio-rng-pci",
        *tpm_args,
        "-no-reboot",
    ]
    if args.window:
        # The guest picks 1920x1080, so the window opens at that size and fills
        # a laptop screen. zoom-to-fit lets it be dragged to any size, which is
        # the difference between a usable window and one that has to be
        # scrolled. The backend differs per platform and an unknown one is left
        # to QEMU's default rather than guessed at.
        backend = {"Darwin": "cocoa", "Linux": "gtk"}.get(platform.system())
        command += ["-display", f"{backend},zoom-to-fit=on"] if backend else []
        command += ["-serial", f"file:{args.serial}"] if args.serial else ["-serial", "mon:stdio"]
    elif args.serial:
        command += ["-display", "none", "-serial", f"file:{args.serial}"]
    else:
        # No serial option here. The reader below attaches one of its own so it
        # can take the console a line at a time, and QEMU refuses two.
        command += ["-display", "none"]

    for label, why in [
        (f"-m {args.memory}", "RAM. The whole system lives here, so give it room"),
        (f"-smp {args.cpus}", "processors"),
        ("-machine q35", "a modern chipset, which is what the drivers expect"),
        ("-bios ...", "the UEFI firmware found above"),
        ("-cdrom ...", "the image, attached as a DVD and read only"),
        ("virtio-net-pci", "the network card the network stack knows best"),
        ("virtio-rng-pci", "a real source of entropy for the guest"),
        ("tpm-crb", "a real TPM, so measured boot has something to measure"),
        ("-no-reboot", "stop rather than loop if boot is ever refused"),
    ]:
        out(f"    {C}{label:<16}{X}{D}{why}{X}", pause=0.18)
    out()
    note("The whole command, so you can run it yourself without this script:")
    out()
    out(f"    {' '.join(command[:1])} \\")
    for i in range(1, len(command), 2):
        tail = " \\" if i + 2 < len(command) else ""
        out(f"      {' '.join(command[i:i + 2])}{tail}")

    heading(5, "Booting")
    note(
        "Verified boot runs first. It hashes the kernel, checks both "
        "signatures, then attests every capsule as it spawns. On a slow "
        "machine that takes a minute or two and the splash sits at 100 "
        "per cent while it works, which is the system doing exactly what it "
        "promises rather than hanging."
    )
    out()
    out(f"    {D}Serial output follows. Quit with Ctrl+C.{X}")
    rule()
    out()

    if args.window or args.serial:
        try:
            return subprocess.call(command)
        except KeyboardInterrupt:
            out()
            out("    stopped")
            return 0

    # Read the serial console a line at a time so it can be slowed down. A boot
    # that scrolls past faster than it can be read is no use to anyone watching,
    # and this is the part worth watching: every capsule is proved before it is
    # allowed to run, and each one says so.
    process = subprocess.Popen(
        command + ["-serial", "stdio", "-monitor", "none"],
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
    )
    highlights = ("PASS", "ZK-ATTEST", "SECURITY", "Measured", "DISPLAY", "runqueue_ok")
    try:
        for line in process.stdout:
            line = line.rstrip("\n")
            if any(word in line for word in highlights):
                out(f"  {G}{line}{X}")
            elif line.startswith("[ERROR]") or "FAIL" in line:
                out(f"  {R}{line}{X}")
            elif line.startswith("[WARN]"):
                out(f"  {Y}{line}{X}")
            else:
                out(f"  {line}")
            if args.line_delay:
                time.sleep(args.line_delay)
    except KeyboardInterrupt:
        pass
    finally:
        process.terminate()
        if tpm_process:
            tpm_process.terminate()
        if tpm_socket and os.path.exists(tpm_socket):
            os.unlink(tpm_socket)
    out()
    out("    stopped")
    return 0


if __name__ == "__main__":
    sys.exit(main())
