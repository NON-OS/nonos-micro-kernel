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

use crate::crypto::keys::NONOS_SIGNING_KEY;
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
    let pk_result = VerifyingKey::from_bytes(NONOS_SIGNING_KEY);
    let ok = pk_result.is_ok();
    log_debug("crypto", &format!("Ed25519 health check: {}", ok));
    ok
}

pub fn run_all_crypto_health_checks() -> (bool, bool) {
    let blake3_ok = blake3_health_check();
    let ed25519_ok = ed25519_health_check();
    (blake3_ok, ed25519_ok)
}
