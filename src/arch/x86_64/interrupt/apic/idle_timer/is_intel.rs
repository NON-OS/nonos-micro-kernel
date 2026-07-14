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

use crate::arch::x86_64::boot::cpu_ops::cpuid;

pub(super) fn is_intel() -> bool {
    let (_, ebx, ecx, edx) = cpuid(0);
    // "GenuineIntel" == ebx="Genu", edx="ineI", ecx="ntel"
    ebx == 0x756E_6547 && edx == 0x4965_6E69 && ecx == 0x6C65_746E
}
