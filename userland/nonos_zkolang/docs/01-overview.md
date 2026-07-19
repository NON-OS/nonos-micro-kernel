<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Overview and philosophy

## The one sentence

zKølang is a language whose programs carry a proof of their own execution. You run
a program once, and you get back a receipt that a verifier checks in far less
work than running the program again, without trusting the party that produced it.

## Why this is not just a VM

A virtual machine runs a program and tells you the answer. You either trust the
machine that ran it or you run it yourself. zKølang runs a program and hands you a
short cryptographic object that stands in for having watched the whole run. The
guarantee is not "this machine says the answer is 27"; it is "here is evidence,
checkable by anyone, that a specific public program on specific public inputs
produced this output, and no step cheated."

That shift is the entire point. It turns computation into something you can
settle, delegate, and pay for without a trusted intermediary.

## Why not a general zkVM

There are larger provable virtual machines that run RISC-V or a general bytecode.
zKølang is deliberately smaller, for three reasons that come straight from living
inside an operating system.

The first is a shared field. Every zKølang value is a Goldilocks field element, the
`Fp` type from the `nonos-stark` crate (`userland/nonos_zkolang/src/vm.rs` holds the
register file as `[Fp; REGS]`). That is the same scalar the kernel's transparent
STARK already commits to. There is exactly one definition of the field in the
tree, and no translation between a run and its proof; a zKølang trace is already in
the form the prover consumes.

The second is a core small enough to audit by hand. zKølang has no data-dependent
jumps. Control is selection, not branching: a conditional is the `sel` operator,
which evaluates both arms and picks one, so the shape of the trace is a function
of the program text and never of the input. Bounded loops are unrolled by the
front-end before they reach the compiler, so a program's length is a static
property. Because the trace shape is fixed, the algebraic constraint system that
proves a step is small and readable, not a sprawling interpreter. You can read
every constraint in `userland/nonos_zkolang/src/air.rs` in one sitting.

The third is a settlement rail. Proving is metered in NOX, so it is a paid,
first-class operation rather than a free side effect (see
[proving economics](06-economics.md)). A small language with a small proof makes
the price predictable and the market legible.

## The shape of a claim

A zKølang proof attests a statement of the form: for this public program, on these
public inputs, the public outputs are these, and every intermediate step obeyed
the rules. Public inputs and outputs are bound into the proof by construction, so
the statement is about public data, not about some self-contained run whose
inputs a prover could have chosen after the fact. That binding is what makes a
proof worth paying for.

## Scope

zKølang is a complete straight-line verifiable-arithmetic language. Every opcode
is proven, register binding and public input and output binding are enforced, and
there are no placeholder instructions. What it does not have is a random-access
memory or a hash primitive; those are a deliberate future direction rather than a
missing piece of the current language, and the pages below never lean on them.

## Universal core, NØNOS flagship

The language core is a plain `no_std` crate that depends only on the prover
(`nonos-stark`) and makes no operating-system calls, so it compiles and runs on
any host that can run the prover. NØNOS is the flagship: it ships the `zkolang`
terminal command that compiles, runs, proves, and verifies in a capsule. Nothing
in the core is NØNOS-specific, so the same crate is portable to any OS, with NØNOS
as the first-class integration.
