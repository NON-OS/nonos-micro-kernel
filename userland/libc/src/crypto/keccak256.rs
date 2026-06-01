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

use crate::syscall::{call_raw, N_CRYPTO_KECCAK256};

/// keccak256 over `data`, writing 32 bytes to `out`. `out_len` must be 32.
/// Returns 32 on success, negative errno otherwise.
#[no_mangle]
pub extern "C" fn crypto_keccak256(
    data: *const u8,
    len: usize,
    out: *mut u8,
    out_len: usize,
) -> i64 {
    call_raw(N_CRYPTO_KECCAK256, [data as u64, len as u64, out as u64, out_len as u64, 0, 0])
}
