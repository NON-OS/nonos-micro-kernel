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
use crate::wallet::theme::{DIM, FG, MUTED};

pub fn paint_nox(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let sw = (cw - 48) / 4;

    // Every figure is read live from the pinned mainnet contracts. A field that
    // has not returned yet shows "fetching" while the link is up, a dash only
    // when there is no route, and never a fabricated number.
    let up = state.net.rpc_chain_ok;
    let pending = if up { "\u{2026}" } else { "\u{2014}" };
    let mut apr_b = [0u8; 16];
    let apr = if state.nox.apr_ready {
        let n = crate::wallet::nox::format_apr(state.nox.apr_bps, &mut apr_b);
        core::str::from_utf8(&apr_b[..n]).unwrap_or(pending)
    } else {
        pending
    };
    let mut rev_b = [0u8; 48];
    let rev = crate::wallet::nox::live_amount(
        state.nox.stats_ready,
        &state.nox.rewards_distributed_wei,
        up,
        &mut rev_b,
    );
    let mut pos_b = [0u8; 20];
    let pos = if state.nox.positions_ready {
        let n = super::format_u64::format_u64(state.nox.positions, &mut pos_b);
        core::str::from_utf8(&pos_b[..n]).unwrap_or(pending)
    } else {
        pending
    };

    stat(fb, cx, 146, sw, "STAKING APR", apr);
    stat(fb, cx + sw + 16, 146, sw, "REWARDS PAID", rev);
    stat(fb, cx + 2 * (sw + 16), 146, sw, "YOUR POSITIONS", pos);
    let mut bal_b = [0u8; 48];
    let bal = crate::wallet::nox::live_amount(
        state.nox.balance_ready,
        &state.nox.balance_wei,
        up,
        &mut bal_b,
    );
    stat(fb, cx + 3 * (sw + 16), 146, sw, "NOX BALANCE", bal);

    ui::card(fb, cx, 252, cw, 150);
    let _ = fb.text_ttf((cx + 20) as i32, 270, "NOX ON-CHAIN STATE", DIM(), 12.1);
    let mut ts_b = [0u8; 48];
    let ts = crate::wallet::nox::live_amount(
        state.nox.stats_ready,
        &state.nox.total_staked_wei,
        up,
        &mut ts_b,
    );
    let _ = fb.text_ttf((cx + 20) as i32, 300, "Total staked", MUTED(), 14.9);
    let tw = fb.measure_ttf(ts, 17.2).max(0) as u32;
    let _ = fb.text_ttf((cx + cw - 20 - tw) as i32, 298, ts, FG(), 17.2);
    let mut sa = [0u8; 13];
    crate::wallet::hex::short_addr(&crate::wallet::nox::constants::STAKING_PROXY, &mut sa);
    let _ = fb.text_ttf((cx + 20) as i32, 328, "Staking contract", MUTED(), 14.9);
    let _ = fb.text_ttf_mono(
        (cx + cw - 20 - 96) as i32,
        328,
        core::str::from_utf8(&sa).unwrap_or(""),
        DIM(),
        14.9,
    );
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        360,
        "Read live from Ethereum mainnet (chain 1).",
        MUTED(),
        14.9,
    );

    super::paint_nox_stake::paint_nox_stake(state, fb);
}

fn stat(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, val: &str) {
    ui::card(fb, x, y, w, 90);
    let _ = fb.text_ttf((x + 18) as i32, (y + 18) as i32, label, DIM(), 12.1);
    let _ = fb.text_ttf((x + 18) as i32, (y + 40) as i32, val, FG(), 32.2);
}
