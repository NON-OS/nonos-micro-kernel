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

use super::types::Prefs;
use super::{codec, gate, paths};

// Best-effort throughout: an absent VFS, a first boot with no file, or a corrupt
// record all yield the defaults, so the terminal paints either way. Called once,
// from `Terminal::new`, never on a frame path.
pub fn load() -> Prefs {
    if !gate::live() {
        return Prefs::default();
    }
    let pid = mk_getpid();
    if pid == 0 {
        return Prefs::default();
    }
    match vfs::read_file(pid, paths::PREFS, paths::MAX_FILE_BYTES) {
        Ok(bytes) => codec::decode(&bytes),
        Err(err) => {
            gate::note(err);
            Prefs::default()
        }
    }
}

// The owner pid is this window's own: the vfs server rejects a claimed owner
// pid that differs from the real sender pid. Every step is dropped on failure,
// so a refused write leaves the live setting untouched for this session.
pub fn save(p: &Prefs) {
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
    if let Err(err) = vfs::write_file(pid, paths::PREFS, &codec::encode(p)) {
        gate::note(err);
        return;
    }
    if let Err(err) = vfs::persist(pid, paths::PREFS) {
        gate::note(err);
    }
}
