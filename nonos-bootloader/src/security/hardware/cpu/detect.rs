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

#[cfg(target_arch = "x86_64")]
use super::cpuid::{cpuid_extended_leaf, cpuid_leaf_1, cpuid_leaf_7};
use super::types::CpuSecurityFeatures;

pub fn detect_cpu_security_features() -> CpuSecurityFeatures {
    let mut f = CpuSecurityFeatures::default();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        detect_x86(&mut f);
    }
    f
}

#[cfg(target_arch = "x86_64")]
unsafe fn detect_x86(f: &mut CpuSecurityFeatures) {
    let (ebx7, ecx7, edx7) = cpuid_leaf_7();
    let (ecx1, _) = cpuid_leaf_1();
    let edx_ext = cpuid_extended_leaf();
    f.smep = (ebx7 & (1 << 7)) != 0;
    f.smap = (ebx7 & (1 << 20)) != 0;
    f.umip = (ecx7 & (1 << 2)) != 0;
    f.ibrs = (edx7 & (1 << 26)) != 0;
    f.stibp = (edx7 & (1 << 27)) != 0;
    f.aes_ni = (ecx1 & (1 << 25)) != 0;
    f.rdrand = (ecx1 & (1 << 30)) != 0;
    f.rdseed = (ebx7 & (1 << 18)) != 0;
    f.sha_ext = (ebx7 & (1 << 29)) != 0;
    f.nx_bit = (edx_ext & (1 << 20)) != 0;
    f.tpm_support = true;
}
