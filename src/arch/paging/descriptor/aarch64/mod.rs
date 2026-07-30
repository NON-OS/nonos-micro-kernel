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

//! aarch64 stage 1 descriptors.
//!
//! Three differences from the neutral vocabulary, each silent if got wrong:
//! write permission is inverted, a block is the absence of a flag rather than
//! a flag, and the access flag is mandatory.

mod bits;
mod build;
mod read;

pub use bits::ADDR_MASK;
pub use build::{leaf, table};
pub use read::{address, is_block, is_present, is_user, is_writable, table_grants_user};
