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

//! Offsets a reply is read at.

use crate::ack::PADDED_ADDRESS_BYTES;
use crate::sphinx::constants::{ACK_PAYLOAD_SIZE, HEADER_SIZE};

/// Bytes the acknowledgement takes at the front of a reply, before anything
/// meant for us begins.
pub const ACK_SPAN: usize = PADDED_ADDRESS_BYTES + HEADER_SIZE + ACK_PAYLOAD_SIZE;

/// Bytes of the tag naming which reply block a reply came back on. It is a
/// digest of that block's key, so it identifies one of ours to us and nothing
/// to anyone else.
pub const DIGEST_BYTES: usize = 32;

/// A message that says it is a reply.
pub const TYPE_REPLY: u8 = 2;

/// Reply content that carries data, rather than a request for more blocks.
pub const TAG_REPLY_DATA: u8 = 0;
