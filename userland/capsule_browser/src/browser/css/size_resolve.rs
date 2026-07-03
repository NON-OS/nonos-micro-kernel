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

use super::computed::Size;

impl Size {
    // Whole pixels against a containing base, None when auto. Percentages
    // and the per-mille part of calc() resolve against `base`.
    pub fn resolve(self, base: i32) -> Option<i32> {
        match self {
            Size::Auto => None,
            Size::Px(p) => Some(p as i32),
            Size::Pct(p) => Some(base.saturating_mul(p.min(1000) as i32) / 100),
            Size::Calc(px, pml) => {
                Some(px.saturating_add((base as i64 * pml as i64 / 1000) as i32))
            }
        }
    }

    // A definite pixel size with no base available: plain pixels, or a
    // calc() whose percentage part is zero.
    pub fn definite_px(self) -> Option<i32> {
        match self {
            Size::Px(p) => Some(p as i32),
            Size::Calc(px, 0) => Some(px),
            _ => None,
        }
    }
}
