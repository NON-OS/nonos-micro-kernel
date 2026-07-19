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

## Binding the wiring to the program: a soundness obligation

The step AIR's periodic columns are not fixed constants. `air/air_impl.rs`
`periodic_columns` derives them from the program's wiring, and `air/wiring.rs`
`WireRow::of` builds that wiring from the opcodes. They are the program's data
flow, one-hot columns for which register each row writes and reads. So the
recursion's preprocessed periodic root must be bound to the program commitment.
Otherwise a prover satisfies program X's transition constraints while committing
program Y's routing: the transitions get checked against a wiring the committed
program never had. The shield recursion never had this obligation because its
periodic columns were fixed; it is the one genuinely new thing the zKølang
recursion must get right.

The binding is well-defined, because the wiring is a deterministic public function
of the committed program. `WireRow::of` maps each opcode to its read and write
ports with no witness input, and `serialize` commits the exact opcode list, so
`periodic_root = f(programCommit)` is a public, recomputable relation.

Two sound ways to enforce it:

- Re-derive the wiring inside the recursion from the committed opcode list and
  check the periodic root against it. Sound, but it arithmetizes `WireRow::of` over
  the whole program.
- Expose the periodic root as a recursion public input and enforce
  `periodic_root == f(programCommit)` outside the heavy circuit: a one-time
  per-program registration that anyone can recompute and challenge, pinned by the
  contract against the posted program commitment.

For a first launch the registration path is the lighter one: the wiring root is a
cheap deterministic function a deployer or a challenger can recompute, so it does
not need to be derived inside the recursion. The prover side can expose that
computation as a helper so the registration is a library call.

## The verifier-key helper, and its trust model

The prover side now exposes the registration primitive. `nonos_zkolang::periodic_root(program, extra_blowup_bits)` returns the 32-byte preprocessed-periodic root, computed through the same `nonos-stark` function the preprocessed prover commits its periodic tree with (`air/periodic_root.rs`, shared by `prove_ext_pre`), so a registered root and a proof's committed root are the same object by construction. `nonos_zkolang::verifier_key(program, extra_blowup_bits)` returns
`keccak256(descriptor ‖ periodic_root)`, where the descriptor is a wiring-version byte, the 32-byte program commitment (which commits the whole opcode list, and so every boundary and wiring position), the padded trace length, the trace width, and the FRI rate. keccak256 is the tree's own hash and the contract's on-chain hash, so a challenger recomputes both halves from the committed opcode list.

The golden test is the strong form the obligation deserves: it proves a real program with the preprocessed prover and requires the preprocessed verifier to accept using the helper's root and reject a wrong one (`vkey_tests.rs`,
`the_helper_root_is_the_prover_baked_root`). Same code path both sides, checked end to end.

The trust model is stated plainly, not discovered later. Because option 2 has the recursion expose `periodic_root` but not prove `periodic_root == f(programCommit)`, the registry row is the only binding, so a wrong row is directly exploitable, not merely a liveness bug: an attacker who registers `programCommit(X) -> f(Y)` can settle a proof carrying X's commitment under Y's routing. On-chain adjudication cannot save a permissionless set-once registry, because the contract cannot recompute `f` in EVM gas, so a challenge has nothing to adjudicate against. At launch, registration is governance-only: the same multisig that gates everything going live writes each row after validating it off chain with the helper, and can correct it. Governance is the adjudication path, resolved by the multisig rather than by on-chain recomputation. A permissionless registry is a later addition that needs a succinct validity proof of `periodic_root == f(programCommit)` verified once per program, not a challenge window. In-circuit derivation of the root is not the cheap alternative to either: checking the periodic root inside the recursion means arithmetizing the commitment itself, the coset extension and the keccak tree over the whole evaluation domain, which is a different design (a Poseidon-committed periodic tree with per-query in-circuit membership), the permissionless endgame, not an add-on.

The `wiring_version` byte fronts the descriptor so a future change to the wiring derivation or the canonical serialization is a new key, never a silent re-registration. The canonical serialization (`commit.rs::serialize`, version 1) is frozen; a change to it is a `wiring_version` bump.

## The FRI rate, made canonical on-chain

`periodic_root` and `verifier_key` are parameterized by `extra_blowup_bits`, the
FRI rate the recursion's inner proof uses, because the periodic evaluation domain
scales with it. That value, call it R, is the one number neither the prover nor the
contract can infer: the recursion's inner proof fixes it. The registry makes it
un-driftable. `ProgramRegistry` holds R as an immutable `extraBlowupBits` set at
deploy, and `registerProgram` reverts `RateMismatch` unless the caller's rate equals
it. So governance registers `periodic_root(program, registry.extraBlowupBits())`,
and a wrong rate fails loudly at registration rather than bricking every later proof
with a `WiringMismatch`. The single open value is R itself, which the STARK team
names for the recursion's inner proof; every other party reads it from the deployed
registry.
