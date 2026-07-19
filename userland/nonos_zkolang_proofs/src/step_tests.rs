// NONOS Operating System (AGPL-3.0-or-later)
//! End-to-end proofs of the zkolang step AIR against the money-grade
//! Poseidon-committed STARK. A real VM program exercising the whole branchless
//! computational core is run, its trace laid out for the AIR, and the prover and
//! verifier driven in process. The honest trace must verify; each targeted tamper
//! must be rejected. These run on the host under `cargo test`, so a pass means the
//! proof system actually accepts and rejects, not merely that the AIR type checks.

use nonos_stark::air::{stark_prove_poseidon_ext, stark_verify_poseidon_ext, Poseidon, RATE};
use nonos_stark::field::Fp;
use nonos_zkolang::{Op, StepAir, Vm, TRACE_WIDTH};

// Column offsets, mirroring the AIR layout. The AIR owns the canonical copy;
// these let a test reach into the flat trace to corrupt one cell.
const CLK: usize = 0;
const S_MUL: usize = 4;
const A: usize = 13;
const C: usize = 15;
const D: usize = 16;
const AUX: usize = 18;
const RF_BASE: usize = 19;

// The proof parameters the framework's own money-grade tests use: 32 queries,
// 16 grinding bits, 3 extra blowup bits.
const QUERIES: usize = 32;
const GRIND: u32 = 16;
const BLOWUP: u32 = 3;

// log2 of the padded trace length. The sample run is thirteen rows, padded to
// sixteen.
const LOG_T: u32 = 4;

fn cell(row: usize, col: usize) -> usize {
    row * TRACE_WIDTH + col
}

fn hasher() -> Poseidon {
    Poseidon::new(2, [Fp::ZERO; RATE])
}

// Run a program that touches every opcode in scope and lay it out for the AIR:
//
//   r0 = 0; r1 = 5; r2 = 5;
//   r3 = r1^{-1};            invert
//   r4 = (r1 == r2) (= 1);   equal
//   r5 = (r1 == r0) (= 0);   unequal
//   r6 = r4 ? r1 : r0 (= 5); select
//   bool r4;                 r4 is boolean
//   assert r0;               r0 is zero
//   r7 = r1 + r2 (= 10);
//   r8 = r1 * r2 (= 25);
//   r9 = r2 - r1 (= 0);
//   halt
//
// Row indices below name each instruction for the tamper tests.
const R_INV: usize = 3;
const R_EQ_TRUE: usize = 4;
const R_EQ_FALSE: usize = 5;
const R_SEL: usize = 6;
const R_BOOL: usize = 7;
const R_ASSERT: usize = 8;
const R_ADD: usize = 9;
const R_MUL: usize = 10;

fn honest() -> (StepAir, Vec<Fp>) {
    let program = [
        Op::Imm { d: 0, v: Fp::from_u64(0) },
        Op::Imm { d: 1, v: Fp::from_u64(5) },
        Op::Imm { d: 2, v: Fp::from_u64(5) },
        Op::Inv { d: 3, a: 1 },
        Op::Eq { d: 4, a: 1, b: 2 },
        Op::Eq { d: 5, a: 1, b: 0 },
        Op::Sel { d: 6, c: 4, a: 1, b: 0 },
        Op::Bool { a: 4 },
        Op::Assert { a: 0 },
        Op::Add { d: 7, a: 1, b: 2 },
        Op::Mul { d: 8, a: 1, b: 2 },
        Op::Sub { d: 9, a: 2, b: 1 },
        Op::Halt,
    ];
    let mut vm = Vm::new();
    let trace = vm.run(&program, &[], 0).expect("honest run produced no trace");
    let air = StepAir::compile(&program, LOG_T, &[], &[]).expect("program did not compile");
    let flat = air.build_trace(&trace).expect("honest trace did not lay out");
    (air, flat)
}

fn accepts(air: &StepAir, flat: &[Fp]) -> bool {
    let h = hasher();
    let proof = stark_prove_poseidon_ext(air, flat, QUERIES, GRIND, BLOWUP, &h);
    stark_verify_poseidon_ext(air, &proof, QUERIES, GRIND, BLOWUP, &h)
}

#[test]
fn honest_step_run_verifies() {
    let (air, flat) = honest();
    assert!(accepts(&air, &flat), "an honest step run was rejected");
}

#[test]
fn tampered_add_result_is_rejected() {
    let (air, mut flat) = honest();
    flat[cell(R_ADD, D)] = flat[cell(R_ADD, D)] + Fp::ONE;
    assert!(!accepts(&air, &flat), "a wrong add result verified");
}

#[test]
fn tampered_multiply_result_is_rejected() {
    let (air, mut flat) = honest();
    flat[cell(R_MUL, D)] = flat[cell(R_MUL, D)] + Fp::ONE;
    assert!(!accepts(&air, &flat), "a wrong multiply result verified");
}

#[test]
fn tampered_inverse_witness_is_rejected() {
    let (air, mut flat) = honest();
    // The inverse witness no longer multiplies with the operand to one.
    flat[cell(R_INV, AUX)] = flat[cell(R_INV, AUX)] + Fp::ONE;
    assert!(!accepts(&air, &flat), "a wrong inverse verified");
}

#[test]
fn tampered_equal_bit_is_rejected() {
    let (air, mut flat) = honest();
    // Force the equal comparison to claim inequality.
    flat[cell(R_EQ_TRUE, D)] = Fp::ZERO;
    assert!(!accepts(&air, &flat), "a false equality bit verified");
}

#[test]
fn tampered_unequal_bit_is_rejected() {
    let (air, mut flat) = honest();
    // Force the unequal comparison to claim equality.
    flat[cell(R_EQ_FALSE, D)] = Fp::ONE;
    assert!(!accepts(&air, &flat), "a false inequality bit verified");
}

#[test]
fn tampered_select_result_is_rejected() {
    let (air, mut flat) = honest();
    flat[cell(R_SEL, D)] = flat[cell(R_SEL, D)] + Fp::ONE;
    assert!(!accepts(&air, &flat), "a wrong select result verified");
}

#[test]
fn nonboolean_select_condition_is_rejected() {
    let (air, mut flat) = honest();
    // A condition outside {0,1} must trip the select's boolean gate.
    flat[cell(R_SEL, C)] = Fp::from_u64(2);
    assert!(!accepts(&air, &flat), "a non-boolean select condition verified");
}

#[test]
fn nonboolean_bool_operand_is_rejected() {
    let (air, mut flat) = honest();
    flat[cell(R_BOOL, A)] = Fp::from_u64(2);
    assert!(!accepts(&air, &flat), "a non-boolean bool operand verified");
}

#[test]
fn failed_assertion_is_rejected() {
    let (air, mut flat) = honest();
    // The assertion demands zero; a nonzero operand must fail.
    flat[cell(R_ASSERT, A)] = Fp::ONE;
    assert!(!accepts(&air, &flat), "a failed assertion verified");
}

#[test]
fn data_in_a_padding_row_is_rejected() {
    let (air, mut flat) = honest();
    // Row 13 is a padding halt row. Smuggling a value into an operand column
    // must trip the halt-cleanliness constraint.
    flat[cell(13, A)] = Fp::from_u64(42);
    assert!(!accepts(&air, &flat), "a padding row carrying data verified");
}

#[test]
fn broken_clock_order_is_rejected() {
    let (air, mut flat) = honest();
    flat[cell(R_EQ_FALSE, CLK)] = flat[cell(R_EQ_FALSE, CLK)] + Fp::ONE;
    assert!(!accepts(&air, &flat), "an out-of-order clock verified");
}

#[test]
fn two_opcodes_in_one_row_is_rejected() {
    let (air, mut flat) = honest();
    // The add row also asserts the multiply selector, breaking one-hot.
    flat[cell(R_ADD, S_MUL)] = Fp::ONE;
    assert!(!accepts(&air, &flat), "a row naming two opcodes verified");
}

#[test]
fn corrupt_final_boundary_is_rejected() {
    let (air, mut flat) = honest();
    // The last row is boundary-pinned to a clean halt. Dirtying it must fail.
    let last = (1usize << LOG_T) - 1;
    flat[cell(last, A)] = Fp::from_u64(7);
    assert!(!accepts(&air, &flat), "a dirty final row verified");
}

// Register binding: the properties this phase newly proves. Each of these leaves
// the per-row opcode arithmetic internally consistent, so only the binding of
// operands and results to the register file can catch them.

#[test]
fn operand_not_from_register_is_rejected() {
    let (air, mut flat) = honest();
    // Row 9 adds r1 (= 5) and r2 (= 5) into 10. Rewrite operand a to 6 and the
    // result to 11 so the add gate still holds: d = a + b = 6 + 5. Only the read
    // binding, a must equal the live r1, can reject this.
    flat[cell(R_ADD, A)] = Fp::from_u64(6);
    flat[cell(R_ADD, D)] = Fp::from_u64(11);
    assert!(!accepts(&air, &flat), "an operand not drawn from its register verified");
}

#[test]
fn forged_initial_register_is_rejected() {
    let (air, mut flat) = honest();
    // Registers must start at zero; seed r1 with a value at row 0.
    flat[cell(0, RF_BASE + 1)] = Fp::from_u64(5);
    assert!(!accepts(&air, &flat), "a nonzero initial register verified");
}

#[test]
fn broken_write_propagation_is_rejected() {
    let (air, mut flat) = honest();
    // Row 1 writes r1 = 5, so from row 2 on the register file must carry 5 in
    // slot r1. Corrupt that carried value.
    flat[cell(2, RF_BASE + 1)] = Fp::from_u64(9);
    assert!(!accepts(&air, &flat), "a register that lost its written value verified");
}
