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

//! The prove pipeline over an already-compiled program: run the VM, size the
//! trace, lay it out, bind the public statement, prove, and verify. This is where
//! the program commitment, the trace length, and the public inputs and outputs are
//! seeded into the transcript, so a proof is tied to exactly one statement.

use alloc::vec::Vec;

use nonos_stark::air::{
    stark_prove_poseidon_ext_pub, stark_verify_poseidon_ext_pub, Poseidon, RATE,
};
use nonos_stark::field::Fp;

use super::{Report, RunError};
use crate::air::{StepAir, TRACE_WIDTH};
use crate::commit;
use crate::isa::Op;
use crate::vm::Vm;

// The soundness parameters, matching the framework's own money-grade tests: 32
// queries, 16 grinding bits, and 3 extra blowup bits.
const QUERIES: usize = 32;
const GRIND: u32 = 16;
const BLOWUP: u32 = 3;

// The largest trace this driver will size to, 2^16 rows. A program needing more
// steps is rejected rather than silently proving a truncation.
const MAX_LOG_T: u32 = 16;

/// Run a VM program on `inputs` (all treated as public), prove it, and verify the
/// proof. Returns the report including the public outputs.
pub fn prove_program(program: &[Op], inputs: &[Fp]) -> Result<Report, RunError> {
    run_and_prove(program, inputs, inputs.len())
}

/// The shared pipeline. `inputs` is the public prefix (`n_public` values) followed
/// by the private witness; only the public prefix enters the bound statement.
pub(super) fn run_and_prove(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
) -> Result<Report, RunError> {
    let mut vm = Vm::new();
    let trace = vm.run(program, inputs, n_public).map_err(RunError::Execute)?;
    let steps = trace.rows.len();
    let log_trace_len = choose_log_t(steps).ok_or(RunError::ProgramTooLong { steps })?;
    let air = StepAir::compile(program, log_trace_len, &trace.public_inputs, &trace.public_outputs)
        .map_err(RunError::Layout)?;
    let flat = air.build_trace(&trace).map_err(RunError::Layout)?;

    // The public statement bound into the proof by Fiat-Shamir: the program
    // commitment, the trace length (so the fee is checkable), then the public
    // inputs and outputs. The verifier replays exactly this, so a proof is tied
    // to one program, one trace size, and one public input and output.
    let trace_len = 1usize << log_trace_len;
    let mut publics: Vec<Fp> = Vec::new();
    publics.extend_from_slice(&commit::commit_limbs(program));
    publics.push(Fp::from_u64(trace_len as u64));
    publics.extend_from_slice(&trace.public_inputs);
    publics.extend_from_slice(&trace.public_outputs);

    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let proof =
        stark_prove_poseidon_ext_pub(&air, &flat, QUERIES, GRIND, BLOWUP, &hasher, &publics);
    let verified =
        stark_verify_poseidon_ext_pub(&air, &proof, QUERIES, GRIND, BLOWUP, &hasher, &publics);
    Ok(Report {
        verified,
        steps,
        log_trace_len,
        trace_len,
        trace_width: TRACE_WIDTH,
        outputs: trace.public_outputs.iter().map(|f| f.value()).collect(),
        program_commit: commit::commit(program),
    })
}

// The smallest `log_t` whose trace holds `n` rows, or `None` past the cap.
pub(crate) fn choose_log_t(n: usize) -> Option<u32> {
    let mut lg = 1u32;
    while (1usize << lg) < n {
        lg += 1;
        if lg > MAX_LOG_T {
            return None;
        }
    }
    Some(lg)
}
