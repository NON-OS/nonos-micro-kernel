# The NØNOS in-kernel STARK

A transparent, post-quantum proof system built from the field up, in `no_std`
Rust, with no trusted setup and no cryptography beyond a hash and field
arithmetic. It lets the kernel prove and verify statements about a computation
(today: knowledge of a Poseidon preimage) without revealing the witness, and
without any assumption a quantum computer breaks.

This document is the map of the system and, more importantly, the honest
statement of what it proves and what it assumes. The claims here are the ones
a reader should audit rather than take on faith.

## The pipeline

```
field/        Goldilocks arithmetic, p = 2^64 - 2^32 + 1
poly/         Lagrange evaluation and the low-degree extension
merkle/       a BLAKE3 Merkle commitment with domain-separated leaves and nodes
transcript    Fiat-Shamir over BLAKE3: challenges folded from the committed data
fri/          the FRI low-degree test, binary folding to a constant layer
air/          the AIR: a trace with transition and boundary constraints, the
              generic prover and verifier, and periodic public columns
poseidon/     Poseidon over Goldilocks, the algebraic hash the AIR reasons about
```

A proof commits each trace column with Merkle, draws constraint-composition
coefficients from the transcript, forms the constraint composition over an
evaluation coset, proves that composition is low degree with FRI, and opens
the sampled positions so the verifier can rebuild the composition from the
committed trace and check it matches. The verifier only reads the proof.

## What is proven, here, on this code

The host proofs in `userland/stark_proofs` include this source unmodified and
check it:

- The Goldilocks field, the Lagrange evaluation, and the low-degree extension
  against their specifications.
- The Merkle commitment: honest openings verify, and a tampered leaf, path,
  or root, or a path of the wrong length, is rejected.
- FRI: an honest low-degree codeword verifies on the subgroup and on a coset
  and across sizes, and a high-degree codeword is rejected (the low-degree
  test bites).
- The Fiat-Shamir transcript: the challenge stream is a deterministic function
  of the label and the absorbed sequence, a one-bit change anywhere in the
  absorbed data changes every later challenge, absorb order matters, digest
  and field absorbs are domain separated, and the query indices stay in range.
- Poseidon: the permutation matches the four published Plonky2 reference
  vectors (produced by the hadeshash reference), so the constants are the real
  set, not an invented one.
- The end-to-end Poseidon preimage STARK: an honest preimage verifies, and a
  wrong digest, a corrupted round, and a nonzero capacity seed are rejected.
- The verifier under forgery: every single-field mutation of a valid proof,
  every structural malformation, and proofs from arbitrary bytes are rejected
  without a panic. The verifier reads attacker-supplied data, so this is the
  property that matters.

## What is assumed, and must be stated

A proof reduces trust; it does not eliminate it. This system rests on:

- **The random-oracle model.** Fiat-Shamir instantiates the verifier's
  challenges with BLAKE3. Soundness of the non-interactive proof is in the
  random-oracle model, with BLAKE3 as the oracle. The transcript tests show
  the construction binds and separates as a random oracle would; they do not
  prove BLAKE3 is one.
- **The FRI low-degree soundness bound.** That a codeword far from every
  low-degree polynomial fails the sampled checks is the standard FRI soundness
  statement. The tests confirm the verifier rejects high-degree and tampered
  codewords; the quantitative soundness of FRI is a cryptographic assumption,
  not a theorem proven in this tree.
- **BLAKE3 as a collision-resistant hash** for the Merkle commitment, checked
  against its own reference vectors elsewhere in the tree.
- **The Poseidon parameters**, taken from the published Plonky2 set and pinned
  by the reference vectors.

There is no trusted setup and no structured reference string: the only public
parameters are the field, the hash, and the Poseidon constants, all fixed and
auditable.

## The security level, stated plainly

The evaluation rate is 1/2 (a blowup of two), and FRI folds by two to a
constant layer. The soundness error falls with the number of query openings;
the proofs draw 32. That query count, together with the rate, sets the
concrete security margin, and reaching a production margin (a lower rate, more
queries, and out-of-domain sampling for a tighter bound per query) is a
parameter and protocol refinement that is ongoing, not a change to the code
documented here. The field is roughly 64 bits, so a statement needing more
than field-sized soundness would move to an extension field; the present
statements do not.

## Post-quantum

The only cryptography is a hash and field arithmetic. There is no discrete
log, no pairing, no factoring. A quantum adversary gains only the generic
square-root speedup against the hash, which the digest width already accounts
for. The system is post-quantum by construction, which is the reason to build
it from the field up rather than reach for an elliptic-curve SNARK.

## Reproduce

```sh
cd userland/stark_proofs
cargo test --release        # field, poly, merkle, fri, transcript, air, poseidon, forgery
```

The kernel builds the module for its real target with
`make nonos-mk-core`.
