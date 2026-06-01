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

use crate::settings::manifest::{HEIGHT, WIDTH};
use crate::settings::state::status::{Status, StatusKind};
use crate::settings::theme::{STATUS_BG, STATUS_FG_ERR, STATUS_FG_IDLE, STATUS_FG_OK};

use super::layout::{PAD_X, STATUS_H};

const HINT: &[u8] = b"[Tab] tabs  [Up/Down] move  [Left/Right] adjust  [Enter] edit/toggle  [Esc] close";

pub fn paint_status(fb: &mut PaintBuffer, status: &Status, ready: bool) {
    let y = HEIGHT - STATUS_H;
    fb.fill_rect(0, y, WIDTH, STATUS_H, STATUS_BG);
    if !ready {
        fb.text(PAD_X, y + 6, b"policy unavailable; showing static defaults", STATUS_FG_ERR);
        return;
    }
    let text = status.as_slice();
    if text.is_empty() {
        fb.text(PAD_X, y + 6, HINT, STATUS_FG_IDLE);
        return;
    }
    let color = match status.kind {
        StatusKind::Ok => STATUS_FG_OK,
        StatusKind::Error => STATUS_FG_ERR,
        StatusKind::Idle => STATUS_FG_IDLE,
    };
    fb.text(PAD_X, y + 6, text, color);
}
