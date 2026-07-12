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

mod bit_ops;
mod contiguous;
mod count;
mod index;
mod range;

pub(super) use bit_ops::{bit_clear, bit_set, bit_test};
pub(super) use contiguous::find_contiguous_free;
pub(super) use count::{count_free_bits, find_first_free};
pub(super) use range::{clear_bit_range, is_range_allocated, set_bit_range};
