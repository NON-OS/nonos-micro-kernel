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
#[cfg(target_arch = "aarch64")]
mod aarch64;
// Anything with no trap of its own gets the stub, which reports ENOSYS rather
// than pretending a call happened.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
mod fallback;
#[cfg(target_arch = "x86_64")]
mod x86;
#[cfg(target_arch = "x86_64")]
mod x86_asm;

#[cfg(target_arch = "aarch64")]
pub(super) use aarch64::raw;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub(super) use fallback::raw;
#[cfg(target_arch = "x86_64")]
pub(super) use x86::raw;
