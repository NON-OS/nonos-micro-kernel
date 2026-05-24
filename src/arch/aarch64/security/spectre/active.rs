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

use alloc::vec::Vec;

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

use super::{is_csv2_enabled, SpectreMitigation};

pub fn mitigations_active() -> Vec<SpectreMitigation> {
    let mut active = Vec::new();
    if has_feature(CpuFeature::Ssbs) {
        active.push(SpectreMitigation::Ssbs);
    }
    if is_csv2_enabled() {
        active.push(SpectreMitigation::CsvEnabled);
    }
    if has_feature(CpuFeature::Bti) {
        active.push(SpectreMitigation::BranchTargetId);
    }
    if active.is_empty() {
        active.push(SpectreMitigation::None);
    }
    active
}
