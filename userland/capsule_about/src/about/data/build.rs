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

pub const VERSION: &[u8] = include_bytes!("../../../../../VERSION");
pub const GIT_SHA: &[u8] = env!("ABOUT_GIT_SHA").as_bytes();
pub const TOOLCHAIN: &[u8] = b"nightly-2026-01-16";

#[cfg(target_arch = "x86_64")]
pub const ARCH: &[u8] = b"x86_64";
#[cfg(target_arch = "aarch64")]
pub const ARCH: &[u8] = b"aarch64";
#[cfg(target_arch = "riscv64")]
pub const ARCH: &[u8] = b"riscv64";
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64")))]
pub const ARCH: &[u8] = b"unknown";
