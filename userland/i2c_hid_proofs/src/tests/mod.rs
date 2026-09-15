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

//! The touchpad driver from service lookup to posted pointer events.

mod configure_tests;
mod driven;
mod fixture;
mod frames;
mod gesture_multi_tests;
mod gesture_tests;
mod layout_tests;
mod parse_tests;
mod poll_scroll_tests;
mod poll_tests;
mod service;
mod setup_tests;
mod wake_tests;
