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

use super::flight::ClientFlight;

pub fn server_encrypted_flight(client: &ClientFlight, bytes: &[u8]) -> bool {
    let Some(ctx) = super::server_keys::server_keys(client, bytes) else { return false };
    let mut pos = ctx.used;
    let mut seq = 0u64;
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            return false;
        }
        if bytes[pos] == 23 {
            return opens_handshake(&ctx.keys.server_key, &ctx.keys.server_iv, seq, &bytes[pos..end]);
        }
        pos = end;
        seq += 1;
    }
    false
}

fn opens_handshake(key: &[u8; 32], iv: &[u8; 12], seq: u64, record: &[u8]) -> bool {
    let Some(plain) = super::record_open::open(key, iv, seq, record) else { return false };
    plain.last() == Some(&super::constants::TLS_HANDSHAKE)
}
