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

use spin::Mutex;

const DS_NONCE_COMMIT: &str = "NONOS:NONCE:COMMIT:v1";

static BOOT_NONCE: Mutex<Option<[u8; 32]>> = Mutex::new(None);

pub fn init_boot_nonce(entropy: &[u8; 64]) {
    let mut h = blake3::Hasher::new_derive_key(DS_NONCE_COMMIT);
    h.update(entropy);
    let nonce = *h.finalize().as_bytes();
    let mut guard = BOOT_NONCE.lock();
    *guard = Some(nonce);
}

pub fn init_boot_nonce_value(nonce: [u8; 32]) {
    let mut guard = BOOT_NONCE.lock();
    *guard = Some(nonce);
}

pub fn get_boot_nonce() -> Result<[u8; 32], &'static str> {
    BOOT_NONCE.lock().ok_or("Boot nonce not initialized")
}

pub fn get_boot_nonce_checked() -> Option<[u8; 32]> {
    *BOOT_NONCE.lock()
}

pub fn is_nonce_initialized() -> bool {
    BOOT_NONCE.lock().is_some()
}

pub fn verify_nonce_freshness(claimed: &[u8; 32]) -> bool {
    match *BOOT_NONCE.lock() {
        Some(ref current) => ct_eq32(current, claimed),
        None => false,
    }
}

#[inline]
fn ct_eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut x = 0u8;
    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    x == 0
}
