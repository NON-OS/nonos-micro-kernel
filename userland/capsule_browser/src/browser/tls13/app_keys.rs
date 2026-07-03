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

use super::traffic_keys::TrafficKeys;

pub fn app_keys(handshake: &TrafficKeys, transcript: &[u8]) -> Option<TrafficKeys> {
    let zero = [0u8; 32];
    let derived = super::schedule::secret(
        &handshake.handshake_secret,
        b"derived",
        &super::schedule::EMPTY_HASH,
    )?;
    let master = super::hkdf::extract(&derived, &zero)?;
    let th = super::hash_sha256::hash_sha256(transcript)?;
    let client_secret = super::schedule::secret(&master, b"c ap traffic", &th)?;
    let server_secret = super::schedule::secret(&master, b"s ap traffic", &th)?;
    let suite = handshake.suite;
    Some(TrafficKeys {
        suite,
        handshake_secret: master,
        client_secret,
        server_secret,
        client_key: super::schedule::key(&client_secret, suite)?,
        client_iv: super::schedule::iv(&client_secret)?,
        server_key: super::schedule::key(&server_secret, suite)?,
        server_iv: super::schedule::iv(&server_secret)?,
    })
}
