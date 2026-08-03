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

pub fn client_flight(host: &[u8]) -> Option<ClientFlight> {
    let mut private = [0u8; 32];
    let mut public = [0u8; 32];
    let mut random = [0u8; 32];
    let mut session = [0u8; 32];
    if nonos_libc::crypto_random(private.as_mut_ptr(), private.len()) != private.len() as i64 {
        return None;
    }
    if nonos_libc::crypto_random(random.as_mut_ptr(), random.len()) != random.len() as i64 {
        return None;
    }
    if nonos_libc::crypto_random(session.as_mut_ptr(), session.len()) != session.len() as i64 {
        return None;
    }
    if nonos_libc::crypto_x25519_public(private.as_ptr(), public.as_mut_ptr()) != 32 {
        return None;
    }
    let handshake = super::client_hello::client_hello(host, &random, &session, &public);
    let record = super::record::handshake_record(&handshake);
    Some(ClientFlight { record, handshake, private })
}
