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

use super::super::ui;
use crate::wallet::swap::Token;
use crate::wallet::theme::INK;

const SIZE: u32 = 26;

/// The token's mark, drawn rather than fetched.
///
/// A logo pulled from a server names the asset to whoever serves it, every
/// time the window opens and before anything is signed. That is a better
/// signal than most chain analysis and it is paid for nothing: the reader
/// already knows what NOX looks like, and a tinted disc with a letter in it
/// is as identifiable at this size as a downloaded image would be.
pub fn mark(fb: &mut PaintBuffer, x: u32, y: u32, t: &Token) {
    ui::bordered(fb, x, y, SIZE, SIZE, t.tint, t.tint);
    let w = fb.measure_ttf(t.mark, 14.0).max(0) as u32;
    let cx = x + SIZE / 2 - w / 2;
    let _ = fb.text_ttf(cx as i32, (y + 5) as i32, t.mark, INK(), 14.0);
}
