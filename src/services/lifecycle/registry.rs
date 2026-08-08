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

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::state::CapsuleState;

// `state` is the authority for pid/generation. Clients use it to reject
// replies from a previous capsule instance. The supervisor poll
// (`tick`) calls `is_alive`, which walks the process table and clears
// the stored pid when the capsule has exited so the next IPC observes
// `Dead` deterministically.
#[derive(Clone, Copy)]
pub struct Capsule {
    pub name: &'static str,
    pub state: &'static CapsuleState,
}

static REGISTRY: Mutex<Vec<Capsule>> = Mutex::new(Vec::new());

pub fn register(c: Capsule) {
    let mut g = REGISTRY.lock();
    if let Some(slot) = g.iter_mut().find(|e| e.name == c.name) {
        *slot = c;
        return;
    }
    g.push(c);
}

pub fn tick() {
    let g = REGISTRY.lock();
    for c in g.iter() {
        let _ = c.state.is_alive();
    }
}
