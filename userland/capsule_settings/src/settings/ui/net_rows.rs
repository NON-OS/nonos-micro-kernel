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

use crate::settings::state::{State, WifiConnect};

use super::bytes::as_str;
use super::control_geom::right_edge;
use super::metrics::BODY_PX;
use super::net_signal::draw_bars;
use super::row_label;
use super::text;
use super::theme::{OK, SUBLABEL_FG};

/// One scanned network. The list is empty until a scan runs, and an empty list
/// says so rather than drawing nothing.
pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    index: usize,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    row_h: u32,
) {
    if state.wifi_network_count == 0 {
        row_label::paint(fb, card_x, screen_y, row_h, "No networks found", None);
        let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
        text::right(fb, right_edge(card_x, card_w), top, "Scan", SUBLABEL_FG, BODY_PX);
        return;
    }
    let net = state.wifi_networks[index.min(state.wifi_network_count - 1)];
    let joined = state.wifi_connect == WifiConnect::Connected && index == state.wifi_cursor;
    let note = if net.secured { Some("Secured") } else { Some("Open") };
    row_label::paint(fb, card_x, screen_y, row_h, as_str(net.ssid()), note);
    let right = right_edge(card_x, card_w);
    let cy = screen_y + (row_h / 2) as i32;
    draw_bars(fb, right, cy, net.signal);
    if joined {
        let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
        text::right(fb, right - 26, top, "Connected", OK, BODY_PX);
    }
}
