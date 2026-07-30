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

//! The end of a boot.
//!
//! ZeroState is a claim about what the machine holds once it stops, so the
//! wipe cannot be a step a caller is trusted to remember. `terminate` is the
//! only route to `arch::power`, which is itself crate-private, so wiping is
//! not a convention here: it is the only reachable path.
//!
//! `verification/lean/Nonos/ZeroState.lean` proves the property this shape
//! buys, that no reachable state powers the machine off with memory unwiped.

mod terminate;

pub(crate) use terminate::terminate;
