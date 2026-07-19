<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Recipes

Real uses of zKølang, each a computation a buyer would pay to have proven. Every
one is a tested program in `userland/nonos_zkolang_proofs/src/recipes_tests.rs`;
run `cargo test` there to reproduce them. Public inputs are supplied to the
prover and enter the statement; `secret` inputs are private witnesses that do not.

## Delegated computation

Prove the value of a public function without the verifier running it. Here a
polynomial:

```
input x;
let y = 3 * x * x + 2 * x + 5;
output y;
```

On `x = 2` this proves `y = 21`. The verifier reads the output off the proof and
trusts it, having checked a proof far cheaper than the computation. This is the
plain delegate-and-verify case: everything is public.

## Knowledge of a secret solution

Prove there is a solution to a public equation, without revealing it. A root of a
public quadratic, rearranged so the coefficients are positive:

```
input b;
input c;
secret x;
assert x * x + c - b * x;   // x^2 + c == b*x
```

With `b = 5, c = 6` the roots of `x^2 - 5x + 6` are 2 and 3, and either proves; a
non-root has no proof. The verifier learns that the prover knows a root, not which
one.

## Private set membership

Prove a secret value is on a public allowlist, without revealing which entry:

```
secret w;
input v0; input v1; input v2;
assert (w - v0) * (w - v1) * (w - v2);
```

The product is zero exactly when `w` equals one of the three, so a proof attests
membership while `w` stays hidden. This is the shape of an allowlist or a
credential check.

## Solvency by conservation

Prove knowledge of private balances that sum to a public total, without revealing
the balances:

```
secret a; secret b; secret c;
let s = a + b + c;
output s;
```

The output is the public sum; the parts are hidden. The same shape proves reserves
add up to a published figure.

## A range proof

Prove a secret value lies in a range by exhibiting its bits, each constrained
boolean, that reconstruct it. Here `[0, 16)`:

```
secret w;
secret b0; secret b1; secret b2; secret b3;
assert b0 * b0 - b0;   // each bit is 0 or 1
assert b1 * b1 - b1;
assert b2 * b2 - b2;
assert b3 * b3 - b3;
assert w - (b0 + 2 * b1 + 4 * b2 + 8 * b3);
```

The prover supplies `w` and its bits as the private witness. This is the pattern
behind proving an age is over a threshold or a balance is within a limit without
revealing the value. Because the compiler reuses registers for dead temporaries,
the bit work fits the sixteen-register file, and wider ranges follow the same
shape.

## Reusable relations with functions

A `fn` names a relation so it is written once and reused. The range proof above
repeats the boolean check `b * b - b` for every bit; naming it makes the intent
plain and the source short. The call is inlined, so the proof is byte-for-byte the
same:

```
fn bit(b) = b * b - b;
secret w;
secret b0; secret b1; secret b2; secret b3;
assert bit(b0);
assert bit(b1);
assert bit(b2);
assert bit(b3);
assert w - (b0 + 2 * b1 + 4 * b2 + 8 * b3);
```

Functions also carry a small library of arithmetic. Here a delegated weighted sum
of squares, with the square and the multiply-add each defined once:

```
fn sq(x) = x * x;
fn madd(a, b, c) = a * b + c;
input x; input y;
let r = madd(sq(x), 3, sq(y));   // 3*x^2 + y^2
output r;
```

On `x = 2, y = 5` this proves `r = 37`. Because every call is inlined, a function
costs nothing the hand-written body would not; it buys clarity, not machinery.

## What these share

None reveals more than its public inputs and outputs, each is a fixed-size program
whose proof cost is known in advance, and each is settled the same way through the
NOX market. They are small because the language is small, and that is what keeps
every one of them checkable by hand.
