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

mod card;
mod event;
mod geom;
mod paint;
mod pane;
mod rail;
mod sect;
mod sect_event;
mod sect_paint;
mod sect_state;
mod sects_a;
mod sects_b;
mod state;
mod style;

pub(crate) use event::settings_event;
pub(crate) use paint::paint_settings;
