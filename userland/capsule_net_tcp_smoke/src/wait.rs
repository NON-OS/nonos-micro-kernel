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

use nonos_libc::{mk_time_millis, mk_yield};

const STEP_MS: i64 = 100;

pub fn poll_until<F: FnMut() -> bool>(timeout_ms: u64, mut f: F) -> bool {
    let start = mk_time_millis().max(0);
    loop {
        if f() {
            return true;
        }
        let now = mk_time_millis().max(0);
        if (now - start) as u64 >= timeout_ms {
            return false;
        }
        let next = now + STEP_MS;
        while mk_time_millis() < next {
            mk_yield();
        }
    }
}
