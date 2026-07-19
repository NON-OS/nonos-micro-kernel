<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Proving economics

Proving is work, and zKølang treats it as a paid operation settled in NOX. This page
describes the fee model that is in the code today and marks clearly what is not yet
wired.

## The model

A proof has a price that follows the proving work. The work is the trace area, the
number of rows times the width, because the prover's field arithmetic, its
commitments, and its low-degree test all scale with it. Charging on the trace area
aligns the price with the cost and cannot be gamed by padding, which only makes the
bill larger.

The quote, computed by `quote` in `userland/nonos_zkolang/src/nox.rs`, is a small
flat floor plus a rate on the area, split into a payment to the prover and a cut to
the protocol:

```
  cells    = trace_len * trace_width
  compute  = cells * 50 microNOX / 1000 cells
  total    = 1000 microNOX (floor) + compute
  protocol = total * 5%          (accrues to the NOX treasury)
  prover   = total - protocol
```

The floor keeps submitting work from ever being free, so the market is not a spam
vector. The rate and the protocol cut are constants in `nox.rs`, meant to be
tuned by governance; the shape, a floor plus an area rate with a basis-point
protocol cut, is the part that matters.

## Measured prices

These are produced by `cargo run --release --example measure` in
`userland/nonos_zkolang_proofs`:

| program | steps | trace | cells | fee (microNOX) | prover | protocol |
|---|---|---|---|---|---|---|
| demo (add, mul, assert, output) | 9 | 2^4 x 35 | 560 | 1028 | 977 | 51 |
| square, y = x^2 | 4 | 2^2 x 35 | 140 | 1007 | 957 | 50 |
| cube, y = x^3 | 5 | 2^3 x 35 | 280 | 1014 | 964 | 50 |
| degree eight, y = x^8 | 6 | 2^3 x 35 | 280 | 1014 | 964 | 50 |

A larger program pads to a larger trace and costs more, and the prover always
keeps the majority. The `nox_tests.rs` suite checks these invariants: the split is
exact, both sides are paid, and a larger trace costs at least as much.

## The business shape

The utility is straightforward. A buyer has a computation it wants a trustworthy
answer to and does not want to run or re-run: an off-chain calculation, a solvency
check, a game move, an agent's decision. It submits the zKølang program and the
public inputs with a NOX fee. A prover produces the STARK and returns it; on
verification the fee is released, most to the prover, a cut to the protocol. NOX is
the required settlement token, so proving traffic is demand for NOX, and the
protocol cut is revenue that accrues to the treasury as volume grows. Because a
proof binds its public inputs and outputs, the buyer pays for a statement about
public data, not for an unverifiable promise.

## What is code and what is not

The quote is code and is tested. The settlement itself, escrow of the fee, release
on verification, and the protocol cut moving on-chain, is not wired into these
crates; it is the job of the NOX rail and its contracts, which live outside
`nonos_zkolang`. This page describes the pricing primitive that is built and the
market it is designed for, and does not claim the on-chain settlement is done.
