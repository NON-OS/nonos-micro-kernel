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
    b"bench",
    b"version",
    b"receipt",
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
    b"type",
    b"which",
    b"tree",
    // Reachable at the prompt and, until this line, never offered by Tab.
    b"jobs",
    b"fg",
    b"bg",
    b"rmdir",
    b"neofetch",
    b"commands",
    b"profile",
    b"theme",
    b"tac",
    b"rev",
];

/// Whether the shell answers to this name, by the same table Tab completes
/// from. One list, so `type` can never claim a command that completion does
/// not offer, or the reverse.
pub fn is_command_name(name: &[u8]) -> bool {
    COMMANDS.contains(&name)
}

/// Commands whose argument is another command's name rather than a path.
///
/// Without this, `help gr<Tab>` looks for a file called `gr`, finds nothing,
/// and appears broken. The completion a reader gets has to match what the
/// command actually takes, or Tab teaches them not to press it.
const TAKES_COMMAND: &[&[u8]] = &[b"help", b"commands", b"type", b"which"];

pub fn takes_command_argument(first: &[u8]) -> bool {
    TAKES_COMMAND.contains(&first)
}

/// Every name that would run: shell commands and installed tools alike.
///
/// One source for completion, for `type`, and for the suggestion made when a
/// name is not found, so all three agree about what exists.
pub fn all_names() -> Vec<&'static [u8]> {
    COMMANDS
        .iter()
        .copied()
        .chain(crate::command::builtin::tool::TOOLS.iter().map(|(typed, _)| *typed))
        .collect()
}

/// Command names, plus the installed tools.
///
/// The tools are read from their own table rather than copied into `COMMANDS`,
/// because a second list of the same names is a list that will disagree with
/// the first one eventually. A tool that can be run and cannot be completed is
/// a tool nobody finds.
pub(super) fn command_candidates(prefix: &[u8]) -> Vec<&'static [u8]> {
    COMMANDS
        .iter()
        .copied()
        .chain(crate::command::builtin::tool::TOOLS.iter().map(|(typed, _)| *typed))
        .filter(|c| c.starts_with(prefix))
        .collect()
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
