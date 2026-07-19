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

//! The one-call driver: source and inputs in, a proven-and-verified report out.
//!
//! This is the whole pipeline behind a single function, the surface a shell
//! command or a capsule calls. It compiles the source, runs the VM on the public
//! inputs, sizes the trace to the next power of two, lays it out for the step AIR,
//! binds the public inputs and outputs, proves it with the money-grade
//! Poseidon-committed STARK, and verifies the proof in process. Nothing here
//! panics: every failure along the way is a typed `RunError`.

use alloc::vec::Vec;

use nonos_stark::air::{stark_prove_poseidon_ext, stark_verify_poseidon_ext, Poseidon, RATE};
use nonos_stark::field::Fp;

use crate::air::{BuildError, StepAir, TRACE_WIDTH};
use crate::isa::Op;
use crate::lang::{compile_source, CompileError};
use crate::vm::{ProveError, Vm};

// The soundness parameters, matching the framework's own money-grade tests: 32
// queries, 16 grinding bits, and 3 extra blowup bits.
const QUERIES: usize = 32;
const GRIND: u32 = 16;
const BLOWUP: u32 = 3;

// The largest trace this driver will size to, 2^16 rows. A program needing more
// steps is rejected rather than silently proving a truncation.
const MAX_LOG_T: u32 = 16;

/// What a proving run produced.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    /// Whether the verifier accepted the proof. For an honest run this is true;
    /// it being false would signal a prover or AIR defect, not a bad program,
    /// since a bad program fails earlier with `RunError`.
    pub verified: bool,
    /// The number of instructions the VM executed, the trace rows before padding.
    pub steps: usize,
    /// The log2 of the padded trace length.
    pub log_trace_len: u32,
    /// The padded trace length, a power of two.
    pub trace_len: usize,
    /// The trace width the AIR proves over.
    pub trace_width: usize,
    /// The public outputs the program exposed, in declaration order.
    pub outputs: Vec<u64>,
}

/// Why a proving run did not complete.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunError {
    /// The source did not compile.
    Compile(CompileError),
    /// The program ran but its witness violated a constraint (a failed assert, an
    /// inverse of zero), so there is no trace to prove. This is the honest result
    /// for a program whose claim is false.
    Execute(ProveError),
    /// The executed trace could not be laid out for the AIR.
    Layout(BuildError),
    /// The program needs more steps than the driver will size a trace to.
    ProgramTooLong { steps: usize },
}

// The smallest `log_t` whose trace holds `n` rows, or `None` past the cap.
fn choose_log_t(n: usize) -> Option<u32> {
    let mut lg = 1u32;
    while (1usize << lg) < n {
        lg += 1;
        if lg > MAX_LOG_T {
            return None;
        }
    }
    Some(lg)
}

/// Run a VM program on `inputs` (all treated as public), prove it, and verify the
/// proof. Returns the report including the public outputs.
pub fn prove_program(program: &[Op], inputs: &[Fp]) -> Result<Report, RunError> {
    // The in-scope subset does not use the Poseidon port, so the injected hash is
    // a placeholder that is provably never called.
    let mut vm = Vm::new(|a, _b| a);
    let trace = vm.run(program, inputs, inputs.len()).map_err(RunError::Execute)?;
    let steps = trace.rows.len();
    let log_trace_len = choose_log_t(steps).ok_or(RunError::ProgramTooLong { steps })?;
    let air = StepAir::compile(program, log_trace_len, &trace.public_inputs, &trace.public_outputs)
        .map_err(RunError::Layout)?;
    let flat = air.build_trace(&trace).map_err(RunError::Layout)?;
    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let proof = stark_prove_poseidon_ext(&air, &flat, QUERIES, GRIND, BLOWUP, &hasher);
    let verified = stark_verify_poseidon_ext(&air, &proof, QUERIES, GRIND, BLOWUP, &hasher);
    Ok(Report {
        verified,
        steps,
        log_trace_len,
        trace_len: 1usize << log_trace_len,
        trace_width: TRACE_WIDTH,
        outputs: trace.public_outputs.iter().map(|f| f.value()).collect(),
    })
}

/// Compile zkolang source, then prove and verify it with the given public inputs.
pub fn prove_source_with_inputs(src: &str, inputs: &[u64]) -> Result<Report, RunError> {
    let program = compile_source(src).map_err(RunError::Compile)?;
    let fp_inputs: Vec<Fp> = inputs.iter().map(|&v| Fp::from_u64(v)).collect();
    prove_program(&program, &fp_inputs)
}

/// Compile zkolang source with no public inputs, then prove and verify it.
pub fn prove_source(src: &str) -> Result<Report, RunError> {
    prove_source_with_inputs(src, &[])
}
