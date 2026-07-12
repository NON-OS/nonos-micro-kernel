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

use core::sync::atomic::Ordering;

use super::pure::bump;
use super::state::SeqLock;

impl<T> SeqLock<T> {
    /// Publish a new value. The sequence goes odd for the duration of the
    /// write and back to even on completion, so a reader that observes the odd
    /// sequence or a change across its read retries. Writers must be
    /// serialised by the caller.
    pub fn write(&self, value: T) {
        let start = self.seq.load(Ordering::Relaxed);
        // Odd: a write is in progress.
        self.seq.store(bump(start), Ordering::Release);
        unsafe {
            *self.data.get() = value;
        }
        // Even: the write is complete and the value is consistent.
        self.seq.store(bump(bump(start)), Ordering::Release);
    }
}
