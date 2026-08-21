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

pub mod adjust;
pub mod adjust_i8;
pub mod adjust_u8;
pub mod clamp_u8;
pub mod commit_bool;
pub mod commit_string;
pub mod next_section;
pub mod on_event;
pub mod on_event_browsing;
pub mod on_event_editing;
pub mod on_event_wifi;
pub mod on_pointer;
pub mod on_search_key;
pub mod on_search_pointer;
pub mod pointer_row;
pub mod push_text_char;
pub mod report;
pub mod toggle_or_inc;

pub use on_event::on_event;
pub use on_search_pointer::on_accessory;
