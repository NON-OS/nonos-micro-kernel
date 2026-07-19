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

//! The source-level entry points: compile first, then prove. These are the
//! functions a caller usually reaches for. Public inputs are supplied in
//! declaration order; a private witness feeds the run without entering the
//! public statement.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::prove::run_and_prove;
use super::{Report, RunError};
use crate::lang::compile_source;

/// Compile zkolang source, then prove and verify it with the given public inputs.
pub fn prove_source_with_inputs(src: &str, inputs: &[u64]) -> Result<Report, RunError> {
    prove_source_with_witness(src, inputs, &[])
}

/// Compile zkolang source, then prove and verify it with public inputs and a
/// private witness. The `secret_inputs` feed the program's `secret` declarations;
/// they are used by the run but never enter the public statement, so a verifier
/// learns the outputs and the public inputs, not the witness.
pub fn prove_source_with_witness(
    src: &str,
    public_inputs: &[u64],
    secret_inputs: &[u64],
) -> Result<Report, RunError> {
    let program = compile_source(src).map_err(RunError::Compile)?;
    let mut inputs: Vec<Fp> = public_inputs.iter().map(|&v| Fp::from_u64(v)).collect();
    inputs.extend(secret_inputs.iter().map(|&v| Fp::from_u64(v)));
    run_and_prove(&program, &inputs, public_inputs.len())
}

/// Compile zkolang source with no public inputs, then prove and verify it.
pub fn prove_source(src: &str) -> Result<Report, RunError> {
    prove_source_with_inputs(src, &[])
}
