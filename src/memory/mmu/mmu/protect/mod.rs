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

// The register modules speak x86_64 instructions and are only declared where
// those assemble. `apply` carries the per-architecture split and reports the
// architectural properties directly on the targets that have no CR4.
mod apply;
#[cfg(target_arch = "x86_64")]
mod cpuid;
#[cfg(target_arch = "x86_64")]
mod cr0;
#[cfg(target_arch = "x86_64")]
mod cr4;
#[cfg(target_arch = "x86_64")]
mod efer;
mod report;

pub(super) use apply::apply;
pub use report::report;
