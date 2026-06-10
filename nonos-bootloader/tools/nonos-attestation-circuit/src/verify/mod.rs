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

mod binding;
mod constants;
mod input;
mod parse_public_inputs;
mod read_capsule;
mod read_u32;
mod read_vk;
mod take;
mod validate_public_input_layout;
mod verify_capsule;
mod verify_groth16;

pub use binding::binding;
pub use constants::{CAPSULE_ZK_MAGIC, PUBLIC_INPUT_BYTES, PUBLIC_INPUT_LAYOUT};
pub use input::ProofInput;
pub use parse_public_inputs::parse_public_inputs;
pub use read_capsule::read_capsule;
pub use read_vk::read_vk;
pub use verify_capsule::verify_capsule;
pub use verify_groth16::verify_groth16;
