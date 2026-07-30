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

//! FEAT_RNG presence, from ID_AA64ISAR0_EL1.RNDR.

use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

/// True iff the core implements FEAT_RNG. Both RNDR and RNDRRS are added by
/// that one feature, so a single probe answers for both taps.
pub fn has_rng() -> bool {
    has_feature(CpuFeature::Rng)
}
