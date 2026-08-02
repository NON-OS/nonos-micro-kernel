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

//! Build a request frame: seq, op, reserved. Every op this client sends carries
//! an empty body.

use super::constants::{HDR_LEN, SEQ};

pub(super) fn build(op: u16) -> [u8; HDR_LEN] {
    let mut out = [0u8; HDR_LEN];
    out[0..4].copy_from_slice(&SEQ.to_le_bytes());
    out[4..6].copy_from_slice(&op.to_le_bytes());
    out
}
