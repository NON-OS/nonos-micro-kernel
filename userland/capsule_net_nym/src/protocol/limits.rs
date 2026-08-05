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

pub const MIX_PAYLOAD_MAX: usize = 1024;
pub const NYM_HEADER_BYTES: usize = 365;
pub const NYM_PAYLOAD_BYTES: usize = 2048;
pub const WIRE_PACKET_MAX: usize = NYM_HEADER_BYTES + NYM_PAYLOAD_BYTES;
pub const COVER_BYTES: usize = MIX_PAYLOAD_MAX;
/// Largest body one request or reply carries over IPC.
///
/// A reply is whatever the far end had to say, reassembled, so it is not
/// bounded by the size of a packet. Anything longer than this is handed over
/// across several reads rather than refused, but a stream reads far better
/// when a whole response usually fits in one.
pub const IPC_PAYLOAD_MAX: usize = 32 * 1024;
