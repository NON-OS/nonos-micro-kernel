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

//! The machine root, fetched and given back.

use nonos_libc::machine_key;
use nonos_vault::ROOT_LABEL;

/// The root every capsule's records derive from.
///
/// Held for exactly the length of one seal or open. `nonos_vault` takes it as
/// an argument rather than fetching it, so the decision about how long a root
/// lives in memory belongs here, where the answer is: not longer than the
/// operation that needed it.
pub(super) fn machine_root() -> Result<[u8; 32], i64> {
    machine_key(ROOT_LABEL)
}
