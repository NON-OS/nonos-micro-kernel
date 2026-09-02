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

/// Share of the scheduler's elapsed ticks a run-tick delta accounts for, as a
/// whole percent. A zero total delta is the first poll or a repeat inside one
/// tick and has no denominator, so it reads as zero rather than dividing.
pub fn cpu_pct(run_delta: u64, total_delta: u64) -> u32 {
    if total_delta == 0 {
        return 0;
    }
    (run_delta.saturating_mul(100) / total_delta).min(100) as u32
}
