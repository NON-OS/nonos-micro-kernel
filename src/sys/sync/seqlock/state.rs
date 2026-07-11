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

use core::cell::UnsafeCell;
use core::sync::atomic::AtomicU32;

// A sequence lock for read-mostly data. Readers never block and retry on a
// concurrent write, so reads are wait-free in the common case and always
// consistent. Writers must be serialised by the caller (the usual pattern is
// one writer, or a spinlock guarding writes); the sequence goes odd while a
// write is in flight so any reader that observes it retries.
pub struct SeqLock<T> {
    pub(super) seq: AtomicU32,
    pub(super) data: UnsafeCell<T>,
}

// Safe to share: the sequence discipline guarantees a reader never observes a
// half-written value, and `T: Send` is required to move it across threads.
unsafe impl<T: Send> Sync for SeqLock<T> {}
