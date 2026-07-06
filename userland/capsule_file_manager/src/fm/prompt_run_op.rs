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

use nonos_app_skeleton::clients::vfs::{mkdir, rename, unlink, write_file};

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
            let sel = state.entries.get(state.cursor).ok_or("no selection")?;
            if sel.is_dir {
                return Err("dirs not supported");
            }
            unlink(pid, sel.full_path.as_bytes()).map(|_| b"deleted".as_slice())
        }
    }
}
