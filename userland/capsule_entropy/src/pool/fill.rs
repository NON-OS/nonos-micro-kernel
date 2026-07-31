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

use crate::protocol::MAX_RANDOM_BYTES;

use super::source;
use super::types::Pool;

impl Pool {
    pub fn fill(&self, out: &mut [u8]) -> i64 {
        let cap = MAX_RANDOM_BYTES as usize;
        let want = if out.len() > cap { cap } else { out.len() };
        if want == 0 {
            return 0;
        }
        self.requests.fetch_add(1, Ordering::Relaxed);
        if !unsafe { source::fill(&mut out[..want]) } {
            self.source_failures.fetch_add(1, Ordering::Relaxed);
            return -5;
        }
        self.bytes_served.fetch_add(want as u64, Ordering::Relaxed);
        want as i64
    }
}
