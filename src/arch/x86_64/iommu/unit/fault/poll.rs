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

use super::drain::drain_faults;
use crate::arch::x86_64::iommu::globals::is_enforcing;

/// Ticks between polls. One uncached status read every second or so costs
/// nothing measurable, and a device being denied is a standing condition, not
/// an event that has to be caught the instant it happens.
const INTERVAL: u64 = 128;

/// Called from the timer interrupt. Delivery by interrupt would need an MSI
/// vector programmed into FECTL; until that exists this is what makes a
/// denial visible rather than silent.
pub fn poll_faults(ticks: u64) {
    if !ticks.is_multiple_of(INTERVAL) {
        return;
    }
    if !is_enforcing() {
        return;
    }
    drain_faults();
}
