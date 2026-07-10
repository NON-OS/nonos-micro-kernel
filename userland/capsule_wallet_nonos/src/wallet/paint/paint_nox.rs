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

use crate::wallet::pool::{revenue, RevenueProvider, Seam};
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, WARN};

// NOX token-utility surface. Values come from RevenueProvider; the Stub reports
// NotWired so tiles show the honest "not wired" state, never a fake number.
pub fn paint_nox(_state: &State, fb: &mut PaintBuffer) {
    let cx = 368;
    let w = fb.width.saturating_sub(cx + 32);
    super::ui::title(fb, cx, 140, b"NOX PROTOCOL", "Revenue and staking");

    let r = revenue();
    let col = (w.saturating_sub(24)) / 2;
    tile_u32(fb, cx, 216, col, b"Fee (bps)", &r.fee_bps());
    tile_u32(fb, cx + col + 24, 216, col, b"Staking APR (bps)", &r.staking_apr_bps());
    tile_wei(fb, cx, 312, col, b"Cumulative revenue", &r.cumulative_revenue_wei());
    tile_wei(fb, cx + col + 24, 312, col, b"Your stake", &r.staked_wei());

    super::ui::card(fb, cx, 416, w, 128);
    super::ui::section(fb, cx + 20, 434, "Where the fee goes");
    fb.text(cx + 22, 476, b"protocol fee   ->  treasury / NOX stakers / buyback-burn", CYAN);
    fb.text(cx + 22, 502, b"relayer fee    ->  the relayer that fronts gas", MUTED);
    fb.text(cx + 22, 528, b"gas            ->  Ethereum L1", MUTED);

    super::ui::primary(fb, cx, 576, 200, b"Stake NOX");
    super::ui::ghost(fb, cx + 220, 576, 200, b"Unstake");
    fb.text(cx, 648, b"Live figures load from the NOX contracts once wired.", MUTED);
}

fn tile_u32(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &[u8], s: &Seam<u32>) {
    let mut b = [0u8; 10];
    match s {
        Seam::Ready(v) => {
            let n = super::format_u32::format_u32(*v, &mut b);
            super::ui::metric(fb, x, y, w, label, &b[..n], b"", ACCENT);
        }
        Seam::Pending => super::ui::metric(fb, x, y, w, label, b"...", b"", MUTED),
        Seam::NotWired => super::ui::metric(fb, x, y, w, label, b"--", b"not wired", MUTED),
        Seam::Failed(_) => super::ui::metric(fb, x, y, w, label, b"err", b"", WARN),
    }
}

fn tile_wei(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &[u8], s: &Seam<[u8; 32]>) {
    match s {
        Seam::Ready(bytes) => {
            let mut lo = [0u8; 16];
            lo.copy_from_slice(&bytes[16..]);
            let eth = (u128::from_be_bytes(lo) / 1_000_000_000_000_000_000) as u64;
            let mut b = [0u8; 20];
            let n = super::format_u64::format_u64(eth, &mut b);
            super::ui::metric(fb, x, y, w, label, &b[..n], b"ETH", if eth > 0 { ACCENT } else { FG });
        }
        Seam::Pending => super::ui::metric(fb, x, y, w, label, b"...", b"", MUTED),
        Seam::NotWired => super::ui::metric(fb, x, y, w, label, b"--", b"not wired", MUTED),
        Seam::Failed(_) => super::ui::metric(fb, x, y, w, label, b"err", b"", WARN),
    }
}
