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

// Build a request frame: the fixed header (magic, version, op, reserved,
// body length) followed by the operation body.

use super::consts::{MAGIC, STATUS_OFF};
use crate::vec::Vec;

pub(crate) fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(STATUS_OFF + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}
