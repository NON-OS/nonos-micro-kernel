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

//! Sizes an acknowledgement is defined in terms of.

pub use crate::sphinx::constants::{
    ACK_IV_SIZE as ACK_IV_BYTES, ACK_PLAINTEXT_SIZE as ACK_PLAINTEXT_BYTES,
    FRAG_ID_SIZE as FRAG_ID_BYTES,
};

/// Width the first hop address is padded to inside a payload. A routing
/// address varies in length with its protocol, and a reader that has to find
/// the packet after it cannot have that offset depend on the address.
pub const PADDED_ADDRESS_BYTES: usize = 19;
