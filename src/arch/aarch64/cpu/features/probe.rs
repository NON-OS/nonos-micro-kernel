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

use super::kind::CpuFeature;
use super::registers::{
    read_aa64isar0, read_aa64isar1, read_aa64mmfr1, read_aa64pfr0, read_aa64pfr1, read_aa64zfr0,
};

pub fn has_feature(feature: CpuFeature) -> bool {
    let aa64isar0 = read_aa64isar0();
    let aa64isar1 = read_aa64isar1();
    let aa64pfr0 = read_aa64pfr0();
    let aa64pfr1 = read_aa64pfr1();
    match feature {
        CpuFeature::Fp => (aa64pfr0 & 0xF) != 0xF,
        CpuFeature::Asimd => ((aa64pfr0 >> 4) & 0xF) != 0xF,
        CpuFeature::Aes => ((aa64isar0 >> 4) & 0xF) >= 1,
        CpuFeature::Pmull => ((aa64isar0 >> 4) & 0xF) >= 2,
        CpuFeature::Sha1 => ((aa64isar0 >> 8) & 0xF) >= 1,
        CpuFeature::Sha256 => ((aa64isar0 >> 12) & 0xF) >= 1,
        CpuFeature::Crc32 => ((aa64isar0 >> 16) & 0xF) >= 1,
        CpuFeature::Atomics => ((aa64isar0 >> 20) & 0xF) >= 2,
        CpuFeature::Rdm => ((aa64isar0 >> 28) & 0xF) >= 1,
        CpuFeature::Sha3 => ((aa64isar0 >> 32) & 0xF) >= 1,
        CpuFeature::Sm3 => ((aa64isar0 >> 36) & 0xF) >= 1,
        CpuFeature::Sm4 => ((aa64isar0 >> 40) & 0xF) >= 1,
        CpuFeature::Dp => ((aa64isar0 >> 44) & 0xF) >= 1,
        CpuFeature::Fhm => ((aa64isar0 >> 48) & 0xF) >= 1,
        CpuFeature::Ts => ((aa64isar0 >> 52) & 0xF) >= 1,
        CpuFeature::Flagm => ((aa64isar0 >> 52) & 0xF) >= 2,
        CpuFeature::Ssbs => ((aa64pfr1 >> 4) & 0xF) >= 1,
        CpuFeature::Sb => ((aa64isar1 >> 36) & 0xF) >= 1,
        CpuFeature::Pauth => has_pauth(aa64isar1),
        CpuFeature::Dcpop => (aa64isar1 & 0xF) >= 1,
        CpuFeature::Dcpodp => (aa64isar1 & 0xF) >= 2,
        CpuFeature::Sve => ((aa64pfr0 >> 32) & 0xF) >= 1,
        CpuFeature::Sve2 => (read_aa64zfr0() & 0xF) >= 1,
        CpuFeature::Sme => ((aa64pfr1 >> 24) & 0xF) >= 1,
        CpuFeature::Bti => (aa64pfr1 & 0xF) >= 1,
        CpuFeature::Mte => ((aa64pfr1 >> 8) & 0xF) >= 1,
        CpuFeature::Mte2 => ((aa64pfr1 >> 8) & 0xF) >= 2,
        CpuFeature::Rng => ((aa64isar0 >> 60) & 0xF) >= 1,
        CpuFeature::Pan => (read_aa64mmfr1() & 0xF) >= 1,
    }
}

fn has_pauth(aa64isar1: u64) -> bool {
    ((aa64isar1 >> 4) & 0xF) >= 1 || ((aa64isar1 >> 8) & 0xF) >= 1
}
