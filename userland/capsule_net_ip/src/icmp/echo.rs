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

use super::build::{build as icmp_build, BuildError};
use super::types::{IcmpHeader, TYPE_ECHO_REPLY, TYPE_ECHO_REQUEST};

#[derive(Clone, Copy)]
pub struct Echo<'a> {
    pub identifier: u16,
    pub sequence: u16,
    pub payload: &'a [u8],
}

// Read identifier + sequence out of an ICMP echo header. The
// header's `rest` field carries them in network byte order.
pub fn echo_of<'a>(header: &IcmpHeader, payload: &'a [u8]) -> Echo<'a> {
    let identifier = u16::from_be_bytes([header.rest[0], header.rest[1]]);
    let sequence = u16::from_be_bytes([header.rest[2], header.rest[3]]);
    Echo { identifier, sequence, payload }
}

pub fn is_echo_request(h: &IcmpHeader) -> bool {
    h.icmp_type == TYPE_ECHO_REQUEST && h.code == 0
}

// Build an echo reply for an inbound echo request: type flipped
// to 0, identifier and sequence echoed, payload copied verbatim.
// Caller-provided `out` must be at least 8 + payload.len() bytes.
pub fn build_reply(req: &Echo<'_>, out: &mut [u8]) -> Result<usize, BuildError> {
    let id = req.identifier.to_be_bytes();
    let seq = req.sequence.to_be_bytes();
    let rest = [id[0], id[1], seq[0], seq[1]];
    icmp_build(TYPE_ECHO_REPLY, 0, rest, req.payload, out)
}
