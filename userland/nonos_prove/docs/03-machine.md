<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# The machine

The compiler lowers a program to a flat list of instructions for a small register
machine. The machine is defined in `userland/nonos_prove/src/isa.rs` and executed
by `src/vm.rs`.

## The register file and the field

There are sixteen registers, `REGS = 16` in `isa.rs`. Each holds one field
element of type `Fp`, the Goldilocks field with modulus p = 2^64 - 2^32 + 1, taken
from the `nonos-stark` crate. This is the same scalar the kernel's transparent
STARK commits to. There is one definition of the field in the whole tree, and no
conversion happens between running a program and proving it: the register file
(`src/vm.rs`, `regs: [Fp; REGS]`) already holds exactly the values the prover
reads. A run and its proof are the same object seen two ways.

The machine also has a field-addressed memory in the executor, but the memory
opcodes are not yet constrained by the proof; see the note at the end of this
page.

## The instruction set

One flat opcode per step. The full set in `isa.rs` is:

| Opcode | Meaning | Proven today |
|---|---|---|
| `Imm { d, v }` | `r_d = v` (a literal) | yes |
| `Add { d, a, b }` | `r_d = r_a + r_b` | yes |
| `Sub { d, a, b }` | `r_d = r_a - r_b` | yes |
| `Mul { d, a, b }` | `r_d = r_a * r_b` | yes |
| `Inv { d, a }` | `r_d = r_a^{-1}`; zero is unprovable | yes |
| `Sel { d, c, a, b }` | `r_d = r_c ? r_a : r_b`, `r_c` a bit | yes |
| `Eq { d, a, b }` | `r_d = (r_a == r_b)` as a bit | yes |
| `Bool { a }` | constrain `r_a` to a bit | yes |
| `Assert { a }` | constrain `r_a` to zero | yes |
| `Inp { d, idx }` | `r_d = public_input[idx]` | yes |
| `Out { a, idx }` | `public_output[idx] = r_a` | yes |
| `Halt` | end of program | yes |
| `Load { d, a }` | `r_d = mem[r_a]` | not yet |
| `Store { a, b }` | `mem[r_a] = r_b` | not yet |
| `Pos { d, a, b }` | `r_d = poseidon2(r_a, r_b)` | not yet |

`Inv`, `Eq`, `Bool`, `Assert`, and `Sel` carry a witness or a constraint rather
than a plain result: an inverse is supplied and checked, an equality is decided by
an inverse-of-the-difference witness, a boolean or a zero is asserted. The
executor never panics on a violated constraint; it returns an unprovable result,
which is the honest outcome for a program whose claim is false (`src/vm.rs`,
`ProveError::Unprovable`).

## No dynamic control flow, on purpose

There is no jump, no branch, and no data-dependent control transfer of any kind.
A conditional is `Sel`, which computes both arms and selects one. A bounded loop is
unrolled by the front-end. The result is that the sequence of opcodes a program
executes, and therefore the shape of its trace, depends only on the program text,
never on the inputs.

This is a feature, not a gap. A fixed trace shape is what lets the proof be a
fixed-width algebraic object: the same small constraint system proves every step
of every program, and the cost of a proof is known before the program runs. A
machine with data-dependent branching would need its control flow itself proven,
which is a much larger and more error-prone constraint system. prøve trades
generality for a core you can read and check by hand.

## The deferred opcodes

`Load` and `Store` give the machine a random-access memory, and `Pos` gives it the
Poseidon hash. All three are present in the instruction set so the executor and
the language can grow into them, but the step AIR does not yet enforce them. A
program that reaches one of these opcodes is rejected when the trace is laid out
(`BuildError::NotInScope` in `src/air.rs`), not silently proven. Memory
consistency needs a permutation argument over sorted accesses, and the hash needs
its own constraint region; both are future work described in
[the AIR](05-the-air.md).
