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

//! Compare two desktop listings so a refresh only repaints on real changes.

use crate::vfs_client::Entry;

/// Two listings match when they hold the same names in the same order with the
/// same directory flags. The server returns entries in a stable order, so an
/// order-sensitive compare is both correct and cheap.
pub(super) fn same(a: &[Entry], b: &[Entry]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.is_dir == y.is_dir && x.name == y.name)
}
