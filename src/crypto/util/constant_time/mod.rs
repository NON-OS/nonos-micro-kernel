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

mod barriers;
mod compare;
mod copy;
mod lookup;
mod math;
mod select;

pub use barriers::{
    black_box, black_box_slice, compiler_fence, dummy_work, memory_fence, serialize_execution,
    time_constant_execute, volatile_read, volatile_write,
};

pub use compare::{
    ct_eq, ct_eq_16, ct_eq_32, ct_eq_64, ct_eq_u64, ct_gt_u64, ct_is_nonzero_u64, ct_is_zero_u64,
    ct_lt_u64,
};

pub use copy::{
    ct_conditional_move, ct_conditional_swap, ct_conditional_swap_32, ct_copy, secure_erase,
    secure_zero,
};

pub use lookup::{
    ct_is_nonzero_slice, ct_is_zero_slice, ct_lookup_u32, ct_lookup_u8, ct_lookup_u8_16,
};

pub use math::{
    ct_add_overflow_u64, ct_add_u64, ct_bswap_u32, ct_bswap_u64, ct_clz_u64, ct_conditional_negate,
    ct_mod_u64, ct_mul_u64, ct_popcount_u64, ct_sub_u64,
};

pub use select::{
    ct_select_u16, ct_select_u32, ct_select_u64, ct_select_u64_bit, ct_select_u8, ct_select_usize,
};
