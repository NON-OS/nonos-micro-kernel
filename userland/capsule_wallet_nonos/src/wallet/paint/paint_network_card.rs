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
use crate::wallet::theme::{CYAN, FG, MUTED, WARN};

pub fn paint_network_card(state: &State, fb: &mut PaintBuffer, x: u32) {
    fb.text(x, 160, b"Network privacy", MUTED);
    fb.text_scaled(x, 198, if state.net.route_ready { b"Route ready" } else { b"Local mode" }, FG, 2);
    fb.text(x, 250, super::paint_network_labels::primary(state), CYAN);
    fb.text(x, 286, super::paint_network_labels::secondary(state), WARN);
    super::paint_button::paint_button(fb, x, 326, 170, b"Probe route");
}
