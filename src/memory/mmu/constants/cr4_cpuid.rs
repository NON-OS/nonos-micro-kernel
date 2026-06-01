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

pub const CR4_PGE: u64 = 1 << 7;
pub const CR4_UMIP: u64 = 1 << 11;
pub const CR4_SMEP: u64 = 1 << 20;
pub const CR4_SMAP: u64 = 1 << 21;

pub const CPUID_FEATURES_LEAF: u32 = 0x07;
pub const CPUID_EBX_SMEP: u32 = 1 << 7;
pub const CPUID_EBX_SMAP: u32 = 1 << 20;
pub const CPUID_ECX_UMIP: u32 = 1 << 2;
pub const CPUID_EXTENDED_LEAF: u32 = 0x8000_0001;
pub const CPUID_EDX_NX: u32 = 1 << 20;

pub const MSR_IA32_EFER: u32 = 0xC000_0080;
pub const EFER_NXE: u64 = 1 << 11;
