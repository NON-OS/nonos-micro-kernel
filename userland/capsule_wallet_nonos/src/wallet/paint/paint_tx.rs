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

pub fn paint_tx(state: &State, fb: &mut PaintBuffer) {
    if state.proof_count >= 2 {
        return;
    }
    fb.text(368, 532, b"Signed tx", MUTED);
    if !state.tx_ready {
        fb.text(368, 560, b"No transaction", FG);
        return;
    }
    let mut len = [0u8; 10];
    let n = super::format_u32::format_u32(state.tx_len, &mut len);
    let mut hash = [0u8; 66];
    super::hex_hash::hex_hash(&state.tx_hash, &mut hash);
    fb.text(368, 560, state.tx_kind, FG);
    fb.text(432, 560, &len[..n], FG);
    fb.text(368, 596, &hash[..34], ACCENT);
    fb.text(368, 620, &hash[34..], ACCENT);
}
