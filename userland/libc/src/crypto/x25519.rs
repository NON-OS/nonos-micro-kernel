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

use crate::syscall::{call_raw, N_CRYPTO_X25519_PUBLIC, N_CRYPTO_X25519_SHARED};

#[no_mangle]
pub extern "C" fn crypto_x25519_public(private: *const u8, out: *mut u8) -> i64 {
    call_raw(N_CRYPTO_X25519_PUBLIC, [private as u64, out as u64, 0, 0, 0, 0])
}

#[no_mangle]
pub extern "C" fn crypto_x25519_shared(private: *const u8, public: *const u8, out: *mut u8) -> i64 {
    call_raw(N_CRYPTO_X25519_SHARED, [private as u64, public as u64, out as u64, 0, 0, 0])
}
