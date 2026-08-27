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

mod basic;
mod kinds;
mod prog_base;
mod prog_four;
mod prog_hex;
mod prog_one;
mod prog_seven;
mod programmer;
mod row_four;
mod row_function;
mod row_memory;
mod row_one;
mod row_seven;
mod row_zero;
mod sci_const;
mod sci_inverse;
mod sci_logs;
mod sci_power;
mod sci_tail;
mod sci_trig;
mod scientific;
mod table;

pub use kinds::{Action, Button, Role};
pub use table::grid;
