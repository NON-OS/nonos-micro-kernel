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

pub const MAX_SUBSCRIBERS: usize = 16;

pub struct SubscriptionList {
    entries: [u32; MAX_SUBSCRIBERS],
}

impl SubscriptionList {
    pub const fn new() -> Self {
        Self { entries: [0u32; MAX_SUBSCRIBERS] }
    }

    pub fn add(&mut self, pid: u32) -> bool {
        if pid == 0 {
            return false;
        }
        for slot in self.entries.iter() {
            if *slot == pid {
                return true;
            }
        }
        for slot in self.entries.iter_mut() {
            if *slot == 0 {
                *slot = pid;
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries.iter().copied().filter(|p| *p != 0)
    }

    pub fn remove_pid(&mut self, pid: u32) -> bool {
        let mut removed = false;
        for slot in self.entries.iter_mut() {
            if *slot == pid {
                *slot = 0;
                removed = true;
            }
        }
        removed
    }

    pub fn purge_dead(&mut self) -> u32 {
        let mut removed = 0u32;
        for slot in self.entries.iter_mut() {
            if *slot != 0 && !nonos_libc::mk_pid_alive(*slot) {
                *slot = 0;
                removed += 1;
            }
        }
        removed
    }
}
