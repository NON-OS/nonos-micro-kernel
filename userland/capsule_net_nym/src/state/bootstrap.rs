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

use super::gateway::{Gateway, Transport};

/// Entry gateways to try before a directory is available.
///
/// The same pattern Tor uses for fallback directories: a client with no
/// directory yet has to reach the network somehow, and asking it to be
/// configured by hand first is worse than shipping a list that can be
/// replaced. These are only a starting point. Every one is authenticated by
/// its identity key during registration, so a stale or hostile entry cannot
/// impersonate a gateway; the worst it can do is fail to answer.
///
/// Replace with the directory client once it lands.
pub const BOOTSTRAP_GATEWAYS: [(([u8; 4], u16), [u8; 32]); 5] = [
    (
        ([109, 207, 171, 114], 9000),
        [
            0x9a, 0x26, 0x2b, 0xc9, 0x03, 0x68, 0xec, 0x22, 0xdc, 0x79, 0xc7, 0x07, 0x92, 0x06,
            0x90, 0xd4, 0xa2, 0x58, 0x12, 0xc7, 0xcb, 0x9c, 0x25, 0x4b, 0xfa, 0x11, 0x39, 0xd1,
            0x55, 0xa0, 0x2e, 0x2f,
        ],
    ),
    (
        ([185, 186, 147, 240], 9000),
        [
            0x4a, 0x59, 0x8f, 0xc5, 0xa7, 0xa0, 0xaa, 0xbd, 0x46, 0xe1, 0x4d, 0xaf, 0x99, 0x85,
            0xa0, 0xc3, 0x0b, 0x1b, 0xb2, 0x8f, 0x78, 0xb2, 0x75, 0x3f, 0x82, 0xd1, 0x0c, 0xbe,
            0xcb, 0x6a, 0x8c, 0xa6,
        ],
    ),
    (
        ([162, 247, 153, 178], 9000),
        [
            0xf9, 0x89, 0x67, 0xb3, 0xe5, 0xfc, 0xa7, 0xb8, 0x5d, 0xa0, 0x88, 0x71, 0x6b, 0x14,
            0x2d, 0x47, 0x57, 0x0d, 0x6c, 0x87, 0x16, 0x2c, 0x4d, 0x0a, 0xcf, 0x37, 0xf2, 0x3b,
            0xe2, 0x2d, 0x65, 0xdd,
        ],
    ),
    (
        ([142, 91, 109, 111], 9000),
        [
            0x8c, 0xaa, 0xf9, 0xeb, 0x66, 0x73, 0xa9, 0x84, 0x92, 0x8a, 0x29, 0x66, 0xfa, 0x63,
            0x95, 0xae, 0xd0, 0x21, 0x2f, 0x84, 0xba, 0xb2, 0xeb, 0xf6, 0x9b, 0xee, 0xa8, 0xf3,
            0xeb, 0x89, 0x6e, 0x3a,
        ],
    ),
    (
        ([152, 53, 108, 138], 9000),
        [
            0x2e, 0x76, 0x79, 0xcb, 0xee, 0x8b, 0x4d, 0x38, 0x6c, 0xae, 0xcd, 0x29, 0xe2, 0x82,
            0xfe, 0xea, 0x33, 0xc7, 0xeb, 0x84, 0x7c, 0x60, 0x43, 0xd3, 0xef, 0xe4, 0xa5, 0xe0,
            0x52, 0xbf, 0xdb, 0x80,
        ],
    ),
];

/// The next bootstrap entry to try, as a gateway record ready to connect.
pub fn bootstrap_gateway(index: usize) -> Option<Gateway> {
    let ((ip, port), identity) = *BOOTSTRAP_GATEWAYS.get(index)?;
    Some(Gateway {
        ip,
        port,
        stream: 0,
        transport: Transport::WebSocket,
        identity,
        shared_key: [0u8; 32],
    })
}
