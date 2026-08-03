// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

// Every command name and alias the shell accepts as a first token, used
// for Tab completion of the command word.
const COMMANDS: &[&[u8]] = &[
    b"help",
    b"about",
    b"version",
    b"whoami",
    b"caps",
    b"capsules",
    b"clear",
    b"display",
    b"echo",
    b"history",
    b"market",
    b"motd",
    b"ping",
    b"svc",
    b"service",
    b"nox",
    b"exit",
    b"where",
    b"pwd",
    b"in",
    b"cd",
    b"ls",
    b"dir",
    b"read",
    b"cat",
    b"write",
    b"copy",
    b"cp",
    b"mk",
    b"mkdir",
    b"rm",
    b"del",
    b"mv",
    b"move",
    b"stat",
    b"find",
    b"du",
    b"touch",
    b"basename",
    b"dirname",
    b"date",
    b"env",
    b"ps",
    b"ifconfig",
    b"ip",
    b"nslookup",
    b"nym",
    b"id",
    b"sys",
    b"apps",
    b"grep",
    b"wc",
    b"head",
    b"tail",
    b"cut",
    b"run",
    b"open",
    b"set",
    b"unset",
    b"alias",
    b"unalias",
    b"sort",
    b"uniq",
    b"nl",
    b"install",
];

pub(super) fn command_candidates(prefix: &[u8]) -> Vec<&'static [u8]> {
    COMMANDS.iter().copied().filter(|c| c.starts_with(prefix)).collect()
}

pub(super) fn common_prefix(cands: &[&[u8]]) -> Vec<u8> {
    let Some(first) = cands.first() else {
        return Vec::new();
    };
    let mut p = first.to_vec();
    for c in &cands[1..] {
        let n = p.iter().zip(c.iter()).take_while(|(a, b)| a == b).count();
        p.truncate(n);
    }
    p
}
