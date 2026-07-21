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

//! Curated component library. One visual language across every screen: elevated
//! cards, status badges, and buttons, all keyed off theme.rs and the 8px
//! spacing scale.

mod badge;
mod button;
mod card;

pub use badge::{badge, chip};
pub use button::{outline, primary};
pub use card::{bordered, card, edge};
