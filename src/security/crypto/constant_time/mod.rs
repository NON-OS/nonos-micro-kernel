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

pub mod aes;
pub mod core;
pub mod ed25519;
pub mod init;
pub mod memory;
pub mod ops;
pub mod types;
pub mod x25519;

pub use self::core::{
    ct_compare, ct_select_slice, ct_select_u32, ct_select_u64, ct_select_u8, ct_swap_slices,
    ct_verify,
};
pub use init::init;
pub use memory::{ct_hmac_verify, ct_signature_verify, ct_zero, ct_zero_u64};
pub use ops::{
    ct_copy_bounded, ct_eq_u32, ct_eq_u64, ct_gt_u32, ct_lt_u32, ct_lt_u64, ct_max_u32, ct_min_u32,
};
pub use types::{CtVerifyResult, SelfCheckResult, TimingMode};

pub mod ed25519_ct {
    pub use super::ed25519::*;
}

pub mod x25519_ct {
    pub use super::x25519::*;
}

pub mod aes_ct {
    pub use super::aes::*;
}
