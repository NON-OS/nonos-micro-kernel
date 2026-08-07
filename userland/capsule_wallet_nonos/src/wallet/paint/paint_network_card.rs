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
use crate::wallet::theme::{DIM, FG, GREEN, LINE, MUTED};

pub fn paint_network_card(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    ui::card(fb, x, y, w, 200);
    let _ =
        fb.text_ttf((x + 20) as i32, (y + 18) as i32, "GAS  \u{00b7}  ETHEREUM L1", DIM(), 12.1);

    // The real gas price read from the RPC, to two decimals so a sub-gwei price
    // does not collapse to zero. A dash before the read lands.
    let mut gb = [0u8; 24];
    let g = if state.fee_ready {
        let n = format_gwei(state.fee_wei, &mut gb);
        core::str::from_utf8(&gb[..n]).unwrap_or("\u{2014}")
    } else if state.net.rpc_chain_ok {
        "\u{2026}"
    } else {
        "\u{2014}"
    };
    let gx = fb.text_ttf((x + 20) as i32, (y + 38) as i32, g, FG(), 34.5);
    let _ = fb.text_ttf(gx + 8, (y + 50) as i32, "gwei", MUTED(), 14.9);

    // Honest facts about the route, not invented fee tiers.
    row(fb, x, w, y + 84, "Route", "PublicNode RPC", "TLS 1.3");
    row(fb, x, w, y + 122, "Chain", "Ethereum mainnet", "0x1");
    row(fb, x, w, y + 160, "Transfer gas", "21000", "fixed");
}

// A label on the left, its value on the right, and a tag after it.
//
// Everything is measured and nothing is assumed. The value used to be placed
// at a fixed inset that reserved room for the tag, so on a narrow card the
// label and the value ran into each other and read as one word: Chain and
// Ethereum mainnet became ChainEthereum mainnet. When the room is not there
// the tag is dropped first, since it is the least of the three, and the value
// is dropped before it is allowed to overlap.
fn row(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, label: &str, val: &str, t: &str) {
    fb.fill_rect(x + 20, y + 30, w.saturating_sub(40), 1, LINE());
    let _ = fb.text_ttf((x + 20) as i32, (y + 6) as i32, label, MUTED(), 16.1);
    let lw = fb.measure_ttf(label, 16.1).max(0) as u32;
    let floor = x + 20 + lw + 12;
    let vw = fb.measure_ttf(val, 16.1).max(0) as u32;
    let tw = fb.measure_ttf(t, 14.9).max(0) as u32;
    let right = x + w.saturating_sub(20);
    // With the tag: value, gap, tag, all inside the card.
    if right.saturating_sub(tw + 12 + vw) >= floor {
        let _ = fb.text_ttf(right.saturating_sub(tw) as i32, (y + 7) as i32, t, GREEN(), 14.9);
        let _ =
            fb.text_ttf(right.saturating_sub(tw + 12 + vw) as i32, (y + 6) as i32, val, FG(), 16.1);
        return;
    }
    // Without it, if the value alone still fits.
    if right.saturating_sub(vw) >= floor {
        let _ = fb.text_ttf(right.saturating_sub(vw) as i32, (y + 6) as i32, val, FG(), 16.1);
    }
}

// wei-per-gas to "N.NN gwei" with two decimals.
fn format_gwei(wei: u64, out: &mut [u8]) -> usize {
    let whole = wei / 1_000_000_000;
    let cents = (wei % 1_000_000_000) / 10_000_000;
    let mut wb = [0u8; 20];
    let wn = super::format_u64::format_u64(whole, &mut wb);
    out[..wn].copy_from_slice(&wb[..wn]);
    let mut n = wn;
    out[n] = b'.';
    out[n + 1] = b'0' + ((cents / 10) % 10) as u8;
    out[n + 2] = b'0' + (cents % 10) as u8;
    n += 3;
    n
}
