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
    (([188, 172, 228, 15], 9000), [
        219, 112, 40, 128, 37, 198, 11, 235, 220, 12, 95, 179, 91, 171, 93, 58, 171, 138, 81, 0,
        87, 128, 223, 189, 68, 255, 51, 42, 131, 120, 34, 26,
    ]),
    (([172, 232, 103, 199], 9000), [
        230, 122, 158, 77, 196, 15, 219, 163, 28, 237, 222, 184, 50, 198, 155, 253, 163, 202, 39,
        51, 155, 239, 74, 98, 79, 193, 59, 183, 122, 96, 114, 117,
    ]),
    (([159, 89, 174, 132], 9000), [
        178, 14, 245, 27, 83, 237, 35, 212, 201, 25, 245, 151, 131, 42, 9, 143, 12, 62, 77, 172,
        249, 251, 89, 66, 227, 138, 247, 50, 108, 77, 131, 111,
    ]),
    (([213, 226, 71, 95], 9000), [
        207, 224, 239, 236, 106, 184, 47, 206, 230, 220, 24, 184, 209, 226, 112, 253, 119, 95, 53,
        153, 153, 29, 39, 125, 34, 226, 91, 46, 169, 65, 186, 93,
    ]),
    (([152, 53, 92, 255], 9000), [
        120, 185, 66, 17, 195, 209, 178, 148, 19, 237, 44, 128, 207, 69, 167, 228, 245, 177, 49,
        137, 155, 53, 227, 136, 44, 86, 88, 242, 244, 235, 243, 168,
    ]),
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
