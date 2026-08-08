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

use super::constants::{LEGACY_RECORD_VERSION, TLS_HANDSHAKE};

pub fn handshake_record(handshake: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(handshake.len() + 5);
    out.push(TLS_HANDSHAKE);
    super::push::u16(&mut out, LEGACY_RECORD_VERSION);
    super::push::u16(&mut out, handshake.len() as u16);
    out.extend_from_slice(handshake);
    out
}
