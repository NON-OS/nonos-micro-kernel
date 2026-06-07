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

// Per-capsule zero-knowledge attestation. Before a capsule is mapped or
// scheduled, the kernel verifies its embedded Groth16 proof and binds it to the
// capsule's real bytes and the capabilities about to be granted. A capsule that
// does not carry a valid, correctly bound proof is refused. This mirrors the
// bootloader's kernel attestation, applied per capsule on the same BLS12-381
// curve.

mod error;
mod layout;
mod registry;
mod trailer;
mod verify;

pub use error::AttestError;
pub use registry::trailer_for;
pub use verify::verify_capsule_attestation;
