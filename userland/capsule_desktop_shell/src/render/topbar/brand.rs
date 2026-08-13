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

//! The real NØNOS logo and wordmark on the left. Clicking here opens the app
//! launcher; see brand_hit.

use super::metrics::{LOGO_SIZE, LOGO_X, WORDMARK, WORDMARK_X};
use crate::render::icons::draw_logo;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{top_y_centered, TITLE_PX};
use crate::state::Context;

pub(super) fn brand(ctx: &Context) {
    let bar_h = crate::render::layout::MENUBAR_HEIGHT;
    let logo_y = bar_h.saturating_sub(LOGO_SIZE) / 2;
    draw_logo(ctx, LOGO_X, logo_y, LOGO_SIZE);
    let text_y = top_y_centered(0, bar_h, TITLE_PX);
    text_aa_bytes(ctx, WORDMARK_X, text_y, b"NONOS", WORDMARK, TITLE_PX);
}
