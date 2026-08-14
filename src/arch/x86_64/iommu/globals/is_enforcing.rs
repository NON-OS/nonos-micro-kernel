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

//! Whether DMA is actually being remapped, as opposed to merely describable.

use core::sync::atomic::Ordering;

use super::state::ENFORCING;

/// True once a unit has Translation Enable set with this kernel's tables
/// installed. Every call that claims to confine a device checks this, because
/// writing a page table entry into tables no hardware consults protects
/// nothing while reporting success.
pub fn is_enforcing() -> bool {
    ENFORCING.load(Ordering::Acquire)
}

/// Record that a unit came into service. Called only from bring-up, after the
/// hardware acknowledged the enable.
pub fn set_enforcing() {
    ENFORCING.store(true, Ordering::Release);
}
