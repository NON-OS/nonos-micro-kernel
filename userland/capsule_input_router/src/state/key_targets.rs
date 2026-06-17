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

// Tracks which capsule received the key-down for each currently held key, so
// the matching key-up is delivered to that same capsule even if window focus
// moved while the key was held. Without this, a release routed by current
// focus lands in the wrong window and the original app sees a stuck key.

const MAX_HELD_KEYS: usize = 16;

#[derive(Clone, Copy)]
struct Held {
    code: u32,
    pid: u32,
}

pub struct KeyTargets {
    held: [Held; MAX_HELD_KEYS],
}

impl KeyTargets {
    pub const fn new() -> Self {
        Self { held: [Held { code: 0, pid: 0 }; MAX_HELD_KEYS] }
    }

    // Record that `pid` received the down for `code`. Updates an existing entry
    // for the same key, else claims a free slot; a full table drops the record
    // (that key-up then falls back to current focus).
    pub fn remember(&mut self, code: u32, pid: u32) {
        if let Some(slot) = self.held.iter_mut().find(|h| h.pid != 0 && h.code == code) {
            slot.pid = pid;
            return;
        }
        if let Some(slot) = self.held.iter_mut().find(|h| h.pid == 0) {
            *slot = Held { code, pid };
        }
    }

    // Return and clear the pid that received the down for `code`, if known.
    pub fn take(&mut self, code: u32) -> Option<u32> {
        let slot = self.held.iter_mut().find(|h| h.pid != 0 && h.code == code)?;
        let pid = slot.pid;
        *slot = Held { code: 0, pid: 0 };
        Some(pid)
    }

    // Drop every entry for a pid that has gone away.
    pub fn forget_pid(&mut self, pid: u32) {
        for h in self.held.iter_mut().filter(|h| h.pid == pid) {
            *h = Held { code: 0, pid: 0 };
        }
    }
}
