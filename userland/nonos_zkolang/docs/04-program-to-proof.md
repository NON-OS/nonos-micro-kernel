<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# From program to proof

## The pipeline

```
  source text
     |  compile_source            (src/lang/)
     v
  Program: [Op]                   a flat instruction list ending in Halt
     |  Vm::run(program, inputs)  (src/vm.rs)
     v
  Trace                           one Row per step, plus public inputs/outputs
     |  StepAir::compile + build_trace   (src/air.rs)
     v
  trace matrix + public bindings
     |  stark_prove_poseidon_ext  (nonos-stark)
     v
  proof
     |  stark_verify_poseidon_ext (nonos-stark)
     v
  verified: true / false
```

Every arrow is a function in the crate. The whole path is wrapped in one call,
`prove_source_with_inputs` in `src/driver.rs`, which compiles, runs, sizes the
trace to the next power of two, lays it out, binds the public values, proves, and
verifies, returning a `Report`.

## The one command

From the terminal capsule:

```
prove             runs a built-in demo program
prove hello.zkl   reads a program from the VFS and proves it
```

The command compiles, runs, proves, and verifies inside the capsule and prints
the outcome, the trace shape, the public outputs, and the NOX fee
(`userland/capsule_terminal/src/command/builtin/prove.rs`). From Rust, the same
thing is one call:

```rust
let report = nonos_zkolang::prove_source_with_inputs(
    "input x; let y = x * x * x; output y;",
    &[3],
)?;
assert!(report.verified);
assert_eq!(report.outputs, vec![27]);
```

## What `verified: true` means

It means the verifier, given only the public program, the public inputs, the
claimed public outputs, and the proof, accepted. Concretely it means that a trace
exists in which every step obeyed its opcode's rules, every operand a step read
was the live value of the register it named, every register carried its value
forward until the step that wrote it, the clock advanced by one each step, and the
public inputs and outputs matched the committed values. The verifier reconstructs
the same constraint system from the public program and the public values, so a
prover cannot substitute a different program or different public data.

## What it does not mean

It does not mean the program is the one you intended; it means the program that
was proven is the one whose text the verifier holds. It is a proof about the code
as written, not a claim that the code is the right code for your purpose. Choosing
the right program is your job; proving it ran is zKølang's.

## When there is no proof

A program whose claim is false does not produce a false proof. It produces no
proof at all. If an assertion fails or the program inverts zero, the run returns
`RunError::Execute` and there is nothing to prove
(`src/driver.rs`, `src/vm.rs`). This is the honest failure mode: zKølang will not
hand you a receipt for a computation that did not hold.
