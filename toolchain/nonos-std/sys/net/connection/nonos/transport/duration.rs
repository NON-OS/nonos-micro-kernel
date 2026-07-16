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

// Convert between an optional Duration and the millisecond deadline the IPC
// call takes. None maps to 0 ("no deadline"); a nonzero sub-millisecond
// duration rounds up to 1 ms so it never collapses into "no deadline".

use crate::time::Duration;

pub(crate) fn dur_to_ms(t: Option<Duration>) -> u64 {
    match t {
        None => 0,
        Some(d) => {
            let ms = d.as_millis();
            if ms == 0 && !d.is_zero() {
                1
            } else {
                ms.min(u64::MAX as u128) as u64
            }
        }
    }
}

pub(crate) fn ms_to_dur(ms: u64) -> Option<Duration> {
    if ms == 0 {
        None
    } else {
        Some(Duration::from_millis(ms))
    }
}
