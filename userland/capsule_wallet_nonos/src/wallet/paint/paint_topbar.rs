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

use crate::wallet::hex::hex_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, WARN};

pub fn paint_topbar(state: &State, fb: &mut PaintBuffer) {
    let compact = fb.width < 1420;
    fb.text_scaled(
        336,
        28,
        if compact { b"ETH custody" } else { b"Ethereum mainnet custody" },
        FG,
        2,
    );
    fb.text(340, 66, b"self-custody keys inside NONOS keyring", MUTED);
    let x = if compact { 760 } else { fb.width.saturating_sub(520) };
    if !compact {
        fb.text(x, 30, b"PublicNode", MUTED);
        fb.text(x + 118, 30, b"ethereum-rpc.publicnode.com", CYAN);
    }
    fb.text(x, 58, b"Route", MUTED);
    fb.text(
        x + 118,
        58,
        if state.net.route_ready { b"ready" } else { b"local" },
        ACCENT,
    );
    fb.text(
        x + 202,
        58,
        if state.net.rpc_chain_ok { b"chain 0x1" } else if state.net.tls_client_finished_ok { b"client fin" } else if state.net.tls_finished_ok { b"TLS Finished" } else if state.net.tls_validity_ok { b"cert time" } else if state.net.tls_hostname_ok { b"host match" } else if state.net.tls_certificate_ok { b"cert chain" } else if state.net.tls_record_ok { b"TLS record" } else if state.net.tls_server_ok { b"TLS hello" } else if state.net.rpc_tcp_ok { b"tcp 443" } else { b"TLS pending" },
        WARN,
    );
    if !compact {
        fb.text(x + 336, 58, if state.net.dns_ok { b"dns" } else { b"-" }, MUTED);
        fb.text(x + 380, 58, if state.net.sockets_ok { b"sock" } else { b"-" }, MUTED);
        fb.text(x + 436, 58, if state.net.nym_ok { b"nym" } else { b"-" }, MUTED);
    }
    if state.address_ready && !compact {
        let mut addr = [0u8; 42];
        hex_addr(&state.address, &mut addr);
        fb.text(x, 76, &addr[..22], ACCENT);
    }
}
