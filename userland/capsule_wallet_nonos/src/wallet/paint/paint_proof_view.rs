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
use crate::wallet::theme::{FG, MUTED};

pub fn paint_proof_view(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    super::panel::panel(fb, 336, 128, w - 16, fb.height.saturating_sub(220));
    fb.text(368, 164, b"Transaction proofs", MUTED);
    if state.proof_count >= 2 {
        super::paint_proofs::paint_proofs(state, fb);
    } else if state.tx_ready {
        super::paint_tx::paint_tx(state, fb);
    } else {
        fb.text_scaled(368, 224, b"No signed transaction", FG, 2);
        fb.text(368, 282, b"Use Send or Proof action after generating a wallet.", MUTED);
    }
}
