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

mod browse;
mod downloads;
mod home;
mod lib_geom;
mod library;
mod nowplaying;
mod radio;
mod search;
mod settings;
mod settings_view;

pub use browse::paint as browse;
pub use downloads::{paint as downloads, row_at as downloads_row_at, visible as downloads_visible};
pub use radio::{paint as radio, tile_at as radio_tile_at};
pub use home::{card_at, paint as home};
pub use lib_geom::{row_at as lib_row_at, rows_for, tab_hit, visible as lib_visible};
pub use library::paint as library;
pub use nowplaying::paint as nowplaying;
pub use search::{paint as search, row_at as search_row_at, play_at as search_play_at};
pub use settings::{hit as settings_hit, Hit as SettingsHit};
pub use settings_view::paint as settings;
