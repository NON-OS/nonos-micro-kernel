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

use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, FG, MUTED};

pub fn paint_net(state: &State, fb: &mut PaintBuffer) {
    fb.text(340, 220, b"Network", MUTED);
    fb.text(340, 240, b"dns", if state.net.dns_ok { ACCENT } else { FG });
    fb.text(384, 240, b"sockets", if state.net.sockets_ok { ACCENT } else { FG });
    fb.text(464, 240, b"nym", if state.net.nym_ok { ACCENT } else { FG });
    fb.text(340, 260, b"https", ACCENT);
    fb.text(392, 260, b"ethereum-rpc.publicnode.com", FG);
    fb.text(340, 280, b"chain", MUTED);
    fb.text(392, 280, b"0x1", FG);
    fb.text(444, 280, if state.net.route_ready { b"route ok" } else { b"blocked" }, FG);
    fb.text(552, 280, if state.net.rpc_tcp_ok { b"tcp 443" } else { b"tls" }, FG);
}
