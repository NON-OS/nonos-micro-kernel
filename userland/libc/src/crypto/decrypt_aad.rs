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

use crate::syscall::{call_raw, N_CRYPTO_DECRYPT_AAD};

#[no_mangle]
pub extern "C" fn crypto_decrypt_aad(
    algo: u64,
    key: *const u8,
    nonce: *const u8,
    frame: *const u8,
    frame_len: usize,
    plaintext: *mut u8,
) -> i64 {
    call_raw(
        N_CRYPTO_DECRYPT_AAD,
        [algo, key as u64, nonce as u64, frame as u64, frame_len as u64, plaintext as u64],
    )
}
