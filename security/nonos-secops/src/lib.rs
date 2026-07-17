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

//! nonos-secops: the shared library behind the defense and offense tools. Every
//! check runs the bootloader's operation over the boot-side byte layout using the
//! shared nonos-stark verifier, so a verdict from these tools is the verdict at
//! boot.

pub mod attest;
pub mod offense;
pub mod rng;

pub use attest::{
    assemble_image, enroll_kernel, kernel_context, parse_image_footer, proof_parser_is_total,
    verify_kernel_attestation,
};
pub use offense::{battery, fuzz, Finding, Severity};
pub use rng::Rng;
