<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# The recursion seam and the on-chain ABI

This page pins the one interface that connects three pieces that are otherwise
built: the zKølang prover, the recursive verifier, and the `ProvingMarket`
contract. It is written so the STARK team can route a zKølang proof through the
recursion (launch gap 1) and the smart-contract team can match the ABI, with no
guesswork on either side. Everything below is fixed by code that exists today
(`src/commit.rs`, `src/driver.rs`); the only new work is the recursion exposing
these values as its public inputs.

## The statement the prover already binds

`prove_program` in `src/driver.rs` binds a public statement into the proof by
seeding the Fiat-Shamir transcript through `stark_prove_poseidon_ext_pub`. The
seed is a flat vector of field elements, in this exact order:

```
  publics = commit_limbs(program)   // 4 field elements, the program commitment
          ++ [ trace_len ]          // 1 field element, the padded trace length
          ++ public_inputs          // P field elements, in declaration order
          ++ public_outputs         // Q field elements, in declaration order
```

The verifier `stark_verify_poseidon_ext_pub` replays exactly this vector. A proof
therefore is already bound to its program, its trace length, and its public inputs
and outputs. What is missing for the chain is only that the recursion re-expose
this same vector as its own public inputs, so a succinct on-chain verifier checks
it.

## The program commitment: bytes and field limbs

`commit(program)` is `blake3(serialize(program))`, a 32-byte digest over the
versioned canonical `Op` encoding. This is the `programCommit` a job is posted
against on chain, a `bytes32`.

`commit_limbs(program)` splits the same 32 bytes into four field elements, each
the little-endian `u64` of an eight-byte word reduced into the field:

```
  limb[i] = Fp( u64_le( digest[8*i .. 8*i+8] ) )   for i in 0..4
```

So the four limbs carry the whole commitment. The recursion and the contract must
agree that `programCommit` decomposes into these four limbs in this order and
encoding. The contract holds the `bytes32`; the recursion holds the four limbs;
they are the same 32 bytes.

## Field and word encoding

Every field element is canonical, in `[0, p)` with `p = 2^64 - 2^32 + 1`, so it
fits in a `uint256` (indeed a `uint64`). The contract's `publicInputs` and
`publicOutputs` are `uint256[]`, each entry the canonical `u64` of one field
element. `trace_len` is a small integer, at most `2^16`, also one field element.

## What the recursion must expose

The recursive verifier attests that a zKølang STARK verifies. For the chain it must
expose, as its public inputs, the same `publics` vector the base proof was bound
to, in the same order:

```
  recursion_public_inputs =
      program_commit_limbs[0..4]    // the 4 limbs above
      trace_len                      // 1
      public_inputs[0..P]            // P
      public_outputs[0..Q]           // Q
```

with `P` and `Q` fixed by the program (the count of `input` and `output`
statements). Nothing else needs to cross the boundary. A verifier that checks the
recursion and reads these public inputs has checked exactly the statement the
buyer paid for.

## The contract mapping

`IZkolangVerifier.verify(programCommit, publicInputs, publicOutputs, proof)` maps
to the recursion public inputs like this:

- `programCommit` (`bytes32`) is split into the four limbs and compared against
  `recursion_public_inputs[0..4]`.
- `publicInputs` (`uint256[]`, length `P`) is compared against
  `recursion_public_inputs[5 .. 5+P]`.
- `publicOutputs` (`uint256[]`, length `Q`) is compared against
  `recursion_public_inputs[5+P .. 5+P+Q]`.
- `trace_len` is `recursion_public_inputs[4]`, read by the market for the fee
  check below.

The `proof` bytes are the succinct recursion proof the Solidity verifier checks.

## Closing gap 4 on chain

Today the market enforces `fee <= maxFee` because it cannot see the trace. Once the
recursion exposes `trace_len` as a verified public input, the market can enforce
the exact fee. The fee is deterministic in the trace area, `trace_len * trace_width`,
by `quote` in `src/nox.rs`. The trace width is the fixed constant `TRACE_WIDTH`
(35). So the market computes:

```
  cells    = trace_len * 35
  expected = 1000 + (cells * 50) / 1000     // microNOX, matching quote()
  require(fee == expected)
```

and pricing is bound to proven work, not to the prover's declaration. This needs
no new proof machinery, only the one public input.

## Status of the seam

Built and fixed by code: the statement layout, the commitment encoding, the field
encoding, and the fee formula. Remaining, and the sole dependency for a trustless
launch: the recursion exposing `recursion_public_inputs` in the order above, and a
Solidity verifier for the recursion's succinct proof. That is the recursive
verifier's per-query work, tracked as the on-chain verification gap. The market is
already fail-closed against it, so nothing settles until it lands.
