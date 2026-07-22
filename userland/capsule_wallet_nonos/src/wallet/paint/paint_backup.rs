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
use crate::wallet::theme::{ACCENT, DIM, FG, MUTED, PANEL_2};

/// The one-time recovery-phrase screen. The words render from state that is
/// volatile-wiped the moment the user confirms, and nothing here ever writes
/// them anywhere else: no clipboard, no log, no file. Numbered 4x3 grid so
/// transcription order is unambiguous.
pub fn paint_backup(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    ui::card(fb, cx, 146, cw, 470);

    let _ = fb.text_ttf((cx + 20) as i32, 170, "RECOVERY PHRASE", ACCENT(), 13.2);
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        196,
        "Write these words down, in order, on paper. They are shown exactly once.",
        FG(),
        15.5,
    );
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        220,
        "Anyone holding this phrase controls the account. Never type it into a website, never photograph it.",
        DIM(),
        13.2,
    );

    let count = state.backup_count as usize;
    let cols = 4u32;
    let cell_w = (cw.saturating_sub(40 + 3 * 16)) / cols;
    let cell_h = 64u32;
    for (i, &word_index) in state.backup_words.iter().take(count).enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        let bx = cx + 20 + col * (cell_w + 16);
        let by = 244 + row * (cell_h + 12);
        ui::bordered(fb, bx, by, cell_w, cell_h, PANEL_2(), DIM());

        let mut num = [0u8; 20];
        let n = super::format_u64::format_u64((i + 1) as u64, &mut num);
        let _ = fb.text_ttf(
            (bx + 12) as i32,
            (by + 14) as i32,
            core::str::from_utf8(&num[..n]).unwrap_or(""),
            MUTED(),
            12.1,
        );

        let word = nonos_hd::ENGLISH_WORDLIST.get(word_index as usize).copied().unwrap_or("");
        let _ = fb.text_ttf_mono((bx + 12) as i32, (by + 34) as i32, word, FG(), 17.2);
    }

    ui::primary(fb, cx + 20, 560, 280, b"I wrote them down (Enter)");
    let _ = fb.text_ttf(
        (cx + 320) as i32,
        574,
        "Confirming wipes the phrase from this machine.",
        DIM(),
        13.2,
    );
}
