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

pub const PRIVACY: Glyph = Glyph {
    strokes: &[&[(4, 9), (16, 9), (16, 18), (4, 18), (4, 9)]],
    arcs: &[Arc { cx: 10, cy: 9, rx: 4, ry: 4, from: 0, sweep: 32 }],
    wedges: &[],
    dots: &[(10, 13)],
};

pub const SOUND: Glyph = Glyph {
    strokes: &[&[(3, 8), (6, 8), (10, 4), (10, 16), (6, 12), (3, 12), (3, 8)]],
    arcs: &[
        Arc { cx: 10, cy: 10, rx: 4, ry: 5, from: 56, sweep: 16 },
        Arc { cx: 10, cy: 10, rx: 7, ry: 9, from: 56, sweep: 16 },
    ],
    wedges: &[],
    dots: &[],
};

pub const STORAGE: Glyph = Glyph {
    strokes: &[&[(4, 5), (4, 15)], &[(16, 5), (16, 15)]],
    arcs: &[
        Arc { cx: 10, cy: 5, rx: 6, ry: 3, from: 0, sweep: 64 },
        Arc { cx: 10, cy: 10, rx: 6, ry: 3, from: 32, sweep: 32 },
        Arc { cx: 10, cy: 15, rx: 6, ry: 3, from: 32, sweep: 32 },
    ],
    wedges: &[],
    dots: &[],
};

pub const UPDATES: Glyph = Glyph {
    strokes: &[],
    arcs: &[Arc { cx: 10, cy: 10, rx: 7, ry: 7, from: 22, sweep: 36 }],
    wedges: &[Arc { cx: 14, cy: 16, rx: 5, ry: 5, from: 7, sweep: 7 }],
    dots: &[],
};

pub const DEVELOPER: Glyph = Glyph {
    strokes: &[&[(7, 5), (2, 10), (7, 15)], &[(13, 5), (18, 10), (13, 15)], &[(11, 3), (9, 17)]],
    arcs: &[],
    wedges: &[],
    dots: &[],
};
