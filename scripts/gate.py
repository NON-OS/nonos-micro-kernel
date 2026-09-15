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
"""A shrink-only baseline, shared by the hygiene gates.

A gate scans the tree for sites of something the tree should not have and
compares them with a committed list. A site may leave the list when the code
behind it is fixed; none may join it. --write-baseline records the current set,
which is how a gate is introduced over an existing tree without pretending the
tree is clean.
"""

import argparse
from collections import Counter
from pathlib import Path


def identity(entry):
    """What makes two sites the same site across edits.

    A baseline entry is `path:line`, optionally followed by a name. The line
    number is deliberately not part of the identity: editing anything above a
    site moves it, and a gate that calls that a new site fails on changes with
    nothing to do with it. A gate that cries wolf gets switched off, which is
    worse than not having one.

    A site is therefore identified by its file and, where the scan knows one,
    its name. Where it does not, the file alone identifies it and the count is
    what holds the line: a second stub in a file that already had one still
    reports, because the file then carries two.
    """
    site, _, label = entry.partition(" ")
    path = site.rsplit(":", 1)[0] if ":" in site else site
    return f"{path} {label}".rstrip()


def parser(doc, self_test=True):
    ap = argparse.ArgumentParser(description=doc.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--write-baseline", action="store_true")
    if self_test:
        ap.add_argument("--self-test", action="store_true", help="prove the scan bites on a decoy")
    return ap


def run(name, phrase, sites, baseline, args):
    """Exit status for a gate: 1 when a site is new against the baseline.

    `sites(root)` lists the current sites; `phrase` completes the sentence
    printed for each new one, as in "new lint switch at".
    """
    current = sites(args.root)
    path = args.root / baseline
    if args.write_baseline:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("\n".join(current) + "\n")
        print(f"{name}: baseline written, {len(current)} sites")
        return 0
    # One entry per line, not per whitespace-separated word: an entry may
    # carry a name beside its site, and splitting on spaces tore those in two
    # so that every site read as new and the whole baseline read as closed.
    known = path.read_text().splitlines() if path.exists() else []

    # Compared by identity rather than by text, so a site that moved is the
    # same site, and counted per identity, so a file that gains a second site
    # of the same name still reports one as new.
    known_count = Counter(identity(e) for e in known if e.strip())
    new = []
    for entry in current:
        key = identity(entry)
        if known_count.get(key, 0) > 0:
            known_count[key] -= 1
        else:
            new.append(entry)
    for s in sorted(new):
        print(f"{name}: {phrase} {s}")
    closed = sum(n for n in known_count.values() if n > 0)
    print(f"{name}: {len(current)} sites, {closed} closed since the baseline, {len(new)} new")
    return 1 if new else 0
