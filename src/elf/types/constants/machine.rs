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

pub const EM_NONE: u16 = 0;
pub const EM_386: u16 = 3;
pub const EM_X86_64: u16 = 62;
pub const EM_AARCH64: u16 = 183;
pub const EM_RISCV: u16 = 243;

/// The machine this kernel loads capsules for.
///
/// A capsule is native code, so an image built for another architecture has to
/// be refused rather than run. Naming it once keeps every validator agreeing
/// on which architecture that is.
#[cfg(target_arch = "x86_64")]
pub const EM_NATIVE: u16 = EM_X86_64;
#[cfg(target_arch = "aarch64")]
pub const EM_NATIVE: u16 = EM_AARCH64;
#[cfg(target_arch = "riscv64")]
pub const EM_NATIVE: u16 = EM_RISCV;

pub const NONE: u16 = 0;
pub const X86_64: u16 = 62;
