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

mod accept_suggestion;
mod bool_to_outcome;
mod complete;
mod copy_line;
mod fg_stdin;
mod on_ctrl;
mod on_down;
mod on_enter;
mod on_event;
mod on_key;
mod on_printable;
mod on_tab;
mod on_up;
mod paste_clipboard;
pub(crate) mod search;
pub(crate) mod search_edit;
mod search_place;

#[cfg(feature = "nonos-autorun-selftest")]
pub use on_enter::on_enter;
pub use on_event::on_event;
