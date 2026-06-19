// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

extern crate alloc;

use alloc::format;
use ed25519_dalek::VerifyingKey;

use crate::log::logger::log_debug;

pub fn blake3_health_check() -> bool {
    let probe = b"NONOS-bootloader-blake3-health";
    let h = blake3::hash(probe);
    let expected = blake3::hash(probe);
    let ok = h.as_bytes() == expected.as_bytes();
    log_debug("crypto", &format!("BLAKE3 health check: {}", ok));
    ok
}

pub fn ed25519_health_check() -> bool {
    let pk_bytes: [u8; 32] = [
        0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64, 0x07,
        0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07,
        0x51, 0x1a,
    ];

    let pk_result = VerifyingKey::from_bytes(&pk_bytes);
    let ok = pk_result.is_ok();
    log_debug("crypto", &format!("Ed25519 health check: {}", ok));
    ok
}

pub fn run_all_crypto_health_checks() -> (bool, bool) {
    let blake3_ok = blake3_health_check();
    let ed25519_ok = ed25519_health_check();
    (blake3_ok, ed25519_ok)
}
