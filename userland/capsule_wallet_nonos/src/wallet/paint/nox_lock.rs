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
use super::ui;
use crate::wallet::nox::{lock_days, nft_boost_bps, LOCK_TERMS};
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, DIM, FG, INK, LINE2, MUTED};

/// Lock term and what it is worth, beside the passes this wallet holds.
///
/// The weight is the contract's own: a term multiplies a stake, and so do
/// the passes, both in basis points where ten thousand is no boost. Showing
/// the multiplier rather than a name means the reader can see what the
/// longer lock is actually buying before committing to it.
pub fn lock_card(state: &State, fb: &mut PaintBuffer, l: &NoxLayout) {
    let y = l.lock_y();
    ui::card(fb, l.rx, y, l.rw, NoxLayout::LOCK_H);
    let _ = fb.text_ttf((l.rx + 20) as i32, (y + 14) as i32, "LOCK TERM", DIM(), scale::LABEL);
    let chips = l.rw.saturating_sub(40) / 6;
    for (i, (secs, boost)) in LOCK_TERMS.iter().enumerate() {
        let x = l.rx + 20 + i as u32 * chips;
        let sel = state.stake_lock == i as u8;
        if sel {
            fb.fill_rect(x, y + 32, chips.saturating_sub(4), 26, ACCENT());
        } else {
            ui::edge(fb, x, y + 32, chips.saturating_sub(4), 26, LINE2());
        }
        let mut b = [0u8; 8];
        let n = term_label(*secs, &mut b);
        if let Ok(s) = core::str::from_utf8(&b[..n]) {
            let c = if sel { INK() } else { MUTED() };
            let tw = fb.measure_ttf(s, scale::LABEL).max(0) as u32;
            let tx = x + chips.saturating_sub(4 + tw) / 2;
            let _ = fb.text_ttf(tx as i32, (y + 38) as i32, s, c, scale::LABEL);
        }
        let _ = boost;
    }
    weight_line(state, fb, l, y);
}

// What the current choices multiply a stake by, lock and passes together.
fn weight_line(state: &State, fb: &mut PaintBuffer, l: &NoxLayout, y: u32) {
    let lock = LOCK_TERMS[(state.stake_lock as usize).min(LOCK_TERMS.len() - 1)].1;
    let nft = nft_boost_bps(state.nox.passes);
    let total = (lock as u64 * nft as u64) / 10_000;
    let mut b = [0u8; 48];
    let n = weight_text(total, state.nox.passes, &mut b);
    if let Ok(s) = core::str::from_utf8(&b[..n]) {
        let _ = fb.text_ttf((l.rx + 20) as i32, (y + 68) as i32, s, CYAN(), scale::SMALL);
    }
    let _ = FG();
}

// "1.20x lock, 2 passes" style, in basis points rendered as a multiplier.
fn weight_text(total_bps: u64, passes: u64, out: &mut [u8]) -> usize {
    let mut i = 0;
    let whole = total_bps / 10_000;
    let frac = (total_bps % 10_000) / 100;
    i += super::put_u32::put_u32(&mut out[i..], whole as u32);
    out[i] = b'.';
    i += 1;
    out[i] = b'0' + (frac / 10) as u8;
    out[i + 1] = b'0' + (frac % 10) as u8;
    i += 2;
    let tail = b"x weight, ";
    out[i..i + tail.len()].copy_from_slice(tail);
    i += tail.len();
    i += super::put_u32::put_u32(&mut out[i..], passes as u32);
    let end = b" passes";
    out[i..i + end.len()].copy_from_slice(end);
    i + end.len()
}

fn term_label(secs: u32, out: &mut [u8]) -> usize {
    if secs == 0 {
        out[..4].copy_from_slice(b"none");
        return 4;
    }
    let n = super::put_u32::put_u32(out, lock_days(secs));
    out[n] = b'd';
    n + 1
}
