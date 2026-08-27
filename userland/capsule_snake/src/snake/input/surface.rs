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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::snake::grid::{WIN_H, WIN_W};

static WIDTH: AtomicU32 = AtomicU32::new(WIN_W);
static HEIGHT: AtomicU32 = AtomicU32::new(WIN_H);

// The painter is the only part of the capsule the runtime hands the live
// surface to, so it publishes the dimensions here and every hit test reads
// them back. The nominal window size stands in until the first frame lands.
pub fn note(w: u32, h: u32) {
    WIDTH.store(w.max(1), Ordering::Relaxed);
    HEIGHT.store(h.max(1), Ordering::Relaxed);
}

pub fn size() -> (u32, u32) {
    (WIDTH.load(Ordering::Relaxed), HEIGHT.load(Ordering::Relaxed))
}
