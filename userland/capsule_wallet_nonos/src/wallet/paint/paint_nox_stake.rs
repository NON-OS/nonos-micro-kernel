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
use crate::wallet::theme::{ACCENT, CYAN, DIM, FG, GREEN, INK, LINE2, MUTED, PANEL_2};

pub fn paint_nox_stake(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let lw = 600u32;
    ui::card(fb, cx, 418, lw, 236);
    let stake = state.stake_unstake == 0;
    tab(fb, cx + 20, 438, "Stake NOX", stake);
    tab(fb, cx + 220, 438, "Unstake", !stake);

    let amt = state.stake_amount.min(crate::wallet::state::MAX_STAKE);
    let track = lw - 40;
    let fill = track * amt / crate::wallet::state::MAX_STAKE;
    let _ = fb.text_ttf((cx + 20) as i32, 494, "Amount", MUTED(), 14.9);
    let mut ab = [0u8; 24];
    let an = label(&mut ab, b"", amt, b" NOX");
    let av = core::str::from_utf8(&ab[..an]).unwrap_or("0");
    let aw = fb.measure_ttf(av, 16.1).max(0) as u32;
    let _ = fb.text_ttf((cx + lw - 20 - aw) as i32, 493, av, CYAN(), 16.1);
    fb.fill_rect(cx + 20, 528, track, 5, PANEL_2());
    fb.fill_rect(cx + 20, 528, fill, 5, ACCENT());
    fb.fill_rect(cx + 13 + fill, 523, 14, 14, CYAN());
    let _ = fb.text_ttf((cx + 20) as i32, 546, "0", DIM(), 13.8);
    let mut av_b = [0u8; 48];
    let avail =
        crate::wallet::nox::amount_str(state.nox.balance_ready, &state.nox.balance_wei, &mut av_b);
    let mut avail_line = [0u8; 60];
    let al = build_avail(avail, &mut avail_line);
    let aline = core::str::from_utf8(&avail_line[..al]).unwrap_or("");
    let alw = fb.measure_ttf(aline, 13.8).max(0) as u32;
    let _ = fb.text_ttf((cx + lw - 20 - alw) as i32, 546, aline, DIM(), 13.8);

    // Staking is approve then stake; the label tracks which step is next.
    let mut bb = [0u8; 32];
    let pre: &[u8] = if !stake {
        b"Unstake "
    } else if state.stake_step == 0 {
        b"Approve "
    } else {
        b"Stake "
    };
    let bn = label(&mut bb, pre, amt, b" NOX");
    ui::primary(fb, cx + 20, 596, lw - 40, &bb[..bn]);

    let rx = cx + lw + 16;
    let rw = cw - lw - 16;
    ui::card(fb, rx, 418, rw, 116);
    let _ = fb.text_ttf((rx + 20) as i32, 436, "CLAIMABLE REWARDS", DIM(), 12.1);
    let mut cl_b = [0u8; 48];
    let claim = crate::wallet::nox::amount_str(
        state.nox.claimable_ready,
        &state.nox.claimable_wei,
        &mut cl_b,
    );
    let _ = fb.text_ttf((rx + 20) as i32, 458, claim, GREEN(), 25.3);
    ui::card(fb, rx, 546, rw, 96);
    let _ = fb.text_ttf((rx + 20) as i32, 564, "LOCK PERIOD", DIM(), 12.1);
    let _ = fb.text_ttf((rx + 20) as i32, 586, "\u{2014}", FG(), 27.6);

    ui::card(fb, cx, 668, cw, 92);
    let _ = fb.text_ttf((cx + 20) as i32, 682, "Where the fee goes", FG(), 16.1);
    fee(fb, cx, 708, "protocol fee", "treasury / NOX stakers / buyback-burn");
    fee(fb, cx, 730, "the relayer", "fronts the gas for a stake");
}

fn build_avail(amount: &str, out: &mut [u8]) -> usize {
    let pre = b"avail ";
    let mut i = pre.len().min(out.len());
    out[..i].copy_from_slice(&pre[..i]);
    let a = amount.as_bytes();
    let take = a.len().min(out.len() - i);
    out[i..i + take].copy_from_slice(&a[..take]);
    i += take;
    i
}

fn label(buf: &mut [u8], pre: &[u8], n: u32, suf: &[u8]) -> usize {
    let mut i = pre.len();
    buf[..i].copy_from_slice(pre);
    let mut nb = [0u8; 10];
    let nn = super::format_u32::format_u32(n, &mut nb);
    buf[i..i + nn].copy_from_slice(&nb[..nn]);
    i += nn;
    buf[i..i + suf.len()].copy_from_slice(suf);
    i + suf.len()
}

fn tab(fb: &mut PaintBuffer, x: u32, y: u32, label: &str, sel: bool) {
    if sel {
        fb.fill_rect(x, y, 200, 36, ACCENT());
    } else {
        ui::edge(fb, x, y, 200, 36, LINE2());
    }
    let c = if sel { INK() } else { MUTED() };
    let tw = fb.measure_ttf(label, 14.9).max(0) as u32;
    let _ = fb.text_ttf((x + 100 - tw / 2) as i32, (y + 11) as i32, label, c, 14.9);
}

fn fee(fb: &mut PaintBuffer, x: u32, y: u32, k: &str, v: &str) {
    let _ = fb.text_ttf((x + 20) as i32, y as i32, k, CYAN(), 14.9);
    let _ = fb.text_ttf((x + 150) as i32, y as i32, "\u{203a}", DIM(), 14.9);
    let _ = fb.text_ttf((x + 180) as i32, y as i32, v, MUTED(), 14.9);
}
