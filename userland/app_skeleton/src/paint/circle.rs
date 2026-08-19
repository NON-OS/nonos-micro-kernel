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


use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn circle(&mut self, cx: u32, cy: u32, r: u32, argb: u32) {
        if r == 0 {
            return;
        }
        let d = r * 2;
        self.fill_round(cx.saturating_sub(r), cy.saturating_sub(r), d, d, r, argb);
    }

    pub fn ring(&mut self, cx: u32, cy: u32, r: u32, t: u32, argb: u32) {
        if r == 0 || t == 0 {
            return;
        }
        let d = r * 2;
        self.stroke_round(cx.saturating_sub(r), cy.saturating_sub(r), d, d, r, t, argb);
    }
}
