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

//! QuickJS-ng embedded as a no_std JavaScript engine. The C core is compiled
//! freestanding by build.rs; this crate supplies the C-ABI symbols it links
//! against from the Rust allocator and the libm crate.

#![no_std]

extern crate alloc;

mod alloc_stubs;
mod engine;
mod math_stubs;
mod misc_stubs;
mod str_stubs;

pub use engine::Engine;
