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

use nonos_font::{text_width, GLYPH_HEIGHT};
use nonos_ui::Canvas;

use super::types::Button;

impl Button {
    pub(super) fn paint_inner(&self, canvas: &mut Canvas<'_>) {
        let bg = if self.pressed { self.accent } else { self.bg };
        canvas.fill_rect(self.rect, bg);
        let tw = text_width(&self.text);
        let tx = self.rect.x + self.rect.w.saturating_sub(tw) / 2;
        let ty = self.rect.y + self.rect.h.saturating_sub(GLYPH_HEIGHT) / 2;
        canvas.draw_text(tx, ty, &self.text, self.fg);
    }
}
