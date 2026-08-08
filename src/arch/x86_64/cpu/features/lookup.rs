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

use super::types::CpuFeatures;

pub fn has_feature(features: &CpuFeatures, name: &str) -> bool {
    match name {
        "sse" => features.sse,
        "sse2" => features.sse2,
        "sse3" => features.sse3,
        "ssse3" => features.ssse3,
        "sse4.1" | "sse4_1" => features.sse4_1,
        "sse4.2" | "sse4_2" => features.sse4_2,
        "avx" => features.avx,
        "avx2" => features.avx2,
        "avx512f" => features.avx512f,
        "aes" | "aes-ni" | "aesni" => features.aes_ni,
        "pclmulqdq" => features.pclmulqdq,
        "rdrand" => features.rdrand,
        "rdseed" => features.rdseed,
        "sha" => features.sha,
        "fma" => features.fma,
        "bmi1" => features.bmi1,
        "bmi2" => features.bmi2,
        "popcnt" => features.popcnt,
        "vmx" => features.vmx,
        "svm" => features.svm,
        "smep" => features.smep,
        "smap" => features.smap,
        "nx" => features.nx,
        "pcid" => features.pcid,
        "invpcid" => features.invpcid,
        "fsgsbase" => features.fsgsbase,
        "xsave" => features.xsave,
        "tsc" => features.tsc,
        "rdtscp" => features.rdtscp,
        "x2apic" => features.x2apic,
        "pku" => features.pku,
        "la57" => features.la57,
        _ => false,
    }
}
