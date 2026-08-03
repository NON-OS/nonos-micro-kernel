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

//! Header padding.

use crate::sphinx::constants::{HEADER_INTEGRITY_MAC_SIZE, NODE_META_INFO_SIZE};

/// One routing slot: a hop's meta info plus the MAC that authenticates it.
pub const FILLER_STEP_SIZE_INCREASE: usize = NODE_META_INFO_SIZE + HEADER_INTEGRITY_MAC_SIZE;

mod build;
mod keystream;
mod step;

pub use build::build_filler;
