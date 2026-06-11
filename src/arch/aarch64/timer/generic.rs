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

mod convert;
mod count;
mod freq;
mod offset;

pub use convert::{
    microseconds_to_ticks, milliseconds_to_ticks, nanoseconds_to_ticks, ticks_to_microseconds,
    ticks_to_milliseconds, ticks_to_nanoseconds,
};
pub use count::{current_count, virtual_count};
pub use freq::frequency;
pub use offset::{physical_to_virtual, virtual_offset, virtual_to_physical};
