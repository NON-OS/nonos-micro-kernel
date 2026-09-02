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

/// NONOS mounts no filesystem to measure: there is no statfs syscall and the
/// capsule store is a raw signed region of the block device, not a volume with
/// a capacity. Every figure here is a standing gap, kept so the panel can say
/// so rather than the rail quietly omitting a row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Disk {
    pub total_kb: Metric<u64>,
    pub used_kb: Metric<u64>,
}

impl Disk {
    pub const UNSUPPORTED: Disk =
        Disk { total_kb: Metric::Unsupported, used_kb: Metric::Unsupported };
}
