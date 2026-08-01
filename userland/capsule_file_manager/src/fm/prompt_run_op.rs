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

use nonos_app_skeleton::clients::vfs::{mkdir, rename, rmdir, unlink, write_file};

use super::selection_acting::acting;
use super::state::{PromptKind, State};

pub fn run_op(
    state: &State,
    kind: PromptKind,
    name: &str,
    target: &str,
) -> Result<&'static [u8], &'static str> {
    let pid = state.owner_pid;
    match kind {
        PromptKind::NewFile => {
            write_file(pid, target.as_bytes(), b"").map(|_| b"created".as_slice())
        }
        PromptKind::MkDir => mkdir(pid, target.as_bytes()).map(|_| b"directory created".as_slice()),
        PromptKind::Rename => {
            let old = state.entries.get(state.cursor).ok_or("no selection")?;
            rename(pid, old.full_path.trim_end_matches('/').as_bytes(), target.as_bytes())
                .map(|_| b"renamed".as_slice())
        }
        PromptKind::Delete => {
            if name != "y" {
                return Ok(b"not deleted");
            }
            // Delete what is selected, not just the row under the cursor:
            // copy, cut, duplicate and chmod all act on the selection, and the
            // help text promises "selection or cursor". Directories go through
            // rmdir, which was refused outright before.
            let targets = acting(state);
            if targets.is_empty() {
                return Err("no selection");
            }
            let mut failed = 0usize;
            for (path, is_dir) in &targets {
                let done = if *is_dir {
                    rmdir(pid, path.trim_end_matches('/').as_bytes(), true)
                } else {
                    unlink(pid, path.as_bytes())
                };
                if done.is_err() {
                    failed += 1;
                }
            }
            if failed == 0 {
                Ok(b"deleted")
            } else if failed == targets.len() {
                Err("delete failed")
            } else {
                Ok(b"some items not deleted")
            }
        }
    }
}
