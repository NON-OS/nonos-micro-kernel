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

//! Yielding and debug output. Yields are counted rather than taken, so a
//! driver that settles by yielding runs at full speed and a test can still
//! see that it waited.

use std::cell::{Cell, RefCell};

use crate::raw;

thread_local! {
    static YIELDS: Cell<u64> = const { Cell::new(0) };
    static DEBUG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_yield() -> i64 {
    YIELDS.with(|y| y.set(y.get() + 1));
    0
}

/// How many times the driver yielded on this thread so far.
pub fn yields() -> u64 {
    YIELDS.with(Cell::get)
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    let line = String::from_utf8_lossy(raw::bytes(buf, len)).into_owned();
    DEBUG.with(|d| d.borrow_mut().push(line));
    len as i64
}

/// The lines the driver printed, oldest first, leaving the record empty.
pub fn take_debug() -> Vec<String> {
    DEBUG.with(|d| std::mem::take(&mut *d.borrow_mut()))
}
