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
use nonos_app_skeleton::PaintBuffer;

use super::rail_addr::{ipv4_pfx, ipv6_str};
use super::rail_fmt::bps_into;
use super::rail_metric::pair;
use super::rail_stat::stat;
use super::rail_text::head;
use crate::rail::net::Net;
use crate::term::theme::types::Theme;

pub use super::rail_geom::net_h as height;

/// The interface as the DHCP client last saw it. The name comes from the sample
/// rather than a literal, so a second interface would caption itself correctly.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, n: &Net, t: &Theme) {
    let mut b = [0u8; 64];
    let mut y = head(fb, x, y, w, "NETWORK", t);
    let state = if n.up { "UP" } else { "DOWN" };
    y = stat(fb, x, y, w, n.name_str(), state, t);
    y = stat(fb, x, y, w, "IPV4", ipv4_pfx(&mut b, n.ipv4, n.prefix_len), t);
    y = stat(fb, x, y, w, "IPV6", ipv6_str(&mut b, n.ipv6), t);
    stat(fb, x, y, w, "RATES", pair(&mut b, n.rx_bps, n.tx_bps, bps_into), t);
}
