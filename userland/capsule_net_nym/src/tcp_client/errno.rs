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

//! What went wrong with a call to `net.tcp`.
//!
//! Every fault carries its own value. Folding them together names the call
//! that failed but not what was wrong with the answer, and the difference
//! between a service that refused and one that never replied is the
//! difference between two unrelated fixes.

/// The call itself never completed.
pub const E_CALL: u16 = 8;
/// Reply shorter than a header, so nothing could be read from it.
pub const E_SHORT: u16 = 20;
/// Reply carried another service's magic.
pub const E_MAGIC: u16 = 21;
/// Reply answered a different opcode than the one asked.
pub const E_OP: u16 = 22;
/// Reply claimed a payload that does not fit what was sent or asked for.
pub const E_LEN: u16 = 23;
/// net.tcp took no bytes for long enough that waiting stopped being sensible.
pub const E_SHORT_WRITE: u16 = 24;
/// Added to a service errno so it cannot be confused with the above.
pub const E_ERRNO: u16 = 30;
/// net.tcp reports an empty receive queue as this errno.
pub const RX_EMPTY: u16 = 11;
