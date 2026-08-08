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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFeature {
    Fp,
    Asimd,
    Aes,
    Pmull,
    Sha1,
    Sha256,
    Crc32,
    Atomics,
    Rdm,
    Sha3,
    Sm3,
    Sm4,
    Dp,
    Fhm,
    Ts,
    Flagm,
    Ssbs,
    Sb,
    Pauth,
    /// FEAT_MTE2: tag checking at EL1, which is what GCR_EL1 and the SCTLR
    /// tag-access bits belong to. Plain MTE reports 1 and has neither.
    Mte2,
    Dcpop,
    Dcpodp,
    Sve,
    Sve2,
    Sme,
    Bti,
    Mte,
    Rng,
    /// FEAT_PAN: EL1 accesses to EL0-accessible pages fault unless PSTATE.PAN
    /// is cleared. The aarch64 counterpart of SMAP.
    Pan,
}
