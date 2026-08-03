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
use super::server_context::ServerContext;

pub fn server_keys(client: &ClientFlight, record: &[u8]) -> Option<ServerContext> {
    let (server_hello, used) = first_handshake(record)?;
    let (suite, peer) = super::server_hello::key_share(server_hello)?;
    let mut shared = [0u8; 32];
    if nonos_libc::crypto_x25519_shared(client.private.as_ptr(), peer.as_ptr(), shared.as_mut_ptr())
        != 32
    {
        return None;
    }
    let mut transcript = Vec::with_capacity(client.handshake.len() + server_hello.len());
    transcript.extend_from_slice(&client.handshake);
    transcript.extend_from_slice(server_hello);
    let keys = super::schedule::handshake_keys(&shared, &transcript, suite)?;
    Some(ServerContext { used, keys, transcript, cert11: Vec::new(), validated: false })
}

fn first_handshake(record: &[u8]) -> Option<(&[u8], usize)> {
    if record.len() < 9 || record[0] != super::constants::TLS_HANDSHAKE {
        return None;
    }
    let len = u16::from_be_bytes([record[3], record[4]]) as usize;
    let msg = super::read::slice(record, 5, len)?;
    if msg.first() == Some(&2) {
        Some((msg, 5 + len))
    } else {
        None
    }
}
