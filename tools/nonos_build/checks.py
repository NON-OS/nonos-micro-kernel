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

"""Step verification. A step counts as done when its artifact exists and
has the expected shape, never because make exited zero."""

import hashlib
import sys
from pathlib import Path


def require(path, what, min_bytes=1):
    p = Path(path)
    if not p.is_file() or p.stat().st_size < min_bytes:
        print(f"  MISSING: {what} expected at {path}")
        sys.exit(1)
    return p


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def show(path, what):
    p = require(path, what)
    print(f"  {what}: {p.stat().st_size} bytes  sha256 {sha256(p)[:16]}…")
    return p
