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

//! The CPU generator, as the slide derivation sees it.

use crate::arch::cpu_random;

#[inline]
pub(super) fn has_cpu_random() -> bool {
    cpu_random::random_available()
}

#[inline]
pub(super) fn has_cpu_entropy() -> bool {
    cpu_random::entropy_available()
}

#[inline]
pub(super) fn cpu_random64() -> Option<u64> {
    cpu_random::random_u64()
}

#[inline]
pub(super) fn cpu_entropy64() -> Option<u64> {
    cpu_random::entropy_u64()
}

pub fn has_hardware_rng() -> bool {
    has_cpu_random() || has_cpu_entropy()
}
