<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# zKølang

zKølang is a small language whose every run comes with a proof that it ran
correctly. You write straight-line field arithmetic and a few assertions, the
system runs the program on a tiny register machine, and it returns a STARK that
anyone can check far faster than re-running the program. The proof speaks the
same cryptographic language as the rest of NØNOS, because zKølang computes over the
same field the kernel's transparent STARK already commits to.

The name is written zKølang; the ø is the NØNOS mark. In the source and in code
identifiers it is spelled `zkolang`, which keeps the crate ASCII.

## Read in this order

1. [Overview and philosophy](01-overview.md). What zKølang is, and why a shared
   field and a deliberately small core matter.
2. [The language](02-language.md). Grammar, every construct, and the opcode each
   one lowers to.
3. [The machine](03-machine.md). Sixteen registers, the field, the instruction
   set, and what has no representation on purpose.
4. [From program to proof](04-program-to-proof.md). The whole pipeline, and what
   a verified proof does and does not mean.
5. [The AIR](05-the-air.md). The column layout and the constraints, for a reader
   who wants to check the trace is a fixed-width algebraic object.
6. [Proving economics](06-economics.md). The NOX pay-to-prove fee, and what part
   of it is code today.
7. [Reference](07-reference.md). The opcode table, the grammar, the error types,
   the limits, and an FAQ.

## Where the code is

The crate is `userland/nonos_zkolang/`. The host proof suite that this
documentation's claims are checked against is `userland/nonos_zkolang_proofs/`; run
`cargo test` there to reproduce the accept-and-reject evidence, and
`cargo run --release --example measure` to reproduce the trace shapes and fees
quoted in these pages.

## What is real

The whole language is built and proven. Every opcode is enforced by the AIR:
immediate loads, field add, subtract, multiply, inverse, an equality that yields a
clean bit, a branchless select, a boolean check, a zero assertion, and public
inputs and outputs. Register binding, the guarantee that an operand is the live
value of the register it names, is enforced. Public inputs and outputs are bound
to committed values. There are no placeholder instructions and nothing is rejected
at proving time for being unimplemented.

What the language does not have is a random-access memory or a hash primitive.
That is a deliberate scope, not an unfinished part: zKølang is a straight-line
verifiable-arithmetic language, and a memory extension would be a future major
version. The core is a `no_std` crate that depends only on the prover and makes no
operating-system calls, so it runs on any host; NØNOS is the flagship, shipping
the `zkolang` terminal command.
