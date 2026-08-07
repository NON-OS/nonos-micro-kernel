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

use super::nox_layout::NoxLayout;
use super::nox_lock::lock_card;
use super::scale;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{DIM, GREEN};

/// Claimable rewards, then the lock terms. Both sit in the right column when
/// there is room for one and under the staking card when there is not, which
/// is decided once in the layout rather than guessed at here.
pub fn right_column(state: &State, fb: &mut PaintBuffer, l: &NoxLayout) {
    let ry = l.right_y();
    ui::card(fb, l.rx, ry, l.rw, NoxLayout::REWARD_H);
    let _ =
        fb.text_ttf((l.rx + 20) as i32, (ry + 18) as i32, "CLAIMABLE REWARDS", DIM(), scale::LABEL);
    let mut cl = [0u8; 48];
    let claim = crate::wallet::nox::amount_str(
        state.nox.claimable_ready,
        &state.nox.claimable_wei,
        &mut cl,
    );
    // The figure shrinks rather than overflowing its card on a narrow window.
    let px = if l.rw < 300 { 19.0 } else { 25.3 };
    let _ = fb.text_ttf((l.rx + 20) as i32, (ry + 40) as i32, claim, GREEN(), px);
    lock_card(state, fb, l);
}
