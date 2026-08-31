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

mod board;
mod board_cell;
mod board_fit;
mod board_pieces;
mod board_snake;
mod diamond;
mod glow;
mod home;
mod home_cards;
mod num;
mod num_clock;
mod over;
mod over_summary;
mod pause;
mod play;
mod play_foot;
mod play_hud;
mod play_rail;
mod play_rail_tip;
mod rank;
mod rank_awards;
mod rank_rows;
mod receipt;
mod screen;
mod setup;
mod setup_chips;
mod setup_toggles;
mod wrap;

pub use screen::paint;
