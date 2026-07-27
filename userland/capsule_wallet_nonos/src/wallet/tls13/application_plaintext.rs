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

use super::flight::ClientFlight;

pub fn application_plaintext(
    client: &ClientFlight,
    flight: &[u8],
    response: &[u8],
) -> Option<Vec<u8>> {
    let done = super::server_complete::server_complete(client, flight)?;
    let mut out = Vec::new();
    let mut pos = 0usize;
    let mut seq = 0u64;
    while pos + 5 <= response.len() {
        let len = u16::from_be_bytes([response[pos + 3], response[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > response.len() {
            return None;
        }
        if response[pos] == 23 {
            append(&mut out, &done.app, seq, &response[pos..end]);
            seq += 1;
        }
        pos = end;
    }
    Some(out)
}

fn append(out: &mut Vec<u8>, keys: &super::traffic_keys::TrafficKeys, seq: u64, record: &[u8]) {
    let Some(plain) = super::record_open::open(&keys.server_key, &keys.server_iv, seq, record)
    else {
        return;
    };
    let Some((body, 23)) = super::inner_plain::split(&plain) else { return };
    out.extend_from_slice(body);
}
