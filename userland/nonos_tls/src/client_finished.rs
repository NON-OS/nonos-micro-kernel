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

pub fn client_finished(
    keys: &super::traffic_keys::TrafficKeys,
    transcript: &[u8],
) -> Option<Vec<u8>> {
    let verify = super::finished_value::finished_value(&keys.client_secret, transcript)?;
    let mut msg = Vec::with_capacity(36);
    msg.push(20);
    super::push::u24(&mut msg, verify.len());
    msg.extend_from_slice(&verify);
    super::record_seal::seal(keys.suite, &keys.client_key, &keys.client_iv, 0, 22, &msg)
}
