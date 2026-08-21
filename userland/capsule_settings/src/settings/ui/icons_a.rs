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

use super::icon_glyph::{Arc, Glyph};

const fn ring(cx: i8, cy: i8, r: i8) -> Arc {
    Arc { cx, cy, rx: r, ry: r, from: 0, sweep: 64 }
}

pub const GENERAL: Glyph = Glyph {
    strokes: &[&[(3, 6), (17, 6)], &[(3, 10), (17, 10)], &[(3, 14), (17, 14)]],
    arcs: &[ring(7, 6, 2), ring(13, 10, 2), ring(7, 14, 2)],
    wedges: &[],
    dots: &[],
};

pub const NETWORK: Glyph = Glyph {
    strokes: &[&[(2, 10), (18, 10)]],
    arcs: &[ring(10, 10, 8), Arc { cx: 10, cy: 10, rx: 4, ry: 8, from: 0, sweep: 64 }],
    wedges: &[],
    dots: &[],
};

pub const WIFI: Glyph = Glyph {
    strokes: &[],
    arcs: &[
        Arc { cx: 10, cy: 15, rx: 10, ry: 10, from: 7, sweep: 18 },
        Arc { cx: 10, cy: 15, rx: 7, ry: 7, from: 7, sweep: 18 },
        Arc { cx: 10, cy: 15, rx: 4, ry: 4, from: 7, sweep: 18 },
    ],
    wedges: &[],
    dots: &[(10, 16)],
};

pub const SECURITY: Glyph = Glyph {
    strokes: &[
        &[(10, 2), (17, 5), (17, 10), (10, 18), (3, 10), (3, 5), (10, 2)],
        &[(6, 9), (9, 12), (14, 6)],
    ],
    arcs: &[],
    wedges: &[],
    dots: &[],
};

pub const APPEARANCE: Glyph = Glyph {
    strokes: &[],
    arcs: &[ring(10, 10, 8)],
    wedges: &[Arc { cx: 10, cy: 10, rx: 8, ry: 8, from: 48, sweep: 32 }],
    dots: &[],
};
