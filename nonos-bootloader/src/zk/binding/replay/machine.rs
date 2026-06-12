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

use spin::Mutex;

const DS_MACHINE_ID: &str = "NONOS:MACHINE:ID:v1";

static MACHINE_ID: Mutex<Option<[u8; 32]>> = Mutex::new(None);

pub fn derive_machine_id(tpm_ek_public: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key(DS_MACHINE_ID);
    h.update(tpm_ek_public);
    *h.finalize().as_bytes()
}

pub fn init_machine_id(tpm_ek_public: &[u8]) {
    let id = derive_machine_id(tpm_ek_public);
    let mut guard = MACHINE_ID.lock();
    *guard = Some(id);
}

pub fn get_machine_id() -> Result<[u8; 32], &'static str> {
    MACHINE_ID.lock().ok_or("Machine ID not initialized - TPM binding required")
}

pub fn get_machine_id_checked() -> Option<[u8; 32]> {
    *MACHINE_ID.lock()
}

pub fn is_machine_id_initialized() -> bool {
    MACHINE_ID.lock().is_some()
}

pub fn verify_machine_id(claimed: &[u8; 32]) -> bool {
    match *MACHINE_ID.lock() {
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
