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

use core::sync::atomic::AtomicBool;

pub(super) const CPUID_THERMAL_POWER: u32 = 0x06;
pub(super) const ARAT_BIT: u32 = 1 << 2;

pub(super) const MSR_IA32_POWER_CTL: u32 = 0x1FC;
pub(super) const POWER_CTL_C1E_ENABLE: u64 = 1 << 1;

pub(super) static HALT_SAFE: AtomicBool = AtomicBool::new(true);
