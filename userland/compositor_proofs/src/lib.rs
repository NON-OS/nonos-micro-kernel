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

//! Host proofs for the compositor's trusted-path primitives. The `#[path]`
//! includes pull in the real production source so the tests pin the shipping
//! behavior, not a copy. `state::damage` is re-exported here so the blitter's
//! `crate::state::damage::Rect` path resolves exactly as it does in-tree.

#[path = "../../compositor/src/state/damage.rs"]
pub mod damage;

pub mod state {
    pub mod damage {
        pub use crate::damage::Rect;
    }
}

#[path = "../../compositor/src/sw_blitter/mod.rs"]
pub mod sw_blitter;

#[cfg(test)]
mod blitter_tests;
#[cfg(test)]
mod damage_tests;
