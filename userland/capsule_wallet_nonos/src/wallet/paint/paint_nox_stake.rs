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

use super::nox_amount::amount_row;
use super::nox_layout::NoxLayout;
use super::nox_right::right_column;
use super::scale;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, FG, INK, LINE2, MUTED};

pub fn paint_nox_stake(state: &State, fb: &mut PaintBuffer) {
    let l = NoxLayout::sized(fb.width, fb.height);
    ui::card(fb, l.cx, l.top(), l.lw, NoxLayout::CARD_H);
    let stake = state.stake_unstake == 0;
    tab(fb, l.cx + 20, l.tabs_y(), l.tab_w, "Stake NOX", stake);
    tab(fb, l.cx + 20 + l.tab_w, l.tabs_y(), l.tab_w, "Unstake", !stake);

    amount_row(state, fb, &l, stake);

    // Staking is approve then stake; the label tracks which step is next.
    // Closing a position needs no approve, so it says what it will do.
    let mut bb = [0u8; 80];
    let bn = action_label(state, stake, &mut bb);
    ui::primary(fb, l.cx + 20, l.action_y(), l.track_w, &bb[..bn]);

    right_column(state, fb, &l);

    ui::card(fb, l.cx, l.fees_y(), l.cw, 92);
    let _ = fb.text_ttf(
        (l.cx + 20) as i32,
        (l.fees_y() + 14) as i32,
        "Where the fee goes",
        FG(),
        scale::VALUE,
    );
    fee(fb, l.cx, l.fees_y() + 40, "protocol fee", "treasury / NOX stakers / buyback-burn");
    fee(fb, l.cx, l.fees_y() + 62, "the relayer", "fronts the gas for a stake");
}

// What the button will actually sign, said plainly.
fn action_label(state: &State, stake: bool, out: &mut [u8]) -> usize {
    if !stake {
        let pre = b"Close position ";
        out[..pre.len()].copy_from_slice(pre);
        let mut nb = [0u8; 20];
        let n = super::put_u32::put_u32(&mut nb, state.stake_position as u32);
        out[pre.len()..pre.len() + n].copy_from_slice(&nb[..n]);
        return pre.len() + n;
    }
    let pre: &[u8] = if state.stake_step == 0 { b"Approve " } else { b"Stake " };
    let mut i = pre.len();
    out[..i].copy_from_slice(pre);
    let mut nb = [0u8; 48];
    let nn = crate::wallet::nox::format_nox(state.stake_amount, &mut nb);
    out[i..i + nn].copy_from_slice(&nb[..nn]);
    i += nn;
    out[i..i + 4].copy_from_slice(b" NOX");
    i + 4
}

fn tab(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, sel: bool) {
    if sel {
        fb.fill_rect(x, y, w, 36, ACCENT());
    } else {
        ui::edge(fb, x, y, w, 36, LINE2());
    }
    let c = if sel { INK() } else { MUTED() };
    let tw = fb.measure_ttf(label, scale::BODY).max(0) as u32;
    let tx = x + w.saturating_sub(tw) / 2;
    let _ = fb.text_ttf(tx as i32, (y + 9) as i32, label, c, scale::BODY);
}

fn fee(fb: &mut PaintBuffer, cx: u32, y: u32, what: &str, who: &str) {
    let _ = fb.text_ttf((cx + 20) as i32, y as i32, what, MUTED(), scale::SMALL);
    let _ = fb.text_ttf((cx + 190) as i32, y as i32, who, FG(), scale::SMALL);
}
