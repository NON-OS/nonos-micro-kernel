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

mod cpu;
mod memory;
mod simd;
mod simd_level;
mod simd_types;
mod sse;
mod sse_avx;
mod sse_enable;

pub use cpu::validate_cpu_features;
pub use memory::validate_memory;
pub use simd::{get_simd_support, SimdLevel, SimdSupport};
pub use sse::{enable_avx, enable_avx512, enable_sse, enable_sse_avx};
