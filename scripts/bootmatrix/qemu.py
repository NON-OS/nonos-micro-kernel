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
"""The QEMU command line for a cell, and the accelerator it can use.

An IOMMU needs the split irqchip, which the macOS hypervisor framework does
not offer, so those cells fall back to TCG there. KVM takes every cell.
"""

import os
import platform
import shlex

IOMMU_OPTS = "intel-iommu,intremap=on,caching-mode=on"


def accelerator(cell, requested):
    """Pick the accelerator: the one asked for, or the fastest that fits."""
    if requested != "auto":
        if requested == "hvf" and cell.iommu:
            raise SystemExit(f"boot-matrix: {cell.name} needs a split irqchip; hvf cannot")
        return requested
    if os.path.exists("/dev/kvm") and os.access("/dev/kvm", os.R_OK | os.W_OK):
        return "kvm"
    if platform.system() == "Darwin" and not cell.iommu:
        return "hvf"
    return "tcg"


def cpu_model(accel):
    return "host,+rdrand,+rdseed" if accel in ("kvm", "hvf") else "max"


def command(cell, paths, accel, serial_log, blk_copy, vars_copy):
    """The argv. `paths` carries qemu, esp per profile, ovmf and extra devices."""
    machine = cell.machine + (",kernel-irqchip=split" if cell.iommu else "")
    argv = [
        paths.qemu, "-m", "2G", "-accel", accel, "-cpu", cpu_model(accel),
        "-smp", str(cell.cpus), "-machine", machine,
        "-drive", f"format=raw,file=fat:rw:{paths.esp[cell.profile]}",
        "-drive", f"if=pflash,format=raw,readonly=on,file={paths.ovmf}",
        "-drive", f"if=pflash,format=raw,unit=1,file={vars_copy}",
        "-drive", f"file={blk_copy},if=none,id=vd0,format=raw",
        "-device", "virtio-blk-pci,drive=vd0",
    ]
    if cell.iommu:
        argv += ["-device", IOMMU_OPTS]
    argv += shlex.split(paths.extra)
    argv += ["-serial", f"file:{serial_log}", "-display", "none", "-no-reboot"]
    return argv
