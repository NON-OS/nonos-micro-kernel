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

use crate::settings::state::status::{Status, StatusKind};
use crate::settings::theme::{STATUS_BG, STATUS_FG_ERR, STATUS_FG_IDLE, STATUS_FG_OK};

use super::layout::{PAD_X, STATUS_H};

// At 9px a glyph this has to stay under about 80 characters or it runs past
// the right edge of a default-width window.
const HINT: &[u8] =
    b"[Tab] tabs  [arrows] move/adjust  [PgUp/PgDn] page  [Enter] toggle  [Esc] close";

/// Which set of keys is live, so the hint describes the panel the user is
/// actually looking at rather than always the field list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HintMode {
    Browsing,
    Editing,
    Wifi,
    WifiPassphrase,
}

fn hint_for(mode: HintMode) -> &'static [u8] {
    match mode {
        HintMode::Browsing => HINT,
        HintMode::Editing => b"[type] edit  [Enter] save  [Esc] cancel",
        HintMode::Wifi => b"[Up/Down] pick  [Enter] join  [Space] rescan  [Tab] tabs  [Esc] close",
        HintMode::WifiPassphrase => b"[type] passphrase  [Enter] join  [Esc] cancel",
    }
}

pub fn paint_status(fb: &mut PaintBuffer, status: &Status, ready: bool, mode: HintMode) {
    // Against the manifest height this painted off the bottom of a shorter
    // window, taking policy errors with it.
    let y = fb.height.saturating_sub(STATUS_H);
    fb.fill_rect(0, y, fb.width, STATUS_H, STATUS_BG);
    if !ready {
        fb.text(PAD_X, y + 6, b"policy unavailable; showing static defaults", STATUS_FG_ERR);
        return;
    }
    let text = status.as_slice();
    if text.is_empty() {
        fb.text(PAD_X, y + 6, hint_for(mode), STATUS_FG_IDLE);
        return;
    }
    let color = match status.kind {
        StatusKind::Ok => STATUS_FG_OK,
        StatusKind::Error => STATUS_FG_ERR,
        StatusKind::Idle => STATUS_FG_IDLE,
    };
    fb.text(PAD_X, y + 6, text, color);
}
