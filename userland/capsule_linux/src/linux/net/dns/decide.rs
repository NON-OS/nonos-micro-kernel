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

//! What to answer a query with.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::{automap, name, reply};

/// A record type this capsule can answer with an address.
const TYPE_A: u16 = 1;

pub(super) fn answer(guest: &mut Guest, msg: &[u8]) -> Option<Vec<u8>> {
    let (host, end) = name::read(msg)?;
    let qtype = u16::from_be_bytes([*msg.get(end)?, *msg.get(end + 1)?]);
    let question = msg.get(..end + 4)?;
    if qtype != TYPE_A {
        /*
         * Every other type, AAAA above all, is answered empty rather than
         * ignored.
         */
        return Some(reply::build(question, None));
    }
    Some(reply::build(question, automap::address_for(guest, &host)))
}
