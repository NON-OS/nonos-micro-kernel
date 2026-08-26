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

use crate::syscall::{call_raw, N_MK_SLEEP_MS, N_MK_YIELD};

#[no_mangle]
pub extern "C" fn mk_yield() -> i64 {
    call_raw(N_MK_YIELD, [0; 6])
}

// Park the caller for `ms` milliseconds. Use this instead of a tight
// `mk_yield` backoff loop when waiting for a dependency: yield keeps the task
// runnable and pins the CPU, this releases the core until the wake deadline.
#[no_mangle]
pub extern "C" fn mk_sleep_ms(ms: u64) -> i64 {
    call_raw(N_MK_SLEEP_MS, [ms, 0, 0, 0, 0, 0])
}
