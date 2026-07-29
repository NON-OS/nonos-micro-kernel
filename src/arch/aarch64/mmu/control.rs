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

//! The three registers that have to be right before the MMU is switched on.
//!
//! MAIR_EL1 says what each memory-type slot means, TCR_EL1 describes the shape
//! of the translation tables the walker will read, and SCTLR_EL1 turns the
//! walker on. Each gets its own file because each has its own field layout to
//! justify.

mod id;
mod mair;
mod sctlr;
mod tcr;

pub(super) use mair::configure_mair;
pub(super) use sctlr::enable_mmu;
pub(super) use tcr::configure_tcr;
