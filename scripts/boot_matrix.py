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
"""Boot the built images across the machine matrix and say which cells fail.

    boot_matrix.py --esp up=target/boot-matrix/esp-up --esp smp=target/boot-matrix/esp-smp \\
        --ovmf /path/OVMF_CODE.fd --ovmf-vars target/qemu-OVMF_VARS.fd \\
        --blk-img target/qemu-virtio-blk.img --repeat 5

A cell passes when every one of its boots reaches readiness with nothing
fatal in the log, all its CPUs online, and DMA restricted when an IOMMU is
present. The exit status is the number of failing boots. `make
nonos-mk-boot-matrix` builds both images and runs this.
"""

import sys

from bootmatrix.cells import CELLS, select
from bootmatrix.cli import arguments, images_for
from bootmatrix.report import failed, table, write_json
from bootmatrix.run import Paths, cell_runs


def main():
    a = arguments(__doc__)
    if a.list:
        print("\n".join(c.name for c in CELLS))
        return 0
    cells = select(a.cells)
    paths = Paths(a.qemu, images_for(a, cells), a.ovmf, a.ovmf_vars, a.blk_img, a.extra)
    a.out.mkdir(parents=True, exist_ok=True)
    runs = []
    for cell in cells:
        runs.extend(cell_runs(cell, paths, a.accel, a.out, a.repeat, a.timeout))
        print(table(runs[-a.repeat:]), flush=True)
    write_json(runs, a.out / "boot-matrix.json")
    print(f"\nboot-matrix: {len(runs) - len(failed(runs))} of {len(runs)} boots passed")
    return len(failed(runs))


if __name__ == "__main__":
    sys.exit(main())
