<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# The language

zKølang source is a sequence of statements over field values. There are four
statement forms and a handful of expression forms, and every one lowers to a
fixed set of machine instructions. This page shows each construct next to the
opcode it produces. The lexer, parser, and compiler are in
`userland/nonos_zkolang/src/lang/`.

## Grammar

The tokens the lexer recognises are exactly `let`, `assert`, `input`, `secret`,
`output`, `inv`, `sel`, the operators `+ - * ==`, the punctuation `( ) , ;`,
identifiers, and decimal numbers (`src/lang/lex.rs`). Line comments run from `//`
to the end of the line. The grammar, lowest precedence first, is:

```
program  := stmt*
stmt     := 'let' ident '=' expr ';'
          | 'assert' expr ';'
          | 'input' ident ';'
          | 'secret' ident ';'
          | 'output' expr ';'
expr     := equality
equality := sum ('==' sum)?
sum      := product (('+' | '-') product)*
product  := primary ('*' primary)*
primary  := number | ident | '(' expr ')'
          | 'inv' '(' expr ')'
          | 'sel' '(' expr ',' expr ',' expr ')'
```

## Statements

A `let` binds a name to the value of an expression. Every value is
single-assignment: a fresh register holds each result, and a name resolves to the
most recent `let` that bound it, which gives ordinary lexical shadowing
(`src/lang/compile.rs`).

```
let a = 3;        // Imm  r0 = 3
let s = a + b;    // Add  r_s = r_a + r_b
```

An `assert` states that an expression is zero. It compiles to the zero-assertion
opcode, so it reads as "this must be zero"; to require that two values are equal,
write `assert x - y`.

```
assert p - 64;    // Assert (p - 64) == 0, i.e. p == 64
```

An `input` binds a name to the next public input. Inputs are supplied to the
prover in declaration order, and their values are bound into the proof.

```
input x;          // Inp  r_x = public_input[0]
```

A `secret` binds a name to the next private input, a witness the prover supplies
that never enters the public statement. It lets a program prove knowledge of a
hidden value that satisfies a public relation, for example a square root:

```
secret w;         // Inp  r_w = a private witness
assert w * w - 25;  // proves knowledge of a root of 25 without revealing w
```

This is a private witness, not full zero-knowledge: the STARK is not hiding, so
the openings could still leak trace values. What `secret` guarantees is that the
value is not part of the committed public statement.

An `output` exposes an expression as the next public output. Outputs are the
values a verifier reads off the proof.

```
output y;         // Out  public_output[0] = r_y
```

## Expressions

Arithmetic is field add, subtract, and multiply. Multiply binds tighter than add
and subtract; parentheses group.

```
let p = (a + b) * (a - b);   // two Adds/Subs then a Mul
```

`inv(e)` is the field inverse. The inverse of zero has no value, so a program that
inverts zero produces no valid trace and is reported as unprovable rather than
returning a wrong answer.

```
let q = inv(x);   // Inv  r_q = r_x^{-1}
```

`a == b` yields a clean bit: one when the two are equal, zero otherwise. It is an
equality test, not an assignment.

```
let e = a == b;   // Eq  r_e = (r_a == r_b) as {0,1}
```

`sel(c, a, b)` is a branchless conditional: it returns `a` when `c` is one and `b`
when `c` is zero, and `c` must be a bit. Both arms are always evaluated, which is
what keeps the trace shape independent of the data.

```
let m = sel(e, a, b);   // Sel  r_m = e ? a : b
```

## A complete program

The canonical example, from `src/lang/mod.rs`, computed and proven end to end:

```
let a = 3;
let b = 5;
let s = a + b;      // 8
let p = s * s;      // 64
let q = inv(b);     // 5^{-1}
let eqv = s == 8;   // 1
let pick = sel(eqv, a, b);  // a, because eqv is 1
assert p - 64;      // p == 64
```

A program that reads a public input, computes, and exposes a public output:

```
input x;
let y = x * x * x;
output y;           // proves y = x^3 for the committed public x and y
```

Running this on `x = 3` proves the statement and returns `y = 27`. See
[from program to proof](04-program-to-proof.md) for how to run it.

## What the surface does not have, on purpose

There are no loops or `if` blocks in the surface syntax and no function calls yet.
A bounded loop is unrolled by the front-end into straight-line code before
compilation, so it never appears as control flow in the trace. This is a
deliberate limit: it is what makes a program's step count a static property and
the proof's cost knowable before you run it. Division is written `a * inv(b)`.
