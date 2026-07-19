<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Task brief: route a zKølang proof through the recursion

This is the one remaining dependency for a trustless on-chain launch, written for
the STARK team. Everything on the prover side and the contract side is built; this
task connects them. It is the per-query loop of the recursive verifier, plus
exposing a fixed public-input vector.

## What exists to build against

- The base proof. `stark_prove_poseidon_ext_pub` and its verifier counterpart
  `stark_verify_poseidon_ext_pub` (both in `nonos-stark`) prove and check a
  zKølang trace bound to a public statement. The zKølang driver already calls the
  `_pub` pair, seeding the transcript with the statement.
- The statement. The exact vector the base proof is bound to, from
  `userland/nonos_zkolang/src/driver.rs` and `src/commit.rs`, is fixed. See the ABI
  page (`09-recursion-abi.md`) for the byte and field encoding.
- The contract. `ZkolangVerifier.verify(programCommit, publicInputs, publicOutputs,
  proof)` is built and fail-closed, waiting for the succinct proof this task
  produces. It is documented in `08-nox-utility-contracts.md`.

## The deliverable

A recursive verifier that arithmetizes `stark_verify_poseidon_ext_pub` over a
zKølang `StepAir` proof and produces a succinct proof a Solidity verifier can
check in EVM gas. It must expose, as its own public inputs, this vector in this
order (all field elements):

```
  commit_limbs[0..4]     the program commitment, four little-endian u64 limbs
  trace_len              the padded trace length, one element
  public_inputs[0..P]    the program's public inputs, in order
  public_outputs[0..Q]   the program's public outputs, in order
```

`P` and `Q` are fixed by the program (the count of `input` and `output`
statements). This is the identical vector the base proof is bound to, so the
recursion attests to exactly the statement the base proof committed.

## The per-query loop

The base verifier's soundness is in its query loop: for each of `n_queries`
sampled points it checks the trace, composition, and DEEP openings against the
Poseidon Merkle roots and the out-of-domain frame (see
`nonos-stark/src/air/verify_poseidon_ext.rs`, the loop over `proof.queries`). The
recursion must arithmetize that loop: the transcript replay, the index draws, the
Merkle path checks, and the DEEP consistency equation, so that a satisfying
recursion trace implies every query passed. This is the same shape as the shield
recursion already in the tree; the difference is only the AIR under it and the
public-input vector above.

## Acceptance criteria

1. An honest zKølang proof, taken from the driver, verifies through the recursion,
   and the recursion's public inputs equal the vector above.
2. A proof bound to one statement fails the recursion under a forged program
   commitment or a forged trace length, matching the base-verifier behaviour that
   `commit_tests.rs` already checks off chain.
3. The succinct proof verifies in the Solidity `ZkolangVerifier`, and the market's
   fee check can read `trace_len` from the public inputs to enforce
   `fee == 1000 + trace_len * 35 * 50 / 1000` microNOX.

## Not in scope

The base prover and verifier, the statement binding, the program commitment, and
the market are all done. This task is only the recursion and its public-input
surface. When it lands, the market goes live with no further design on either the
prover or the contract side.
