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

//! The section-03 primitives, drawn straight into the frame buffer.

mod button;
mod card;
mod chip;
mod eq;
mod header;
mod hero;
mod row;
mod slider;
mod switch;
mod tile;
mod wave;

pub use button::{button, measure as button_w, Variant};
pub use card::{card, card_h};
pub use chip::{chip, measure as chip_w};
pub use hero::{action_at as hero_action_at, hero, HERO_H};
pub use header::{card_panel, page_header, section_header};
pub use row::{header as table_header, row, Flags, ROW_H};
pub use slider::{permille, slider};
pub use switch::{switch, tab_at, tab_bar, SWITCH_H, SWITCH_W};
pub use wave::waveform;
pub use tile::{stat_tile, storage_meter, TILE_H};
