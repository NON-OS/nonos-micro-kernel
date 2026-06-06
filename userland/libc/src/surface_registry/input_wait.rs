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

use crate::syscall::{call_raw, N_MK_INPUT_EVENT_WAIT};

#[no_mangle]
pub extern "C" fn mk_input_event_wait(last_seq: u64, timeout_ms: u64, out_seq: *mut u64) -> i64 {
    call_raw(N_MK_INPUT_EVENT_WAIT, [last_seq, timeout_ms, out_seq as u64, 0, 0, 0])
}
