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

//! The operations worth measuring, and why each one is here.
//!
//! Every capsule on this machine pays for these thousands of times a second,
//! and until now nobody had measured any of them: the published numbers were
//! build and boot wall clock, which are the easiest to take and say the least.

use nonos_libc::{mk_getpid, mk_uptime_ms, mk_yield};

/// Samples per probe. Enough that a ninety-ninth percentile means something,
/// small enough that the whole run finishes inside a keystroke.
pub const SAMPLES: usize = 2048;

/// Samples for the round trip. Each one wakes another process and comes back,
/// so the same count as a bare syscall would be a visible pause with no more to
/// say: a few hundred already puts a real value at the 99th percentile.
pub const IPC_SAMPLES: usize = 256;

/// Rounds used to find the counter's own cost. The floor of these is what gets
/// subtracted from every sample.
pub const CALIBRATION: u32 = 512;

/// Run one probe body once. Split out so the timing loop holds no branch on
/// which probe it is running.
pub fn fire(which: usize) {
    match which {
        0 => {
            core::hint::black_box(mk_getpid());
        }
        1 => {
            core::hint::black_box(mk_uptime_ms());
        }
        _ => {
            mk_yield();
        }
    }
}
