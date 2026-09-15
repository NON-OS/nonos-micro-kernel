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

use super::scale;
use nonos_app_skeleton::PaintBuffer;

use super::ui;
use crate::wallet::hex::short_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, GREEN_INK, MUTED};

// The card's own rhythm. The balance is set at the splash size, whose line box
// is around sixty-two pixels, and the rows underneath were placed as though it
// were forty: the ETH line began four pixels before the balance had finished, so
// the two collided on every screen that had an account. Deriving each row from
// the one above it means the next type change moves them together.
const PAD: u32 = 18;
const CAPTION_H: u32 = 26;
const BALANCE_H: u32 = 64;
/// Both cards on the home row are this tall. They used to differ by forty-eight
/// pixels, which left a hole under the shorter one that read as something having
/// failed to load rather than as a card that had said all it had to say.
pub const CARD_H: u32 = super::paint_network_card::NET_H;

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    /*
     * Before there is an account there is nothing to report, so the card stops
     * pretending to be a balance and becomes the one instruction that matters.
     */
    if !state.address_ready {
        super::paint_account_empty::paint_account_empty(fb, x, y, w, CARD_H);
        return;
    }
    ui::card(fb, x, y, w, CARD_H);
    let caption_y = y + PAD;
    let balance_y = caption_y + CAPTION_H;
    let second_y = balance_y + BALANCE_H;

    /*
     * The real account address, short form.
     */
    {
        let mut sa = [0u8; 13];
        short_addr(&state.address, &mut sa);
        let label = core::str::from_utf8(&sa).unwrap_or("");
        let lx = fb.text_ttf(
            (x + 20) as i32,
            caption_y as i32,
            "TOTAL BALANCE  \u{00b7}  ",
            DIM(),
            scale::BODY,
        );
        let _ = fb.text_ttf(lx, caption_y as i32, label, DIM(), scale::BODY);
        let aw = fb.measure_ttf("ACTIVE", scale::BODY).max(0) as u32 + 18;
        ui::badge(
            fb,
            x + w - 20 - aw,
            caption_y.saturating_sub(3),
            b"ACTIVE",
            GREEN(),
            GREEN_INK(),
        );
        /*
         * Whether the machine can keep this wallet past the next boot, beside
         * the badge that says it is live now.
         */
        let badge_y = caption_y.saturating_sub(3);
        super::account_custody::badge(state, fb, x + w - 20 - aw - 8, badge_y);
    }

    /*
     * Headline the NOX balance (the native token), with the live ETH balance on
     * the line beneath it. Each shows a fetching mark while its read is in
     * flight and a dash only when there is no route.
     */
    let up = state.net.rpc_chain_ok;
    let mut nb = [0u8; 48];
    let nox = crate::wallet::nox::live_amount(
        state.nox.balance_ready,
        &state.nox.balance_wei,
        up,
        &mut nb,
    );
    let pen = fb.text_ttf((x + 20) as i32, balance_y as i32, nox, FG(), scale::SPLASH);
    /*
     * The ticker sits on the balance's baseline rather than its top edge, so a
     * taller figure does not leave it floating.
     */
    let _ = fb.text_ttf(pen + 10, (balance_y + 18) as i32, "NOX", ACCENT(), scale::TITLE);

    let mut eb = [0u8; 40];
    let eth = if state.balance_ready {
        let n = format_eth(lower_u64(&state.balance_wei), &mut eb);
        core::str::from_utf8(&eb[..n]).unwrap_or("0")
    } else if up {
        "\u{2026}"
    } else {
        "\u{2014}"
    };
    let ex = fb.text_ttf((x + 20) as i32, second_y as i32, eth, MUTED(), scale::BODY);
    let _ = fb.text_ttf(ex + 6, second_y as i32, "ETH", DIM(), scale::BODY);
}

fn format_eth(v: u64, out: &mut [u8]) -> usize {
    let whole = v / 1_000_000_000_000_000_000u64;
    let frac = ((v % 1_000_000_000_000_000_000u64) / 100_000_000_000_000u64) as u32;
    let mut wb = [0u8; 20];
    let wn = super::format_u64::format_u64(whole, &mut wb);
    out[..wn].copy_from_slice(&wb[..wn]);
    let mut n = wn;
    out[n] = b'.';
    n += 1;
    out[n] = b'0' + ((frac / 1000) % 10) as u8;
    out[n + 1] = b'0' + ((frac / 100) % 10) as u8;
    out[n + 2] = b'0' + ((frac / 10) % 10) as u8;
    out[n + 3] = b'0' + (frac % 10) as u8;
    n + 4
}

fn lower_u64(v: &[u8; 32]) -> u64 {
    u64::from_be_bytes([v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]])
}
