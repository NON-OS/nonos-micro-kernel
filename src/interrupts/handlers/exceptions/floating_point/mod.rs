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

mod clear_simd_exception;
mod clear_x87_exception;
mod finish;
mod handle_simd;
mod handle_x87;
mod log_simd_exception;
mod log_x87_exception;
mod read_mxcsr;
mod read_x87_status;
mod status;

pub use handle_simd::handle_simd;
pub use handle_x87::handle_x87;
