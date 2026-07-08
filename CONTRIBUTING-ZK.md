# Contributing verifiable work to NONOS

NONOS previously documented a Groth16/ceremony-based NOX receipt lane.
That path has been removed. There is no trusted setup anywhere in the
active build, and no reward or release claim may be issued from the old
receipt commands.

What is live today rests on two proof stacks, both transparent, both in
CI on every push:

- A capsule attestation gate enforced in the kernel spawn path. The
  live gate is the transparent enrolled-secret proof (NZKCAPS2
  trailers, a Curve25519 Pedersen commitment with a BLAKE3 Merkle
  membership). Its post-quantum successor, a Poseidon-Merkle STARK that
  hides the same secret and reveals the same path, is under
  construction; its verifier is already proven in Lean before it is
  allowed to gate a spawn.
- A from-scratch transparent post-quantum proof system in
  `src/crypto/stark`: the Goldilocks field, Poseidon and BLAKE3 Merkle
  commitments, FRI, a generic AIR prover and verifier, and a
  recursive, monolithic FRI verifier. Hash-and-integer only, no
  elliptic curve, no trusted setup, so verification is post-quantum by
  construction. It ships with host known-answer tests, a forgery fuzz
  that mutates real proofs and demands rejection, and Lean soundness
  theorems.

The formal-verification stack is the reference for what is actually
proven and what is assumed: `verification/ARCHITECTURE.md`,
`verification/MAP.md`, and `verification/STATUS.md`. Lean states the
abstract theorems (`verification/lean`, checked with no `sorry` and no
added axioms), Verus and Kani discharge them on the real Rust, and the
runnable known-answer and fuzz crates check the primitives against the
published vectors. Read those before auditing; do not take any claim
here on faith.

## What work earns a receipt

    FLEET_VERIFICATION   you rebuilt and verified the transparent capsule fleet
    RUNTIME_BOOT         you booted NONOS in QEMU and captured the serial log
                         with enforced [ZK-ATTEST] ok lines
    HARDWARE_BOOT        the same evidence from real hardware
    PROOF_AUDIT          you audited the STARK verifier or the attestation
                         path against its Lean and host proofs and wrote it up
    CAPSULE_AUDIT        you audited a capsule and wrote it up
    CAPSULE_BUILD        you built a capsule reproducibly; its proof trailer
                         is re-proved before a receipt is issued

The transparent receipt emitter is not currently wired. Until it lands,
accepted evidence is limited to source artifacts and maintainer review.

## The loop in one picture

```
            you                          the project                Ethereum mainnet
   ----------------------       --------------------------    ------------------------
   do verifiable work
   (fleet run, boot
    witness, audit,
    audit)
        |
        v
   nox-work-receipt              nox-receipt-verify
   validates the artifact,  -->  re-checks everything   -->   accepted receipts enter
   refuses bad evidence,         from the raw artifacts        the epoch allocation
   binds it to your addr         and prints the
        |                        verifier hash
        |                                                      nox-merkle builds the
        |                                                      tree; root published +
        |                                                      finalized on the
        |                                                      RewardRootManager
        v                                                            |
   claims.json names the                                             v
   chain, the pool and    <----------------------------------  you claim from
   your Merkle proof                                           NoxRewardPool; the pool
                                                               pays only NOX that
                                                               demand revenue funded
```

## How settlement works

1. You produce work and a receipt after the transparent receipt emitter
   is restored. The submission endpoint must re-run the same verification
   library before accepting anything and answer with a verifier hash.

2. An authorizer reviews the accepted spool; mechanical validity was
   already enforced at the door, so human judgment is only spent on
   audits and edge cases.
3. Accepted receipts for an epoch go into a Merkle tree built by
   nox-merkle. Each leaf binds your address, the receipt id, the circuit
   id, the amount, the epoch and the pool id.
4. The root is published on-chain and finalized. You claim from
   NoxRewardPool with the proof from claims.json, whose root.json
   names the chain and the deployed pool and root-manager addresses.
   The pool pays only NOX it was funded with; nothing is minted.

The receipt and schema formats are frozen in abi/ (
nox_zk_contribution_receipt.schema.json and
nox_zk_work_receipt.schema.json). The identifier derivations and the
on-chain interface are documented in abi/NOX_CONTRACTS.md.

## What is not claimed

Verification is off-chain with open tooling. A receipt is evidence, not
a payout promise: acceptance into an epoch root remains a human decision
backed by a mechanical re-check that anyone can reproduce once the
transparent receipt tooling is restored.
