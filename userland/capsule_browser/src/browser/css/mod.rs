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

mod apply;
mod apply_rules;
mod apply_style_attr;
mod bg_url;
mod border_parts;
mod budget;
mod cache;
mod calc;
mod calc_factor;
mod calc_term;
mod collect;
mod color;
mod compute;
mod computed;
mod decl;
mod grid_tracks;
mod hex;
mod hsl_fn;
mod matching;
mod matching_paren;
mod named;
mod one_track;
mod parse;
mod parse_grow;
mod parse_line_height;
mod parse_px;
mod parse_size;
mod rgb_fn;
mod rule;
mod rule_index;
mod select;
mod selector;
mod set_len;
mod sides;
mod size_resolve;
mod specificity;
mod strip_unit;
mod ua;
mod vars;
mod walk;

pub use cache::{compute_cached, CssCache};
pub use collect::collect_css;
pub use color::parse_color;
pub use compute::compute;
pub use computed::Shadow;
pub use computed::{Align, Computed, GridTrack, Justify, Position, Size, TextAlign, WhiteSpace};
pub use select::select;
