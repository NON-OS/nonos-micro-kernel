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

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use crate::snake::state::Game;

use super::{codec, encode, gate, paths};

// Written once, at the transition into Over, never per tick. Every step is
// dropped on failure: the run stays in memory for this window's lifetime and
// nothing on the frame path ever waits on the outcome.
pub fn save_from(game: &Game) {
    if !gate::live() {
        return;
    }
    let pid = mk_getpid();
    if pid == 0 {
        return;
    }
    if let Err(err) = vfs::mkdir(pid, paths::DIR) {
        gate::note(err);
    }
    if !gate::live() {
        return;
    }
    write(pid, paths::RANKS, &encode::runs(&game.runs));
    let kept = game.awards.len().min(codec::MAX_AWARDS);
    write(pid, paths::AWARDS, &encode::awards(&game.awards[..kept]));
}

// The owner pid is this window's own: the vfs server rejects a claimed owner
// pid that differs from the real sender pid.
fn write(pid: u32, path: &[u8], data: &[u8]) {
    if let Err(err) = vfs::write_file(pid, path, data) {
        gate::note(err);
        return;
    }
    if let Err(err) = vfs::persist(pid, path) {
        gate::note(err);
    }
}
