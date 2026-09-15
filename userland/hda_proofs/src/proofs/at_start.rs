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

//! Reading a window at the instant a DMA engine is switched on.
//!
//! An engine start is the one edge in this driver a second thread can observe
//! reliably, because it is a bit that goes up and stays up. Everything else in
//! a bring-up is a straight run of stores that a concurrent reader has no way
//! to interleave with, which is what makes this the only place an ordering
//! claim can be made at all: latch the registers the first time the run bit is
//! seen set, and whatever they hold is what the bus master started on.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nonos_devmodel::FakeBar;

#[derive(Default)]
pub struct AtStart {
    taken: AtomicBool,
    seen: AtomicBool,
    cells: [AtomicU32; 4],
}

impl AtStart {
    /// The `i`th watched register as it read when the engine started.
    pub fn at(&self, i: usize) -> u32 {
        self.cells[i].load(Ordering::Acquire)
    }
}

/// A device that reads `at` the first time `gate` shows `bit`, and no later.
pub fn watch(s: Arc<AtStart>, gate: usize, bit: u8, at: [usize; 4]) -> impl Fn(&FakeBar) + Send {
    move |bar| {
        if bar.wrote8(gate) & bit == 0 || s.taken.swap(true, Ordering::AcqRel) {
            return;
        }
        for (cell, off) in s.cells.iter().zip(at) {
            cell.store(bar.wrote32(off), Ordering::Release);
        }
        s.seen.store(true, Ordering::Release);
    }
}

/// Block until the model has latched, so a test never reads an empty latch
/// because the model thread had not been scheduled yet.
pub fn wait_for(s: &AtStart) {
    let until = Instant::now() + Duration::from_secs(10);
    while !s.seen.load(Ordering::Acquire) {
        assert!(Instant::now() < until, "the model never saw the engine start");
        std::hint::spin_loop();
    }
}
