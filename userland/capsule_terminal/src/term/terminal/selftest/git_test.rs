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
//! The `git` builtin driven through the real submit path.
//!
//! Each step runs the command a user would type and grades the terminal's own
//! output, so a capsule that faults or hangs here fails the boot rather than
//! shipping broken.

use crate::term::state::State;

use super::{mark, run_cmd, visible_has};

pub fn run(state: &mut State) {
    t_init(state);
    t_add_and_status(state);
    t_commit_and_log(state);
}

fn t_init(state: &mut State) {
    run_cmd(state, b"git init");
    let created = visible_has(state, b"Initialized empty repository");
    // A second init must refuse rather than clobber the first.
    run_cmd(state, b"git init");
    mark(b"git-init", created && visible_has(state, b"already a repository"));
}

fn t_add_and_status(state: &mut State) {
    run_cmd(state, b"write /gitfile.txt hello");
    run_cmd(state, b"git add /gitfile.txt");
    run_cmd(state, b"git status");
    let staged = visible_has(state, b"Changes to be committed")
        && visible_has(state, b"gitfile.txt")
        && visible_has(state, b"No commits yet");
    mark(b"git-add", staged);
}

fn t_commit_and_log(state: &mut State) {
    run_cmd(state, b"git commit -m first");
    let recorded = visible_has(state, b"[main ");
    run_cmd(state, b"git log");
    let logged = visible_has(state, b"commit ") && visible_has(state, b"first");
    mark(b"git-commit", recorded);
    mark(b"git-log", logged);
}
