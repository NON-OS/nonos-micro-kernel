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
use crate::wallet::theme::{ACCENT, FG, MUTED, OK, WARN};

pub fn paint_topbar(state: &State, fb: &mut PaintBuffer) {
    let _ = fb.text_ttf(336, 18, "ETH custody", FG, 32.0);
    let _ = fb.text_ttf(336, 58, "self-custody keys inside NONOS keyring", MUTED, 13.0);

    let route: &[u8] = if state.net.route_ready { b"Route: ready" } else { b"Route: local" };
    let tls: &[u8] = if state.net.rpc_chain_ok {
        b"TLS 1.3 - chain 0x1"
    } else if state.net.tls_client_finished_ok {
        b"TLS 1.3 - client fin"
    } else if state.net.tls_finished_ok {
        b"TLS 1.3 - finished"
    } else if state.net.tls_certificate_ok {
        b"TLS 1.3 - cert chain"
    } else if state.net.tls_server_ok {
        b"TLS 1.3 - hello"
    } else {
        b"TLS pending"
    };
    let tls_tone = if state.net.rpc_chain_ok { OK } else { WARN };

    let wt = fb.measure_ttf(core::str::from_utf8(tls).unwrap_or(""), 11.0).max(0) as u32 + 20;
    let wr = fb.measure_ttf(core::str::from_utf8(route).unwrap_or(""), 11.0).max(0) as u32 + 20;
    let xt = fb.width.saturating_sub(32 + wt);
    let xr = xt.saturating_sub(8 + wr);
    ui::badge(fb, xr, 44, route, ACCENT);
    ui::badge(fb, xt, 44, tls, tls_tone);
}
