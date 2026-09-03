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

mod chain;
mod prose;
mod prose_wrap;
mod runtime_meter;
mod runtime_text;
mod tile_text;

mod display_surface;
mod licenses_banner;
mod licenses_cols;
mod licenses_table;
mod licenses_text;
mod overview_cards;
mod overview_hero;
mod overview_mark;
mod overview_tiles;
mod system_build;
mod system_runtime;
mod system_space;
mod system_uptime;
mod trust_caps;
mod trust_caps_list;
mod trust_chain;
mod trust_hops;

pub mod display;
pub mod licenses;
pub mod overview;
pub mod system;
pub mod trust;

mod extent;
pub use extent::content_h;
