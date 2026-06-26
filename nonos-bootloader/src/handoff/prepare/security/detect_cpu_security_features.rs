// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use core::arch::x86_64::{__cpuid, __cpuid_count};

pub fn detect_cpu_security_features() -> (bool, bool, bool) {
    let max_leaf = __cpuid(0).eax;
    if max_leaf < 7 {
        return (false, false, false);
    }
    let cpuid7 = __cpuid_count(7, 0);
    let smep = (cpuid7.ebx & (1 << 7)) != 0;
    let smap = (cpuid7.ebx & (1 << 20)) != 0;
    let umip = (cpuid7.ecx & (1 << 2)) != 0;
    (smep, smap, umip)
}
