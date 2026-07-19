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
use crate::wallet::theme::{FG, MUTED};

pub fn paint_proof_view(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    ui::title(fb, 368, 128, b"PROOF", "Transaction proofs");
    if state.proof_count >= 2 {
        super::paint_proofs::paint_proofs(state, fb);
    } else if state.tx_ready {
        super::paint_tx::paint_tx(state, fb);
    } else {
        ui::card(fb, 336, 200, w - 16, 120);
        let _ = fb.text_ttf(368, 232, "No signed transaction", FG, 18.0);
        let _ = fb.text_ttf(368, 264, "Use Send or Proof action after generating a wallet.", MUTED, 13.0);
    }
}
