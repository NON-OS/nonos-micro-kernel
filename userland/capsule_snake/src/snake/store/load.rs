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

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use crate::snake::state::Game;

use super::{decode, gate, paths};

// Career state only, and best-effort throughout: an absent VFS, a missing file
// or a corrupt record all leave the constructed defaults in place, so the game
// is playable either way. Called once, before the first frame, never per tick.
pub fn load_into(game: &mut Game) {
    if !gate::live() {
        return;
    }
    let pid = mk_getpid();
    if pid == 0 {
        return;
    }
    if let Some(bytes) = read(pid, paths::RANKS) {
        if let Ok(runs) = decode::runs(&bytes) {
            game.runs = runs;
            game.runs.sort_by(|a, b| b.score.cmp(&a.score));
        }
    }
    if let Some(bytes) = read(pid, paths::AWARDS) {
        if let Ok(awards) = decode::awards(&bytes) {
            game.awards = awards;
        }
    }
}

// The vfs server rejects a claimed owner pid that differs from the real sender
// pid, so the pid is always this window's own and never a service lookup.
fn read(pid: u32, path: &[u8]) -> Option<Vec<u8>> {
    match vfs::read_file(pid, path, paths::MAX_FILE_BYTES) {
        Ok(bytes) => Some(bytes),
        Err(err) => {
            gate::note(err);
            None
        }
    }
}
