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

use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, BG, FG, MUTED, NEUTRAL_800, OK, WARN};

pub fn paint_network_card(state: &State, fb: &mut PaintBuffer, x: u32) {
    let w = 322u32;
    let _ = fb.text_ttf(x as i32, 150, "NETWORK PRIVACY", ACCENT, 10.0);
    let title = if state.net.route_ready { "Route ready" } else { "Local mode" };
    let _ = fb.text_ttf(x as i32, 164, title, FG, 22.0);

    let (tls, tls_tone): (&[u8], u32) = if state.net.tls_client_finished_ok {
        (b"TLS 1.3", OK)
    } else {
        (b"pending", WARN)
    };
    row(fb, x, w, 206, "TLS / RPC codec", tls, tls_tone);
    let (route, route_tone): (&[u8], u32) = if state.net.rpc_chain_ok {
        (b"on-chain", OK)
    } else {
        (b"local", ACCENT)
    };
    row(fb, x, w, 244, "TCP route", route, route_tone);

    ui::primary(fb, x, 300, w, b"Probe route");
}

fn row(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, label: &str, tag: &[u8], tone: u32) {
    fb.fill_rect(x, y, w, 30, BG);
    fb.fill_rect(x, y, w, 1, NEUTRAL_800);
    fb.fill_rect(x, y + 29, w, 1, NEUTRAL_800);
    fb.fill_rect(x, y, 1, 30, NEUTRAL_800);
    fb.fill_rect(x + w - 1, y, 1, 30, NEUTRAL_800);
    let _ = fb.text_ttf((x + 12) as i32, (y + 8) as i32, label, MUTED, 12.0);
    let tw = fb.measure_ttf(core::str::from_utf8(tag).unwrap_or(""), 11.0).max(0) as u32 + 20;
    ui::badge(fb, x + w - 12 - tw, y + 4, tag, tone);
}
