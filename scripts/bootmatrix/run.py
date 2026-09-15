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
"""A cell, repeated. Each run gets its own copy of the disk and the firmware
variables, so one boot cannot leave state for the next to find."""

import shutil
import time
from dataclasses import dataclass, field

from .boot import boot
from .qemu import accelerator, command
from .verdict import judge


@dataclass
class Paths:
    qemu: str
    esp: dict
    ovmf: str
    ovmf_vars: str
    blk_img: str
    extra: str = ""


@dataclass
class Run:
    cell: str
    attempt: int
    accel: str
    seconds: float
    ending: str
    failures: list = field(default_factory=list)
    log: str = ""


def one(cell, paths, accel, out, attempt, timeout):
    """Boot the cell once. The kill cell boots twice on the same disk."""
    stem = out / f"{cell.name}-{attempt}"
    blk = stem.with_suffix(".img")
    shutil.copyfile(paths.blk_img, blk)
    variables = stem.with_suffix(".vars.fd")
    shutil.copyfile(paths.ovmf_vars, variables)
    log = stem.with_suffix(".log")
    started = time.monotonic()
    if cell.kill:
        first = stem.with_suffix(".killed.log")
        argv = command(cell, paths, accel, first, blk, variables)
        _, served, ending = boot(argv, first, timeout, kill_at_serving=True)
        if not served:
            return Run(cell.name, attempt, accel, time.monotonic() - started, ending,
                       [f"first boot: {ending}"], str(first))
    argv = command(cell, paths, accel, log, blk, variables)
    text, reached, ending = boot(argv, log, timeout)
    failures = judge(cell, text, reached, ending)
    blk.unlink()
    variables.unlink()
    return Run(cell.name, attempt, accel, time.monotonic() - started, ending, failures, str(log))


def cell_runs(cell, paths, requested_accel, out, repeat, timeout):
    accel = accelerator(cell, requested_accel)
    return [one(cell, paths, accel, out, n + 1, timeout) for n in range(repeat)]
