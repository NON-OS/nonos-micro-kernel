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

//! The RSN information element the station advertises. Kept in its own file so the
//! association request and the four-way handshake draw the same bytes from one
//! place, and so both the driver crate and the proof crates include it identically.

/// The RSN information element for WPA2-PSK with CCMP: element id 48, then version
/// 1, the CCMP group cipher, one CCMP pairwise cipher, and the PSK authentication
/// suite, and empty RSN capabilities. The association request carries this to the
/// access point, and message two of the handshake repeats it; the two must be
/// byte-identical, which is why they share this constant.
pub const RSN_IE: [u8; 22] = [
    0x30, 0x14, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00,
    0x00, 0x0f, 0xac, 0x02, 0x00, 0x00,
];
