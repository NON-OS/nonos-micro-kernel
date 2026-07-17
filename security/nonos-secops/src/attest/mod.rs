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

//! The attestation operations both tools share, each the boot-side operation
//! over the boot-side byte layout using the shared nonos-stark verifier.

mod constants;
mod context;
mod enroll;
mod image;
mod parser;
mod verify;

pub use context::kernel_context;
pub use enroll::enroll_kernel;
pub use image::{assemble_image, parse_image_footer};
pub use parser::proof_parser_is_total;
pub use verify::verify_kernel_attestation;
