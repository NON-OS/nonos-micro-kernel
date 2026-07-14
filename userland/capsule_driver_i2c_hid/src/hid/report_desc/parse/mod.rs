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

//! Walk a HID report descriptor and locate the touchpad's absolute X/Y, tip
//! switch, contact count and button fields. This is a focused reader: it tracks
//! only the global and local state those fields need, records the first contact
//! it sees (enough to drive the cursor), and is bounded so a malformed
//! descriptor can only ever yield fewer fields, never loop or read out of range.

mod assign;
mod parse;
mod read_le;
mod usage_for;

pub use parse::parse;
