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

use core::hint::spin_loop;
use core::sync::atomic::{fence, Ordering};

use super::pure::{is_stable, read_valid};
use super::state::SeqLock;

impl<T: Copy> SeqLock<T> {
    /// Read a consistent snapshot. The sequence is sampled before and after the
    /// copy; the read retries while a writer holds the lock or races it, so a
    /// torn value is never returned.
    pub fn read(&self) -> T {
        loop {
            let before = self.seq.load(Ordering::Acquire);
            if !is_stable(before) {
                spin_loop();
                continue;
            }
            let value = unsafe { *self.data.get() };
            fence(Ordering::Acquire);
            let after = self.seq.load(Ordering::Acquire);
            if read_valid(before, after) {
                return value;
            }
            spin_loop();
        }
    }
}
