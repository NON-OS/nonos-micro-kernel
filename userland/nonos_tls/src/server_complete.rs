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

pub struct ServerComplete {
    pub handshake: super::traffic_keys::TrafficKeys,
    pub app: super::traffic_keys::TrafficKeys,
    pub transcript: Vec<u8>,
}

pub fn server_complete(
    client: &ClientFlight,
    bytes: &[u8],
    host: &[u8],
    now: u64,
) -> Option<ServerComplete> {
    let mut ctx = super::server_keys::server_keys(client, bytes)?;
    let mut pos = ctx.used;
    let mut seq = 0u64;
    let mut msgs: Vec<u8> = Vec::new();
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            return None;
        }
        if bytes[pos] == 23 {
            let plain = super::record_open::open(
                ctx.keys.suite,
                &ctx.keys.server_key,
                &ctx.keys.server_iv,
                seq,
                &bytes[pos..end],
            )?;
            if let Some((inner, 22)) = super::inner_plain::split(&plain) {
                msgs.extend_from_slice(inner);
            }
            seq += 1;
        }
        pos = end;
    }
    let mut scan = super::scan_server_finished::ScanState {
        secret: &ctx.keys.server_secret,
        transcript: &mut ctx.transcript,
        host,
        now,
        cert11: &mut ctx.cert11,
        validated: &mut ctx.validated,
    };
    let ok = super::scan_server_finished::scan(&msgs, &mut scan);
    if !ok {
        return None;
    }
    let app = super::app_keys::app_keys(&ctx.keys, &ctx.transcript)?;
    Some(ServerComplete { handshake: ctx.keys, app, transcript: ctx.transcript })
}
