<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# The machine

The compiler lowers a program to a flat list of instructions for a small register
machine. The machine is defined in `userland/nonos_zkolang/src/isa.rs` and executed
by `src/vm.rs`.

## The register file and the field

There are sixteen registers, `REGS = 16` in `isa.rs`. Each holds one field
element of type `Fp`, the Goldilocks field with modulus p = 2^64 - 2^32 + 1, taken
from the `nonos-stark` crate. This is the same scalar the kernel's transparent
STARK commits to. There is one definition of the field in the whole tree, and no
conversion happens between running a program and proving it: the register file
(`src/vm.rs`, `regs: [Fp; REGS]`) already holds exactly the values the prover
reads. A run and its proof are the same object seen two ways.

## The instruction set

One flat opcode per step. Every opcode is enforced by the step AIR; there are no
unproven instructions. The full set in `isa.rs` is:

| Opcode | Meaning |
|---|---|
| `Imm { d, v }` | `r_d = v` (a literal) |
| `Add { d, a, b }` | `r_d = r_a + r_b` |
| `Sub { d, a, b }` | `r_d = r_a - r_b` |
| `Mul { d, a, b }` | `r_d = r_a * r_b` |
| `Inv { d, a }` | `r_d = r_a^{-1}`; zero is unprovable |
| `Sel { d, c, a, b }` | `r_d = r_c ? r_a : r_b`, `r_c` a bit |
| `Eq { d, a, b }` | `r_d = (r_a == r_b)` as a bit |
| `Bool { a }` | constrain `r_a` to a bit |
| `Assert { a }` | constrain `r_a` to zero |
| `Inp { d, idx }` | `r_d = public_input[idx]` |
| `Out { a, idx }` | `public_output[idx] = r_a` |
| `Halt` | end of program |

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
which is a much larger and more error-prone constraint system. zKølang trades
generality for a core you can read and check by hand.

## Scope, and a note on growth

This is the complete instruction set. Every opcode above is proven, so a program
built from them is proven whole; there are no placeholder instructions and nothing
is rejected at proving time for being unimplemented. What the machine does not have
is a random-access memory or a hash primitive. Those are not missing pieces of the
current language; they are a deliberate future direction. Adding a memory would
turn the circuit into a general machine and would need a permutation argument over
sorted accesses, a distinct and larger construction that belongs to a future major
version, not to the branchless arithmetic core documented here.
