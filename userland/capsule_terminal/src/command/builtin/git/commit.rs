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
//! `git commit -m <message>`

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::{commit, read_index, write_tree, CommitRequest, ObjectId, RepoError, Signature};

use crate::command::output::Output;
use crate::git::VfsStorage;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

pub(super) fn run(state: &mut State, message: &[u8]) {
    if message.is_empty() {
        Output::new(&mut state.scrollback).writeln(b"git commit: missing message");
        return;
    }
    let mut s = storage(state);
    let result = record(&mut s, message);
    let mut out = Output::new(&mut state.scrollback);
    match result {
        Ok(id) => {
            let mut line = Vec::from(&b"[main "[..]);
            line.extend_from_slice(&id.to_hex().as_bytes()[..7]);
            line.extend_from_slice(b"] ");
            line.extend_from_slice(message);
            out.writeln(&line);
        }
        Err(_) => out.writeln(b"git commit: nothing recorded"),
    }
}

fn record(s: &mut VfsStorage, message: &[u8]) -> Result<ObjectId, RepoError> {
    let entries = read_index(s, GIT_DIR)?;
    let tree = write_tree(s, GIT_DIR, &entries)?;
    let who = author();
    let mut text = String::from_utf8_lossy(message).into_owned();
    text.push('\n');
    let request = CommitRequest { tree, author: who.clone(), committer: who, message: text };
    commit(s, GIT_DIR, &request)
}

/// A NONOS session has no per-user git config yet, so the identity is fixed
/// rather than invented per commit.
fn author() -> Signature {
    Signature {
        name: String::from("nonos"),
        email: String::from("user@nonos.systems"),
        when: nonos_libc::mk_time_millis().max(0) as u64 / 1000,
        offset_minutes: 0,
    }
}
