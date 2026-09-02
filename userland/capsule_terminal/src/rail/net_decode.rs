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

use super::net::Net;
use super::value::Metric;

pub const HDR_LEN: usize = 20;
pub const BODY_LEN: usize = 22;
pub const BODY_MIN: usize = 18;
pub const REPLY_LEN: usize = HDR_LEN + BODY_LEN;
pub const REPLY_MIN: usize = HDR_LEN + BODY_MIN;

const BOUND_STATE: u8 = 3;

/// A reply short of a whole lease body, or one carrying a pre-bound state, is
/// an interface with no address rather than an address of zero.
pub fn decode_lease(rx: &[u8]) -> Net {
    if rx.len() < REPLY_MIN || rx[HDR_LEN] < BOUND_STATE {
        return Net::DOWN;
    }
    Net {
        up: true,
        ipv4: Metric::Known(quad(&rx[HDR_LEN + 1..])),
        prefix_len: Metric::Known(rx[HDR_LEN + 5]),
        gateway: Metric::Known(quad(&rx[HDR_LEN + 6..])),
        ..Net::DOWN
    }
}

fn quad(src: &[u8]) -> [u8; 4] {
    let mut out = [0u8; 4];
    out.copy_from_slice(&src[..4]);
    out
}
