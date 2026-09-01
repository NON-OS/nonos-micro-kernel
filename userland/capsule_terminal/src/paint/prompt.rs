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

//! What sits to the left of the line being typed.

use nonos_app_skeleton::PaintBuffer;

use super::constants::TEXT_LEFT;
use super::line_text::text;
use crate::term::state::State;
use crate::term::theme::types::Theme;

/// Draw the prompt, and say how many cells it took.
///
/// Two heads, because the line means different things in the two modes. Under
/// a search the location is replaced by what is being searched for: the
/// location is not what the reader is looking at, and the line below is a
/// match rather than something they typed.
pub fn draw_prompt(
    state: &State,
    fb: &mut PaintBuffer,
    y: u32,
    adv: u32,
    px: f32,
    room: usize,
    t: &Theme,
) -> usize {
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min(room.max(1));

    if let Some(search) = &state.search {
        let shown = take.min(search.needle.len());
        text(fb, TEXT_LEFT, y, b"?", t.accent, adv, px);
        text(fb, TEXT_LEFT + adv, y, &search.needle[..shown], t.fg, adv, px);
        return 1 + shown + 1;
    }

    // The mark takes the colour of what the last command did, so a reader who
    // looked away while it ran learns the outcome where they are about to
    // type rather than by finding the block it came from.
    let mark = if state.last_status == 0 { t.accent } else { t.err };
    text(fb, TEXT_LEFT, y, b">", mark, adv, px);
    text(fb, TEXT_LEFT + adv, y, &cwd[cwd.len() - take..], t.path, adv, px);
    1 + take + 1
}
