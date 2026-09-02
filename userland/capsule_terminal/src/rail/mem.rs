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

use super::metrics::Proc;
use super::value::Metric;

/// The memory panel's figures. Resident use is a real sum over the live process
/// table. The kernel exposes no physical memory size and NONOS has no swap
/// subsystem, so neither a total nor a swap figure exists to report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mem {
    pub used_kb: Metric<u64>,
    pub total_kb: Metric<u64>,
    pub swap_used_kb: Metric<u64>,
}

impl Mem {
    pub const UNKNOWN: Mem = Mem {
        used_kb: Metric::Unknown,
        total_kb: Metric::Unsupported,
        swap_used_kb: Metric::Unsupported,
    };
}

/// A live process table always has a resident sum, including the degenerate
/// zero; an empty one is a failed read rather than a machine using no memory.
pub fn summarize(procs: &[Proc]) -> Mem {
    if procs.is_empty() {
        return Mem::UNKNOWN;
    }
    let used = procs.iter().fold(0u64, |acc, p| acc.saturating_add(p.mem_kb));
    Mem { used_kb: Metric::Known(used), ..Mem::UNKNOWN }
}
