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

"""Process helpers. Every make invocation is serialized and logged in
full; a failing step surfaces the last lines of its own log instead of
asking the reader to scroll."""

import subprocess
import sys
from pathlib import Path

LOG_DIR = Path("target/build-guide")


def run_make(target, log_name, env=None, extra=()):
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    log = LOG_DIR / f"{log_name}.log"
    cmd = ["make", *extra, target]
    with open(log, "w") as sink:
        proc = subprocess.run(cmd, stdout=sink, stderr=subprocess.STDOUT, env=env)
    if proc.returncode != 0:
        tail = log.read_text(errors="replace").splitlines()[-15:]
        print(f"\n  step failed (exit {proc.returncode}); log: {log}")
        for line in tail:
            print(f"    {line}")
        sys.exit(proc.returncode)
    return log


def out(line=""):
    print(line, flush=True)


def confirm(prompt, assume_yes):
    if assume_yes:
        return
    reply = input(f"  {prompt} [Enter to continue, q to stop] ").strip().lower()
    if reply == "q":
        sys.exit(0)
