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
"""One boot: start QEMU, watch the serial log, stop it.

The boot is over when every readiness marker has appeared or the budget runs
out. A kill boot ends earlier: as soon as the store reports itself serving,
QEMU dies without warning, and the caller boots the same disk again.
"""

import random
import subprocess
import time

from .verdict import READY, STORE_SERVING

POLL = 0.5


def read_log(path):
    try:
        return path.read_text(errors="replace")
    except FileNotFoundError:
        return ""


def wait_for(log, markers, deadline):
    """The log text once every marker is in it, or whatever is there at the deadline."""
    while True:
        text = read_log(log)
        if all(m in text for m in markers):
            return text, True
        if time.monotonic() >= deadline:
            return text, False
        time.sleep(POLL)


def boot(argv, log, timeout, kill_at_serving=False):
    """Run QEMU to readiness. Returns (log text, reached, how it ended)."""
    log.unlink(missing_ok=True)
    proc = subprocess.Popen(argv, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    deadline = time.monotonic() + timeout
    try:
        if kill_at_serving:
            text, reached = wait_for(log, [STORE_SERVING], deadline)
            if reached:
                # Inside the first seconds of store traffic, off any boundary.
                time.sleep(random.uniform(0.0, 2.0))
                proc.kill()
                return read_log(log), True, "killed while serving"
            return text, False, "store never served"
        text, reached = wait_for(log, READY, deadline)
        return text, reached, "ready" if reached else "timed out"
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(10)
            except subprocess.TimeoutExpired:
                proc.kill()
        stderr = proc.stderr.read().decode(errors="replace").strip()
        if stderr:
            log.with_suffix(".qemu-stderr").write_text(stderr + "\n")
