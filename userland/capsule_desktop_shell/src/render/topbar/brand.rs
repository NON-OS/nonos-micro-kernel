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

use super::metrics::{logo_size, logo_x, wordmark_x, WORDMARK, WORDMARK_TEXT};
use crate::render::icons::draw_logo;
use crate::render::text_aa::text_aa_bold;
use crate::render::ui_font::{top_y_centered, BRAND_PX};
use crate::state::Context;

pub(super) fn brand(ctx: &Context) {
    let bar_h = crate::render::layout::menubar_height();
    let logo_y = bar_h.saturating_sub(logo_size()) / 2;
    draw_logo(ctx, logo_x(), logo_y, logo_size());
    let text_y = top_y_centered(0, bar_h, BRAND_PX);
    text_aa_bold(ctx, wordmark_x(), text_y, WORDMARK_TEXT, WORDMARK, BRAND_PX);
}
