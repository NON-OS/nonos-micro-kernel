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

//! How much of the store each kind of node may take.
//!
//! The store holds a fixed number of nodes and the network publishes far more
//! than that, so the three kinds have to be budgeted against each other. Left
//! unbudgeted the first list read fills the store and the ones after it are
//! dropped, which is how a directory ends up with mix layers and no gateway
//! to leave by.

/// Mix hops. The most of the three, because a route picks one per layer and
/// picking from a wider set is what makes two routes differ.
pub const MIX_BUDGET: usize = 60;

/// Gateways to hold a session with. A client uses one at a time and changes
/// it rarely, so a modest spread is enough to not always enter the same way.
pub const ENTRY_BUDGET: usize = 34;

/// Gateways to leave by. Same reasoning as entry, and the two are kept equal
/// so neither starves the other.
pub const EXIT_BUDGET: usize = 34;

// The store refuses a list longer than it can hold, and it refuses the whole
// list rather than the tail, so a budget that overruns loses everything
// rather than the excess.
const _: () = assert!(MIX_BUDGET + ENTRY_BUDGET + EXIT_BUDGET <= crate::topology::NODE_CAP);
