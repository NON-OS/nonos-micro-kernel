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

use crate::syscall::{call_raw, N_MK_SERVICE_REGISTER};

// Anchor the caller as the owner of the named service on `port`. The
// kernel binds (name, port, current_pid) into the service registry so
// peers can resolve it via `mk_service_lookup` without hardcoding the
// wire-side endpoint number. Returns 0 on success or a negative errno.
#[no_mangle]
pub extern "C" fn mk_service_register(name: *const u8, name_len: usize, port: u32) -> i64 {
    call_raw(
        N_MK_SERVICE_REGISTER,
        [name as u64, name_len as u64, port as u64, 0, 0, 0],
    )
}
