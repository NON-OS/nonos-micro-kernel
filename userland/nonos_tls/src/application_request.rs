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

use super::traffic_keys::TrafficKeys;

// Seal one request as an application record at the client's next sequence
// number, for a follow-up request on an established connection. The first
// request rides the handshake flight at sequence zero; each request after it
// advances the sequence by one.
pub fn application_request(app: &TrafficKeys, seq: u64, body: &[u8]) -> Option<Vec<u8>> {
    super::record_seal::seal(app.suite, &app.client_key, &app.client_iv, seq, 23, body)
}
