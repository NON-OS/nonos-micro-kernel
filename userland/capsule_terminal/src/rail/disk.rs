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

use super::value::Metric;

/// The VFS reports the bytes its store currently holds, so `used_kb` is a real
/// measurement. It has no byte ceiling to report against: the store is bounded
/// by a 2048-slot file table, and the 16 MiB budget in the block layer covers
/// only the persisted extents rather than the namespace this figure sums. A
/// total is therefore a standing gap the panel says out loud.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Disk {
    pub total_kb: Metric<u64>,
    pub used_kb: Metric<u64>,
}

impl Disk {
    pub const UNKNOWN: Disk = Disk { total_kb: Metric::Unsupported, used_kb: Metric::Unknown };
}
