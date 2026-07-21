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

pub fn finished_self_check() -> bool {
    let secret = [0x21u8; 32];
    let transcript = b"server finished transcript";
    let Some(hash) = super::hash_sha256::hash_sha256(transcript) else { return false };
    let Some(key) = super::finished_key::finished_key(&secret) else { return false };
    let mut verify = [0u8; 32];
    let n = nonos_libc::crypto_hmac_sha256(
        key.as_ptr(),
        key.len(),
        hash.as_ptr(),
        hash.len(),
        verify.as_mut_ptr(),
    );
    n == 32 && super::finished_verify::verify(&secret, transcript, &verify)
}
