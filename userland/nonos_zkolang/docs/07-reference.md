<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Reference

## Opcodes

The instruction set (`src/isa.rs`). Every opcode is enforced by the AIR.

| Opcode | Fields | Effect |
|---|---|---|
| `Imm` | `d, v` | `r_d = v` |
| `Add` | `d, a, b` | `r_d = r_a + r_b` |
| `Sub` | `d, a, b` | `r_d = r_a - r_b` |
| `Mul` | `d, a, b` | `r_d = r_a * r_b` |
| `Inv` | `d, a` | `r_d = r_a^{-1}` (zero is unprovable) |
| `Sel` | `d, c, a, b` | `r_d = r_c ? r_a : r_b`, `r_c` a bit |
| `Eq` | `d, a, b` | `r_d = (r_a == r_b)` as `{0,1}` |
| `Bool` | `a` | constrain `r_a` to `{0,1}` |
| `Assert` | `a` | constrain `r_a = 0` |
| `Inp` | `d, idx` | `r_d = input[idx]` (public or secret) |
| `Out` | `a, idx` | `public_output[idx] = r_a` |
| `Halt` | | end of program |

## Grammar

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
```

Keywords: `let`, `assert`, `input`, `secret`, `output`, `inv`, `sel`, `for`, `in`. Operators: `+ - * / == != ` and unary `-`. Comments: `//` to end
of line.

## Public API

From `nonos_zkolang` (`src/lib.rs`):

- `compile_source(&str) -> Result<Vec<Op>, CompileError>`
- `prove_source(&str) -> Result<Report, RunError>`
- `prove_source_with_inputs(&str, &[u64]) -> Result<Report, RunError>`
- `prove_source_with_witness(&str, &[u64] public, &[u64] secret) -> Result<Report, RunError>`
- `prove_program(&[Op], &[Fp]) -> Result<Report, RunError>`
- `commit(&[Op]) -> [u8; 32]`, `commit_limbs(&[Op]) -> [Fp; 4]`, `serialize(&[Op]) -> Vec<u8>`
- `quote(&Report) -> Quote`
- `StepAir`, `Vm`, `Trace`, `Report`, `Op`, `REGS`, `TRACE_WIDTH`

`Report` carries `verified`, `steps`, `log_trace_len`, `trace_len`, `trace_width`,
`outputs`, and `program_commit` (the 32-byte commitment the proof is bound to).

## Errors

Each failure is a typed value, never a panic.

- `CompileError` (`src/lang/mod.rs`): `UnexpectedChar { at }`,
  `NumberTooLarge { at }`, `UnexpectedEof`, `UnexpectedToken`, `UnknownVariable`,
  `TooManyRegisters`, `LoopTooLarge`.
- `ProveError` (`src/vm.rs`): `BadRegister`, `BadInput`, `NoHalt`,
  `Unprovable { step }`.
- `BuildError` (`src/air.rs`): `NoHalt`, `TooLong`, `MissingPublicOutput`.
- `RunError` (`src/driver.rs`): `Compile`, `Execute`, `Layout`, `ProgramTooLong`.

## Limits

- Sixteen registers. A program needing more live values than the file holds is a
  `TooManyRegisters` compile error.
- Straight-line only. No loops or conditionals in the surface syntax; bounded
  loops are unrolled by the front-end.
- Traces up to 2^16 rows (`MAX_LOG_T` in `src/driver.rs`); a longer program is a
  `ProgramTooLong` error rather than a silent truncation.
- No random-access memory or hash primitive: zKølang is a straight-line
  arithmetic language. A memory extension is a future major version, not a
  missing part of the current one.

## Proof parameters

The driver proves at 32 queries, 16 grinding bits, and 3 extra blowup bits
(`src/driver.rs`), the same money-grade setting the `nonos-stark` tests use. The
trace width is 35 and the AIR has 46 transition constraints of degree at most
three.

## FAQ

**Is this zero-knowledge?** Partly, and the docs are careful about which part. A
`secret` input is a private witness: it feeds the run and never enters the public
statement, so a proof can attest knowledge of a hidden value that satisfies a
public relation, for example a square root of a public number. That is a private
witness, not full zero-knowledge: the STARK is not hiding, so a determined verifier
could learn trace values from the query openings. Hiding the witness completely is
a further hardening, and is not claimed today.

**What is proven versus assumed?** The AIR constraints listed in
[the AIR](05-the-air.md) are proven: opcode semantics, register binding, ordering,
and public input and output binding. Assumed is the soundness of the underlying
STARK, which is the subject of the NØNOS verification paper, and the correctness of
the compiler and executor, which are checked by the host proof suite but not
formally verified.

**Can I trust the answer if `verified` is true?** You can trust that the program
whose text the verifier holds ran and produced the stated outputs on the stated
inputs. Whether that program is the right program for your purpose is your
judgement, not the proof's.

**Where do I start?** Read [the language](02-language.md), then run `zkolang` in
the terminal, then write a `.zkl` file and run `zkolang myfile.zkl`.

## Reproducing the claims

In `userland/nonos_zkolang_proofs`: `cargo test` runs the 52 host proofs behind this
documentation (opcode tamper rejection, register binding, public input and output
soundness, the language end to end, and the fee model), and
`cargo run --release --example measure` prints the trace shapes and fees.
