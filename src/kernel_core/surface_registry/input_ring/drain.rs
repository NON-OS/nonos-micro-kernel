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

use super::super::types::{InputEvent, INPUT_RING_CAP};
use super::ring::{DRAINED, RING};

pub fn drain_input(out: &mut [InputEvent]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut ring = RING.lock();
    let mut n = 0usize;
    while n < out.len() && ring.tail != ring.head {
        out[n] = ring.buf[ring.tail];
        ring.tail = super::super::ring_math::wrap(ring.tail, INPUT_RING_CAP);
        n += 1;
    }
    DRAINED.fetch_add(n as u64, Ordering::Relaxed);
    n
}
