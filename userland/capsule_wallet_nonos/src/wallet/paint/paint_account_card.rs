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

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer) {
    fb.text(368, 160, b"Account", MUTED);
    fb.text_scaled(
        368,
        198,
        if state.wallet_id == 0 { b"Not created" } else { b"Active wallet" },
        FG,
        2,
    );
    fb.text(
        368,
        250,
        if state.address_ready { b"Receive address ready" } else { b"Generate wallet to start" },
        ACCENT,
    );
    super::paint_button::paint_button(fb, 368, 300, 180, b"Generate");
}
