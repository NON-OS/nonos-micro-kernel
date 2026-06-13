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

pub fn paint_proofs(state: &State, fb: &mut PaintBuffer) {
    if state.proof_count < 2 {
        return;
    }
    let mut eth = [0u8; 66];
    let mut nox = [0u8; 66];
    super::hex_hash::hex_hash(&state.proof_eth_hash, &mut eth);
    super::hex_hash::hex_hash(&state.proof_nox_hash, &mut nox);
    fb.text(368, 224, b"Signed proofs", MUTED);
    fb.text(368, 272, b"ETH", FG);
    fb.text(436, 272, &eth[..34], ACCENT);
    fb.text(368, 326, b"NOX", FG);
    fb.text(436, 326, &nox[..34], ACCENT);
}
