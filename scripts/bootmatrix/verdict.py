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
"""What a serial log has to say for the boot to count.

The markers are the kernel's own lines, the same ones nonos-verify reads. A
cell with more than one CPU must hear from every one of them; a cell with an
IOMMU must not hear that DMA is unrestricted. Every cell must see the ring-0
restrictions read back on: SMEP, SMAP, NX and WP, from the kernel's own
[CPU-PROT] line rather than from what the code asked the part for.
"""

import re

STORE_SERVING = "[VFS] serving, store status "
DESKTOP = ("[COMPOSITOR] capsule spawned", "[WM] capsule spawned", "[desktop_shell]", "[compositor]")
READY = ["Handoff OK", "Capsules spawned", STORE_SERVING]
FATAL = ("[FATAL]", "[PANIC]", "[ZK-ATTEST] FAIL", "[SMP-PROOF] FAIL")
UNRESTRICTED = "DMA is unrestricted"
REMAPPED = "[VT-D] enumerated devices identity mapped; others denied"
SMP_PROOF = re.compile(r"\[SMP-PROOF\] cpu_count=(\d+) PASS")
STORE_STATUS = re.compile(r"\[VFS\] serving, store status ([0-9a-f]{2})")
CPU_PROT = re.compile(r"\[CPU-PROT\] smep=(\d) smap=(\d) umip=(\d) nx=(\d) wp=(\d)")
PROTECTED = {"smep": 0, "smap": 1, "nx": 3, "wp": 4}
# Every guard under a kernel stack armed, or a stack overflows into the next.
GUARDS = re.compile(r"\[STACK-GUARD\] bsp armed (\d+)/(\d+)")


def judge(cell, text, reached, ending):
    """Every reason the boot fails, in the order a reader would check them."""
    bad = []
    if not reached:
        bad.append(f"did not reach readiness: {ending}")
    for word in FATAL:
        if word in text:
            bad.append(f"log carries {word}")
    if reached and not any(d in text for d in DESKTOP):
        bad.append("no desktop capsule reported itself")
    status = STORE_STATUS.search(text)
    if status and status.group(1) != "00":
        bad.append(f"store status {status.group(1)}, not 00")
    if cell.wants_smp_proof():
        proof = SMP_PROOF.search(text)
        if not proof:
            bad.append("no [SMP-PROOF] PASS line")
        elif int(proof.group(1)) != cell.cpus:
            bad.append(f"{proof.group(1)} CPUs online, {cell.cpus} given")
    prot = CPU_PROT.search(text)
    if reached and not prot:
        bad.append("no [CPU-PROT] line")
    if prot:
        bad.extend(f"{name} read back off" for name, i in PROTECTED.items() if prot.group(i + 1) != "1")
    guards = GUARDS.search(text)
    if reached and not guards:
        bad.append("no [STACK-GUARD] line")
    if guards and guards.group(1) != guards.group(2):
        bad.append(f"only {guards.group(1)} of {guards.group(2)} stack guards armed")
    if cell.iommu:
        if UNRESTRICTED in text:
            bad.append("kernel reports DMA is unrestricted with an IOMMU present")
        if REMAPPED not in text:
            bad.append("VT-d never reported devices remapped")
    return bad
