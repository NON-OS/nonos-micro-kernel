---
title: "zKølang: A Verifiable-Compute Language Proven by a Transparent STARK"
subtitle: "A Field-Shared Register Machine, an Auditable Step AIR with Register Binding as a Public Circuit, and a NOX Pay-to-Prove Rail"
author: "NØNOS Contributors"
date: "2026"
abstract: |
  We present zKølang, a small language for verifiable computation built into NØNOS.
  A zKølang program compiles to a fixed-width register machine over the Goldilocks
  field, and a single algebraic intermediate representation (AIR) constrains one
  machine step, so any compiled program is proven by the same transparent STARK
  the kernel already runs. Three choices distinguish zKølang from a general zero
  knowledge virtual machine. First, it shares one field with the kernel prover, so
  a run and its proof are one object with no translation between them. Second, its
  core is deliberately branchless: control is selection, not data-dependent
  jumping, so a program's trace shape is a static property and the AIR that proves
  a step stays small enough to read. Third, because registers are named by
  compile-time index, the data flow of a program is a public circuit, which lets
  register binding, the guarantee that an operand is the live value of the register
  it names, be enforced by linear constraints over public one-hot columns rather
  than by a grand-product permutation. We give the language and its semantics, the
  execution model, the step AIR and an argument that a satisfying assignment
  implies a faithful run, the binding of the trace to the money-grade STARK, and a
  NOX pay-to-prove fee that follows the proving work. We report measured trace
  shapes and an accept-and-reject evidence suite of fifty-eight in-process proofs. We
  are explicit about scope: every opcode is enforced, so a program is proven
  whole with no unimplemented instruction; what the language does not have is a
  random-access memory or a hash primitive, a deliberate scope rather than a
  missing piece, and we never lean on either. The core is a portable no_std crate
  that depends only on the prover and makes no operating-system calls, so it runs
  on any host, with NØNOS as the flagship integration.
---

# Introduction

Sometimes you want to run a computation and hand someone a receipt they can check
faster than re-running it, and you want that receipt to speak the same
cryptographic language as the rest of your system. That is the problem zKølang
solves inside NØNOS. A zKølang program is ordinary straight-line field arithmetic
with a few assertions and a public input and output interface. Running it produces
a transparent STARK: a succinct object that attests, to anyone, that a specific
public program on specific public inputs produced specific public outputs, with
every intermediate step obeying the rules of the machine.

The design is not novel in its cryptography. It reuses the NØNOS transparent
STARK without change, and the reader should treat that prover as a component
documented elsewhere [@nonosverification]. What is worth writing down is the shape
of the language and the intermediate representation that sits between a program
and that prover, and in particular one construction: because zKølang names its
registers statically, the wiring of a program is public, and register binding
becomes a set of linear constraints rather than a permutation argument. That is
the technical heart of this paper, and it is the reason the AIR is small enough to
audit by hand.

We are deliberate about the boundary between what is built and what is not. Every
opcode is proven, so a program is proven whole. What the language deliberately
lacks is a random-access memory and a hash primitive; those are a future direction,
not an unfinished part of what is here, and every claim below respects that line.
The core is a portable no_std crate, and NØNOS is its flagship host.

# Motivation, the NØNOS way

There are capable general zero knowledge virtual machines that prove the execution
of RISC-V or a general bytecode [@risczero; @sp1]. Building a smaller, purpose-made
language instead of adopting one of these is a trade, and it is worth saying what
we bought and what we gave up.

We bought three things. The first is a shared field. Every zKølang value is a
Goldilocks field element, the `Fp` type of the `nonos-stark` crate, which is the
same scalar the kernel STARK commits to. The register file in the executor is
literally `[Fp; REGS]`. There is one definition of the field in the whole source
tree, and a zKølang trace is already in the representation the prover consumes; no
serialization, no re-encoding, no second field to keep in sync. The second is an
auditable core. zKølang has no data-dependent control flow, so the sequence of
opcodes a program runs depends only on its text, and the trace is a fixed-width
algebraic matrix. That makes the constraint system that proves a step small: a
reviewer can read all of it. A general machine must prove its own control flow,
which is a far larger and more error-prone constraint system. The third is a
settlement rail. Proving is metered in NOX, so it is a paid, first-class
operation, and a small language with a predictable proof size makes the price
legible.

We gave up generality. zKølang does not run arbitrary programs; it runs the programs
its front-end can express. That is the point. The conventional parts, a Goldilocks
field, a Poseidon-committed FRI STARK, an AIR over a two-row window, are
conventional on purpose, so the one unconventional choice, static wiring, stands
alone and can be checked.

# The language and its semantics

A zKølang program is a sequence of statements over field values. The concrete syntax
is small enough to give in full. The lexer recognizes exactly the keywords `let`,
`assert`, `input`, `secret`, `output`, `inv`, `sel`, `for`, `in`, `if`, `else`, the operators
`+ - * / == !=` and unary `-`, the punctuation `( ) , ; { } ..`, identifiers, and
decimal numerals (`userland/nonos_zkolang/src/lang/lex.rs`). The grammar, lowest
precedence first, is:

```
program  := stmt*
stmt     := 'let' ident '=' expr ';' | 'assert' expr ';'
          | 'input' ident ';' | 'secret' ident ';' | 'output' expr ';'
          | 'for' ident 'in' number '..' number '{' stmt* '}'
expr     := equality
equality := sum (('==' | '!=') sum)?
sum      := product (('+' | '-') product)*
product  := unary (('*' | '/') unary)*
unary    := '-' unary | primary
primary  := number | ident | '(' expr ')'
          | 'inv' '(' expr ')' | 'sel' '(' expr ',' expr ',' expr ')'
          | 'if' expr '{' expr '}' 'else' '{' expr '}'
```

The semantics are straight-line and single-assignment at the source level. A `let`
evaluates its expression and binds a name to the result; a name resolves to the
most recent binding, giving lexical shadowing. The compiler reuses physical
registers once a temporary is dead, so register indices stay compile-time
constants (which is all the register binding needs) while larger programs fit the
sixteen-register file. Arithmetic is field add, subtract, and
multiply. `inv(e)` is the field inverse, defined for nonzero arguments; inverting
zero yields no valid run. `a == b` is a total equality that produces a bit.
`sel(c, a, b)`, and its familiar form `if c { a } else { b }`, is a branchless
conditional over a boolean `c`, evaluating both arms and selecting one. `assert e`
states that `e` is zero; the forms `assert a == b` and `assert a != b` read the
same intent, the first asserting the difference is zero, the second inverting it so
the assertion fails only when they are equal. `input x` binds `x` to the next public input, `secret w` binds `w` to a private
witness that never enters the public statement, and `output e` exposes `e` as the
next public output.

The well-formedness condition that earns the fixed-width trace is simply this:
there is no construct whose lowering depends on a value. Every statement compiles
to a fixed sequence of opcodes determined by its syntax alone (`src/lang/compile.rs`),
and bounded loops, when present, are unrolled by the front-end before compilation.
Consequently the compiled program, a flat list of opcodes ending in `Halt`, has a
length that is a static function of the source, and so does the number of trace
rows. We state the correspondence precisely: for a program that the front-end
accepts, the sequence of opcodes the executor runs is exactly the sequence the
compiler emitted, in order, until `Halt`; the executor performs no branching of
its own (`src/vm.rs`). This is the property the AIR relies on when it treats the
trace as a matrix of known width and a known, padded height.

# The execution model and the trace

The machine has sixteen registers (`REGS = 16` in `src/isa.rs`), each holding one
`Fp`. The executor runs the opcode list, filling one trace row per step
(`src/vm.rs`, `src/trace.rs`). A row records, for the step it represents: the
opcode, the operand values read, the result written, an immediate, and an
auxiliary witness used by the inverse and equality opcodes. The executor never
panics. A violated constraint, a failed assertion, an inverse of zero, an operand
index out of range, is returned as a typed error, and the honest consequence for a
false claim is that there is no trace to prove (`ProveError::Unprovable`).

The instruction set has twelve opcodes, and every one is enforced by the step AIR:
`Imm`, `Add`, `Sub`, `Mul`, `Inv`, `Eq`, `Sel`, `Bool`, `Assert`, `Inp`, `Out`,
and `Halt`. There is no unproven instruction; a program built from these is proven
whole. Each column of a trace row carries a claim, and the AIR is the statement
that those claims hold and cohere.

# The step AIR

The step AIR (`src/air.rs`) implements the `Air` and `AirExt` traits of
`nonos-stark`, so its transition is written once over a field abstraction and
evaluated over both the base field and its quadratic extension. The trace is a
matrix of `TRACE_WIDTH = 35` columns: a clock, twelve one-hot opcode selectors,
three operand columns, a result, an immediate, an auxiliary witness, and sixteen
register-file columns holding the register state before each row. The transition
reads a two-row window. There are forty-six transition constraints, each of degree
at most three.

The constraints group into five ideas, and we state each with the guarantee it
buys.

Selector well-formedness. Each selector is boolean and the twelve sum to one, so
every row commits to exactly one opcode. A satisfying assignment cannot leave a row
opcode-free or claim two opcodes at once.

Opcode semantics. Gated by the selector, the result column equals the operation
the opcode names. Addition is `s_add * (d - (a + b)) = 0`, and multiply, subtract,
and immediate load have the same shape. The inverse is witnessed:
`s_inv * (a * aux - 1) = 0` forces `aux = a^{-1}`, which also forces `a` nonzero,
and `s_inv * (d - aux) = 0` sets the result. Equality decides a bit from the
difference and its inverse witness. Select verifies a boolean condition and
computes `c*a + b - c*b`. The two constraint opcodes assert a bit or a zero. A
satisfying assignment therefore implies that each row's result is the correct
field operation on that row's operand columns.

Ordering. The clock rises by one, so the rows are a genuine ordered sequence and
padding cannot be interleaved with computation.

Register binding. This is the constraint that upgrades the proof from "a bag of
individually valid rows exists" to "this program ran." We give it its own section.

Public input and output binding. Handled by boundary constraints, covered below.

## Register binding as a public circuit

A general register machine faces a problem: an operand column claims to hold the
current value of some register, but nothing local forces it to. The standard
solution is a permutation argument, a grand product that proves the multiset of
reads matches the multiset of writes, which is necessary when the register or
memory address is computed at runtime.

zKølang does not need it, because its register indices are compile-time constants.
`Add { d, a, b }` names registers `a` and `b` in the program text, and the verifier
knows them. So the map from a row's result to the rows that later read it is a
public property of the program, not of the witness. The AIR carries this map as
periodic columns: for each row, one one-hot vector for the register it writes and
three one-hot vectors for the registers its read ports name, sixty-four periodic
columns in total (`NUM_PERIODIC = 4 * REGS`), regenerated by the verifier from the
public program (`periodic_columns` in `src/air.rs`). The register file is threaded
through the trace. A read is the linear form

$$ \text{operand} \;=\; \sum_{k=0}^{15} \text{read\_onehot}_k \cdot \text{regfile}_k $$

and a write updates the file in place,

$$ \text{regfile\_next}_k \;=\; (1 - \text{write\_onehot}_k)\cdot \text{regfile}_k \;+\; \text{write\_onehot}_k \cdot \text{result}. $$

Both are linear in the trace, because the one-hot coefficients are public
constants supplied by the periodic columns, not witness values. Register binding
therefore adds no constraint degree and, crucially, no soundness assumption: a
prover cannot lie about which register an operand came from, because the wiring is
recomputed by the verifier and not read from the proof. We claim, and the tamper
suite below demonstrates, that an operand which keeps its row's arithmetic
internally consistent but is not the live value of its named register is rejected.
This construction is available precisely because the wiring is static; a dynamic
memory would reintroduce the permutation argument, which a future memory
extension would add.

## Public input and output binding

The boundary constraints pin specific cells (`boundary` in `src/air.rs`): the clock
starts at zero, the final row is a clean halt, and every register starts at zero.
On top of that fixed set, the AIR adds one binding per public input and output. For
each `Inp` row, the immediate column is pinned to the committed public input; for
each `Out` row, the operand column is pinned to the committed public output. The
verifier reconstructs the same AIR, including these bindings, from the public
program and the public values it was given, so they are part of the checked
statement. A prover who ran a different input, or claimed a different output,
produces a trace whose pinned cells disagree with the reconstructed AIR, and the
proof fails.

## Soundness of the AIR, and its honest boundary

Taken together, a satisfying assignment to the step AIR implies a faithful run of
the compiled program on the committed public inputs, producing the committed public
outputs: each row is a well-formed single opcode, each result is the correct
operation on its operands, each operand is the live register value by the binding
argument, the rows are ordered, and the public interface matches. This is what the
STARK, over the AIR, attests.

Every opcode the machine has is enforced, so the statement covers whole programs
with no unproven instruction. What the machine deliberately lacks is a
random-access memory and a hash primitive. Adding a memory would be a future major
version whose consistency is a permutation argument over sorted accesses, exactly
the tool that static register binding avoids today.

# Binding to the kernel STARK

The AIR is handed as a standard instance to the money-grade Poseidon-committed
prover of `nonos-stark` (`stark_prove_poseidon_ext`), which produces a transparent
STARK with no trusted setup, sampling its out-of-domain point in the quadratic
extension of the field and testing low degree with FRI over a DEEP composition. We
do not re-derive that prover here; it is the subject of the NØNOS verification
paper [@nonosverification], and zKølang treats it as a component. The seam is the
whole point: zKølang emits an ordinary AIR over the shared field, and the STARK does
the rest. The soundness of a zKølang proof is the soundness of that STARK applied to
the AIR whose satisfiability we argued above. The driver proves at thirty-two
queries, sixteen grinding bits, and three extra blowup bits (`src/driver.rs`), the
setting the `nonos-stark` money-grade tests use.

# Proving economics

Proving is work, and zKølang prices it. The cost driver is the trace area, rows times
width, because the prover's field arithmetic, its commitments, and its low-degree
test all scale with it, and padding to price cannot help an attacker because it
only enlarges the bill. The quote (`src/nox.rs`) is a flat floor plus a rate on the
area, split into a prover payment and a protocol cut that accrues to the NOX
treasury:

```
  cells    = trace_len * trace_width
  compute  = cells * 50 microNOX / 1000
  total    = 1000 microNOX + compute
  protocol = total * 5%
  prover   = total - protocol
```

The floor keeps submission from being free, so the market is not a spam vector. The
rates are governance-tunable constants; the shape is the durable part. The intended
market is direct: a buyer with a computation it will not run or re-run submits the
program and public inputs with a fee, a prover returns the STARK, and on
verification the fee releases, most to the prover and a cut to the protocol. NOX is
the settlement token, so proving traffic is demand for NOX and the cut is protocol
revenue. Because the proof binds its public inputs and outputs, the buyer pays for
a statement about public data. The quote is code and is tested; the on-chain
escrow and release are the job of the NOX rail and its contracts, outside this
crate, and we do not claim that settlement is wired here.

# Evaluation

All numbers below are produced by running the crate:
`cargo run --release --example measure` in `userland/nonos_zkolang_proofs`.

| program | steps | trace | cells | fee (microNOX) | prover | protocol |
|---|---|---|---|---|---|---|
| demo (add, mul, assert, output) | 9 | 2^4 x 35 | 560 | 1028 | 977 | 51 |
| square, y = x^2 | 4 | 2^2 x 35 | 140 | 1007 | 957 | 50 |
| cube, y = x^3 | 5 | 2^3 x 35 | 280 | 1014 | 964 | 50 |
| degree eight, y = x^8 | 6 | 2^3 x 35 | 280 | 1014 | 964 | 50 |

The trace width is constant at thirty-five columns; the height is the next power of
two above the step count. Each of these programs proves and verifies in process on
the host. The fee follows the trace area and always leaves the prover the majority.

The correctness evidence is a suite of fifty-eight in-process proofs in
`userland/nonos_zkolang_proofs` (`cargo test`). An honest run of a program touching
every enforced opcode verifies. Each of a targeted tamper set is rejected: a wrong
add, subtract, multiply, or select result; a forged inverse witness; a false
equality bit in each direction; a non-boolean select condition; a non-boolean
boolean check; a failed assertion; an operand that is not drawn from its register,
with the arithmetic gate kept internally consistent so that only the register
binding can catch it; a forged initial register; a broken write propagation; an
out-of-order clock; two opcodes named in one row; a dirtied final boundary; a
forged public input; and a forged public output. The statement binding is
exercised too: a proof bound to one program commitment or trace length is rejected
under a forged one, since the public statement seeds the Fiat-Shamir transcript.
The private witness is covered: a program proves knowledge of a secret square root
without the witness entering the public statement, and a wrong witness yields no
proof. The language is covered end to end: a true program verifies, a false claim
yields no proof, and a malformed program is a typed compile error.

# Threat model and honest scope

An adversary is a prover trying to make the verifier accept a false statement. The
verifier holds the public program, the public inputs, the claimed public outputs,
and the proof, and reconstructs the AIR from the first three. What the adversary
cannot forge, given the soundness of the underlying STARK: a result that
disobeys its opcode, since the arithmetic constraints bind it; an operand that is
not the live register value, since the register wiring is public and recomputed by
the verifier; a false equality or a satisfied assertion of a nonzero, since these
are witnessed and checked; a public input or output different from the committed
value, since the boundary pins them; and out-of-order or opcode-ambiguous rows,
since the selectors and the clock forbid them.

What is assumed: the soundness of the `nonos-stark` prover, which this paper does
not re-establish [@nonosverification]; and the correctness of the compiler and
executor, which the host suite exercises but which are not formally verified, so a
compiler bug could produce a proof of the wrong program relative to the source
text, though not a proof of a program that did not run.

What is future work, stated plainly so it is not mistaken for present fact: a
random-access memory, which would need a permutation argument over sorted accesses
for consistency and would be a future major version; a hash primitive as its own
constraint region; full zero-knowledge, a hiding proof, since a `secret` input is
a private witness kept out of the public statement but the STARK is not yet hiding
so the openings could leak trace values; and the on-chain settlement of the NOX
fee. None of these is in the tree, and no claim
above depends on them.

# Related work

zKølang sits in the family of STARK-based provable virtual machines. Cairo
[@cairo] is a general Turing-complete machine with an algebraic instruction set and
a nondeterministic read-only memory proven by a permutation argument. RISC Zero
[@risczero] and SP1 [@sp1] prove RISC-V execution, trading a familiar target and
full generality for a large interpreter circuit and a memory argument. zKølang makes
the opposite trade on purpose: a tiny, branchless, statically-wired language, which
lets register binding be linear and public rather than a grand product, and which
shares one field with the operating system's own prover. It is smaller, field
shared, and OS-native, and it is not trying to be a general zkVM.

# Conclusion

zKølang shows that a verifiable-compute language can be small, legible, and honest
about its edges. By naming registers statically it turns register binding into a
public linear circuit, which keeps the step AIR at a size a person can audit, and
by sharing the kernel's field it makes a run and its proof one object. The
branchless core, register binding, and public input and output binding are built
and proven, with no unimplemented instruction; a random-access memory is a
deliberate future direction, not a missing piece. The NOX fee makes proving a
paid operation whose price follows its work. The result is a foundation on which
a memory extension can be added later as its own constraint region, held to the
same standard of evidence.

# Appendix A: Opcode table

Every opcode is enforced by the step AIR.

| Opcode | Fields | Effect |
|---|---|---|
| `Imm` | `d, v` | `r_d = v` |
| `Add` | `d, a, b` | `r_d = r_a + r_b` |
| `Sub` | `d, a, b` | `r_d = r_a - r_b` |
| `Mul` | `d, a, b` | `r_d = r_a * r_b` |
| `Inv` | `d, a` | `r_d = r_a^{-1}` |
| `Sel` | `d, c, a, b` | `r_d = r_c ? r_a : r_b` |
| `Eq` | `d, a, b` | `r_d = (r_a == r_b)` |
| `Bool` | `a` | `r_a` in `{0,1}` |
| `Assert` | `a` | `r_a = 0` |
| `Inp` | `d, idx` | `r_d = public_input[idx]` |
| `Out` | `a, idx` | `public_output[idx] = r_a` |
| `Halt` | | end of program |

# Appendix B: Constraint summary

Trace width thirty-five: clock (1), selectors (12), operands A B C (3), result D
(1), immediate (1), auxiliary (1), register file (16). Window two rows.
Forty-six transition constraints of degree at most three: selector booleanity (12),
one-hot sum (1), clock step (1), opcode semantics (11: immediate, add, subtract,
multiply, two for inverse, two for equality, two for select, boolean, assert, and
one for input), read binding (3), write propagation (16). Boundary constraints: the
fixed set (clock start, final halt, sixteen register initializations) plus one per
public input and one per public output. Sixty-four periodic columns carry the
public wiring. Proof parameters: thirty-two queries, sixteen grinding bits, three
extra blowup bits.

# Appendix C: Notation

`Fp` is the Goldilocks field with modulus p = 2^64 - 2^32 + 1. `r_d`, `r_a`, `r_b`,
`r_c` are registers named by compile-time index. A trace row is one machine step;
the trace is the matrix of rows. A selector is a one-hot column choosing an opcode.
The register file columns hold register state before a row. A periodic column is a
public per-row constant the verifier recomputes. microNOX is one millionth of a
NOX.

# Appendix D: A worked proof

Take `input x; let y = x * x * x; output y;` on `x = 3`. The compiler emits: read
input into a register, two multiplies to form `x^2` and `x^3`, an output of the
last register, and halt. The executor runs these five opcodes, producing a
five-row trace padded to eight rows. The AIR pins the input row's immediate column
to the committed input three and the output row's operand column to the committed
output twenty-seven, threads the register file so each multiply reads the live
prior result, and the money-grade STARK proves the whole matrix. Verification, over
the same reconstructed AIR, accepts. Substituting a committed output of twenty-eight
makes the output boundary disagree with the trace, and verification rejects; this
is `forged_public_output_is_rejected` in the host suite.

# References
