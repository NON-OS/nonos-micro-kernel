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

use super::types::{Prefs, Project, MAX_PROJECTS, PATH_CAP};

/// One length byte then the fixed path buffer, so a slot's offset is a
/// multiplication rather than a walk over the ones before it.
pub const SLOT: usize = 1 + PATH_CAP;
pub const BYTES: usize = 1 + MAX_PROJECTS * SLOT;

pub fn encode_into(p: &Prefs, out: &mut [u8]) {
    if out.len() < BYTES {
        return;
    }
    let n = (p.project_count as usize).min(MAX_PROJECTS);
    out[0] = n as u8;
    for i in 0..n {
        let at = 1 + i * SLOT;
        let bytes = p.projects[i].as_bytes();
        out[at] = bytes.len() as u8;
        out[at + 1..at + 1 + bytes.len()].copy_from_slice(bytes);
    }
}

/// Total: a short or absent blob yields no projects rather than a panic, so an
/// older record still loads its head fields.
pub fn decode_from(b: &[u8]) -> ([Project; MAX_PROJECTS], u8) {
    let mut out = [Project::default(); MAX_PROJECTS];
    if b.len() < BYTES {
        return (out, 0);
    }
    let n = (b[0] as usize).min(MAX_PROJECTS);
    let mut kept = 0usize;
    for i in 0..n {
        let at = 1 + i * SLOT;
        let len = (b[at] as usize).min(PATH_CAP);
        let slot = Project::new(&b[at + 1..at + 1 + len]);
        if slot.len > 0 {
            out[kept] = slot;
            kept += 1;
        }
    }
    (out, kept as u8)
}
