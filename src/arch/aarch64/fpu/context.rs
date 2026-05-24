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





#[repr(C, align(16))]
#[derive(Clone)]
pub struct FpSimdContext {
    pub q: [u128; 32],
    pub fpsr: u32,
    pub fpcr: u32,
    _pad: [u32; 2],
}

impl FpSimdContext {
    pub const fn zeroed() -> Self {
        Self { q: [0u128; 32], fpsr: 0, fpcr: 0, _pad: [0u32; 2] }
    }
}

pub const FP_SIMD_CONTEXT_BYTES: usize = core::mem::size_of::<FpSimdContext>();
