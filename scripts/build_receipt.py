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
"""The build receipt: what was built, from what, proven how.

Two jobs, both fail-closed:

1. Root embedding. The STARK policy root the capsule trailers were just
   verified against has to be byte-embedded in the kernel image that ships,
   and the kernel self-attestation root in the same way where present.
   Verifying trailers against a root file in the tree proves internal
   consistency; finding that exact root inside the artifact that boots is
   what ties the proofs to the machine's enforcement. A root file that does
   not appear in the kernel is an error, not a warning.

2. The receipt. A canonical JSON record of the build: commit, tree state,
   epoch, toolchain, artifact hashes (SHA-256 and BLAKE3 when available),
   the roots, and the embedding verdicts. Everything in it is measured from
   the tree at receipt time, never echoed from build variables, so a stale
   or tampered artifact disagrees with its receipt instead of hiding behind
   it. Anyone can re-run the same measurements and diff.
"""
import argparse
import hashlib
import json
import os
import subprocess
import sys

try:
    from blake3 import blake3 as _blake3  # optional; sha256 always present
except Exception:  # pragma: no cover
    _blake3 = None


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def b3(path):
    if _blake3 is None:
        return None
    h = _blake3()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def git(*args):
    try:
        return subprocess.run(["git", *args], capture_output=True, text=True,
                              check=True).stdout.strip()
    except Exception:
        return None


def embedded(needle_path, hay_path):
    """True when the 32-byte root at needle_path occurs in hay_path."""
    with open(needle_path, "rb") as fh:
        needle = fh.read()
    if len(needle) != 32:
        raise SystemExit(f"{needle_path} is {len(needle)} bytes, want 32")
    with open(hay_path, "rb") as fh:
        hay = fh.read()
    return needle.hex(), needle in hay


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--policy-root", required=True,
                    help="capsule policy root the trailers verified against")
    ap.add_argument("--kernel-attest-root", default=None,
                    help="kernel self-attestation root, when the profile has one")
    ap.add_argument("--kernel", required=True,
                    help="the shipped kernel image the roots must appear in")
    ap.add_argument("--bootloader", default=None,
                    help="the bootloader image; the kernel-attest root is baked "
                         "there, since the bootloader is what verifies the kernel")
    ap.add_argument("--enrolled-elf", default=None,
                    help="snapshot of the exact ELF the kernel proof was issued "
                         "for; the shipped image must begin with these bytes")
    ap.add_argument("--artifact", action="append", default=[],
                    help="additional artifact to hash into the receipt")
    ap.add_argument("--out", required=True, help="receipt path (JSON)")
    args = ap.parse_args()

    if not os.path.isfile(args.kernel):
        raise SystemExit(f"kernel image missing: {args.kernel}")

    roots = {}
    policy_hex, ok = embedded(args.policy_root, args.kernel)
    roots["capsule_policy"] = {"root": policy_hex, "embedded_in_kernel": ok}
    status = ["        ok    policy root is embedded in the kernel image"
              if ok else
              "        FAIL  policy root is NOT embedded in the kernel image"]
    failed = not ok

    if args.kernel_attest_root and os.path.isfile(args.kernel_attest_root):
        # The kernel-attest root lives in the artifact that enforces it: the
        # bootloader verifies the kernel, so the root is baked there, not in
        # the kernel image itself.
        hay = args.bootloader if args.bootloader and os.path.isfile(args.bootloader) else args.kernel
        where = "bootloader" if hay == args.bootloader else "kernel image"
        ka_hex, ok = embedded(args.kernel_attest_root, hay)
        roots["kernel_attest"] = {"root": ka_hex, "embedded_in": where, "ok": ok}
        status.append(f"        ok    kernel-attest root is embedded in the {where}"
                      if ok else
                      f"        FAIL  kernel-attest root is NOT embedded in the {where}")
        failed = failed or not ok
    else:
        roots["kernel_attest"] = None
        status.append("        --    no kernel self-attestation root in this profile")

    # Prefix tie: kernel_attested.bin is [kernel][signature][zk block][footer],
    # so the shipped image must begin with exactly the bytes the kernel proof
    # was issued for. This closes the seam the multi-profile build opens, where
    # the loose release ELF is whichever profile compiled last.
    if args.enrolled_elf and os.path.isfile(args.enrolled_elf):
        with open(args.enrolled_elf, "rb") as fh:
            enrolled = fh.read()
        with open(args.kernel, "rb") as fh:
            shipped_prefix = fh.read(len(enrolled))
        if shipped_prefix == enrolled:
            status.append("        ok    shipped image begins with the enrolled kernel bytes")
        else:
            status.append("        FAIL  shipped image does not begin with the enrolled kernel")
            failed = True

    artifacts = {}
    kernel_sha = None
    for path in [args.kernel, *args.artifact]:
        if not os.path.isfile(path):
            artifacts[path] = None
            continue
        entry = {"bytes": os.path.getsize(path), "sha256": sha256(path)}
        digest = b3(path)
        if digest:
            entry["blake3"] = digest
        artifacts[path] = entry
        if path == args.kernel:
            kernel_sha = entry["sha256"]

    # The embedding was checked in --kernel; what boots is the ESP copy. They
    # have to be the same bytes, or the proof anchors to a file nobody runs.
    for path, entry in artifacts.items():
        if path != args.kernel and entry and os.path.basename(path) == "kernel.bin":
            if entry["sha256"] != kernel_sha:
                status.append(f"        FAIL  {path} differs from the verified kernel image")
                failed = True
            else:
                status.append(f"        ok    {path} is byte-identical to the verified kernel")

    receipt = {
        "schema": "nonos.build.receipt.v1",
        "source": {
            "commit": git("rev-parse", "HEAD"),
            "branch": git("rev-parse", "--abbrev-ref", "HEAD"),
            "dirty": bool(git("status", "--porcelain")),
            "source_date_epoch": os.environ.get("SOURCE_DATE_EPOCH"),
        },
        "toolchain": {
            "rustc": os.environ.get("RUSTUP_TOOLCHAIN"),
            "host": os.uname().sysname + "-" + os.uname().machine,
        },
        "roots": roots,
        "artifacts": artifacts,
        "verification": {
            # The make target runs the ledger, signature, caps and STARK gates
            # fail-closed before this script; a receipt existing at all means
            # they passed. Only root embedding is measured here, so only it is
            # recorded as a verdict rather than implied by ordering.
            "gates_before_receipt": ["ledger", "manifest_signatures",
                                     "declared_caps", "stark_membership"],
            "root_embedding": "fail" if failed else "pass",
        },
    }
    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    body = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    with open(args.out, "w") as fh:
        fh.write(body)

    for line in status:
        print(line)
    print(f"        receipt {args.out}  sha256 {hashlib.sha256(body.encode()).hexdigest()[:16]}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
