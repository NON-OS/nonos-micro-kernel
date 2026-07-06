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

// Arbitrary-precision integers used by RSA. Self-contained apart from the
// constant-time fence already provided by `crate::crypto::constant_time`.
#[allow(
    clippy::needless_range_loop,
    clippy::manual_rotate,
    clippy::identity_op,
    clippy::unnecessary_cast,
    clippy::manual_is_multiple_of,
    clippy::useless_conversion,
    clippy::should_implement_trait,
    clippy::manual_memcpy,
    clippy::redundant_closure,
    clippy::wrong_self_convention,
    clippy::manual_div_ceil,
    clippy::needless_borrow
)]
#[path = "../../../../../src/crypto/util/bigint/mod.rs"]
pub mod bigint;

// Kernel code addresses the constant-time primitives as
// `crate::crypto::util::constant_time`; alias the single real module.
pub use crate::crypto::constant_time;
