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

use crate::sys::serial;

pub(super) fn init_token_signing_key() {
    let mut key = [0u8; 32];
    if crate::crypto::random_api::get_bytes_secure(&mut key).is_err() {
        serial::println(b"[FATAL] token signing key init failed: RNG not ready");
        crate::arch::halt_loop();
    }
    if let Err(msg) = crate::capabilities::token::set_signing_key(&key) {
        serial::print(b"[FATAL] token signing key init failed: ");
        serial::println(msg.as_bytes());
        crate::arch::halt_loop();
    }
    crate::crypto::util::constant_time::secure_zero(&mut key);
    serial::println(b"[NONOS] token signing key latched");
}
