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
use super::scale;
use crate::wallet::nox::{format_nox, held_wei};
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, DIM, MUTED, PANEL_2};

/// The amount being staked, its bar, and what is available.
///
/// On the Unstake tab there is no amount to set: the contract closes a whole
/// position by index, so the row says which position rather than offering a
/// figure that has nowhere to go.
pub fn amount_row(state: &State, fb: &mut PaintBuffer, l: &NoxLayout, stake: bool) {
    let label_y = (l.track_y() - 34) as i32;
    if !stake {
        let _ = fb.text_ttf((l.cx + 20) as i32, label_y, "Position", MUTED(), scale::BODY);
        let mut nb = [0u8; 20];
        let n = super::put_u32::put_u32(&mut nb, state.stake_position as u32);
        let v = core::str::from_utf8(&nb[..n]).unwrap_or("0");
        right_text(fb, l, label_y - 1, v, CYAN(), scale::VALUE);
        let _ = fb.text_ttf(
            (l.cx + 20) as i32,
            (l.track_y() + 18) as i32,
            "press 0-9 to choose, Enter to close",
            DIM(),
            13.8,
        );
        return;
    }
    let held = held_wei(state.nox.balance_ready, &state.nox.balance_wei).unwrap_or(0);
    let amt = state.stake_amount;
    let fill = if held == 0 { 0 } else { (l.track_w as u128 * amt.min(held) / held) as u32 };
    let _ = fb.text_ttf((l.cx + 20) as i32, label_y, "Amount", MUTED(), scale::BODY);
    let mut ab = [0u8; 64];
    let an = format_nox(amt, &mut ab);
    let av = core::str::from_utf8(&ab[..an]).unwrap_or("0");
    right_text(fb, l, label_y - 1, av, CYAN(), scale::VALUE);
    fb.fill_rect(l.track_x, l.track_y(), l.track_w, 5, PANEL_2());
    fb.fill_rect(l.track_x, l.track_y(), fill, 5, ACCENT());
    fb.fill_rect(l.track_x + fill.saturating_sub(7), l.track_y() - 5, 14, 14, CYAN());
    let _ = fb.text_ttf((l.cx + 20) as i32, (l.track_y() + 18) as i32, "0", DIM(), scale::SMALL);
    let mut avb = [0u8; 64];
    let n = format_nox(held, &mut avb);
    let av2 = core::str::from_utf8(&avb[..n]).unwrap_or("0");
    right_text(fb, l, (l.track_y() + 18) as i32, av2, DIM(), scale::SMALL);
}

// Right-aligned inside the card, so a long figure grows leftward instead of
// running off the edge when the window narrows.
fn right_text(fb: &mut PaintBuffer, l: &NoxLayout, y: i32, s: &str, c: u32, px: f32) {
    let w = fb.measure_ttf(s, px).max(0) as u32;
    let x = l.cx + l.lw.saturating_sub(20 + w);
    let _ = fb.text_ttf(x as i32, y, s, c, px);
}
