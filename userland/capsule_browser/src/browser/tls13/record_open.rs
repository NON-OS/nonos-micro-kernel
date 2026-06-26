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

use alloc::vec::Vec;

pub fn open(key: &[u8; 32], iv: &[u8; 12], seq: u64, record: &[u8]) -> Option<Vec<u8>> {
    if record.len() < 22 || record.first() != Some(&23) {
        return None;
    }
    let len = super::read::u16_at(record, 3)? as usize;
    if len + 5 != record.len() || len <= 16 {
        return None;
    }
    let nonce = super::nonce::nonce(iv, seq);
    let frame = super::aad_frame::aad_frame(&record[..5], &record[5..]);
    let mut pt = Vec::new();
    pt.resize(len - 16, 0);
    let n = nonos_libc::crypto_decrypt_aad(
        0,
        key.as_ptr(),
        nonce.as_ptr(),
        frame.as_ptr(),
        frame.len(),
        pt.as_mut_ptr(),
    );
    if n == pt.len() as i64 { Some(pt) } else { None }
}
