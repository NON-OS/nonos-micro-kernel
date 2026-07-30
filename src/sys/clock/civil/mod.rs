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

//! Calendar arithmetic, owned in one place.
//!
//! No hardware here. The CMOS clock hands over a date and wants seconds; a
//! PL031 hands over seconds and the syscall wants a date. Both conversions are
//! the same maths, so it lives above the arch boundary and each side calls in.

mod days;
mod time;

pub use days::{days_in_month, days_in_year, is_leap_year};
pub use time::{from_unix, to_unix, CivilTime, UNIX_EPOCH_YEAR};
