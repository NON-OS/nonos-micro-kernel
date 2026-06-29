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

use crate::protocol::E_BAD_PACKET;
use crate::state::{pop_any, pop_for_protocol, Packet};

pub fn wanted_protocol(body: &[u8]) -> Result<Option<u8>, u16> {
    match body.len() {
        0 => Ok(None),
        1 => Ok(Some(body[0])),
        _ => Err(E_BAD_PACKET),
    }
}

pub fn queued(wanted: Option<u8>) -> Option<Packet> {
    match wanted {
        Some(proto) => pop_for_protocol(proto),
        None => pop_any(),
    }
}
