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

//! The version a hop reads out of the header to know how we built it.

use super::fields::VERSION_LENGTH;

/// Versions are a big endian u16 behind a leading zero. The spare byte is
/// left over from an older reading of the field and is not part of the
/// number, so a version written into the first byte names no version at all.
const fn version_bytes(value: u16) -> [u8; VERSION_LENGTH] {
    let b = value.to_be_bytes();
    [0, b[0], b[1]]
}

/// Explicit payload keys over standard X25519.
///
/// This says how a hop derives the key that unwraps its own payload layer:
/// from the full key carried per hop rather than from a seed it expands
/// itself. It has to agree with how the keys were built, and ours are built
/// in full, so a hop told anything else would derive a key that does not
/// match the layer it is handed.
pub const PACKET_VERSION: [u8; VERSION_LENGTH] = version_bytes(258);
