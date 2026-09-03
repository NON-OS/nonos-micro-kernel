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

pub mod card;
pub mod chip;
pub mod chip_wrap;
pub mod chrome;
pub mod gauge;
pub mod hit;
pub mod kv;
pub mod metrics;
pub mod nav_geom;
pub mod screens;
pub mod sidebar;
pub mod status_bar;
pub mod text;

mod paint;

pub use paint::paint as frame;
