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

use crate::wallet::state::{
    State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED, VIEW_SWAP,
};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    crate::wallet::theme::set_light(state.light_mode);
    super::paint_background::paint_background(fb);
    super::paint_sysbar::paint_sysbar(fb);
    super::paint_sidebar::paint_sidebar(state, fb);
    super::paint_topbar::paint_topbar(state, fb);
    super::paint_statusline::paint_statusline(fb);
    match state.view {
        VIEW_RECEIVE => super::paint_receive::paint_receive(state, fb),
        VIEW_SEND => super::paint_send::paint_send(state, fb),
        VIEW_PROOF => super::paint_proof_view::paint_proof_view(state, fb),
        VIEW_NOX => super::paint_nox::paint_nox(state, fb),
        VIEW_SWAP => super::swap::paint_swap(state, fb),
        VIEW_SHIELDED => super::paint_portfolio::paint_portfolio(state, fb),
        VIEW_HOME => super::paint_home::paint_home(state, fb),
        _ => super::paint_home::paint_home(state, fb),
    }
    super::paint_statusbar::paint_statusbar(state, fb);
    super::paint_panels::paint_panels(state, fb);
}
