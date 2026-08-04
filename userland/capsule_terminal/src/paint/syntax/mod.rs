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

//! Colouring a command as it is typed.\n//!\n//! A line of one colour has to be read word by word to find where the command\n//! ends and its arguments begin. Colouring the parts differently means the\n//! shape is visible before it is read, which matters most when the line is\n//! long enough that reading it is the slow part.

mod classify;
mod part;
mod word;

pub use classify::classify;
pub use part::Part;
