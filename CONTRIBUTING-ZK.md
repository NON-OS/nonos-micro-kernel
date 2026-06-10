# Contributing verifiable work to NONOS

NONOS pays NOX for security work that anyone can re-verify from public
artifacts. No trust in us is required: every claim is backed by a receipt
that an independent party can re-check with the open tools in this repo,
and rewards settle from epoch Merkle roots that bind your address, your
receipt and the amount.

What is live today: the 5-contributor external trusted-setup ceremony,
the Groth16 attestation of every capsule (verifying key fingerprint
6cd2015037ea6181), the enforced runtime attestation gate, the receipt
tooling described here, and the reward contracts on Ethereum mainnet
under a 3/5 Safe (addresses in abi/nox_deployment.json, custody and
audit record in abi/NOX_CONTRACTS.md). Receipts accumulate toward the
first epoch root; the pool pays only NOX that demand revenue funded.

## The one command

    make nonos-mk-nox CONTRIB=0x<your reward address>

From a clean checkout this builds the toolchain and every capsule, runs a
live BLS12-381 pairing check on the whole fleet against the ceremony
verifying key, issues a FLEET_VERIFICATION receipt bound to your address,
re-verifies that receipt the way an authorizer would, and exports the
canonical circuit registry entry. Everything lands under target/nox/.

## What work earns a receipt

    CEREMONY_ROUND       you contributed a round to a trusted-setup ceremony
    FLEET_VERIFICATION   you ran the full-fleet pairing check yourself
    RUNTIME_BOOT         you booted NONOS in QEMU and captured the serial log
                         with enforced [ZK-ATTEST] ok lines
    HARDWARE_BOOT        the same evidence from real hardware
    CIRCUIT_AUDIT        you audited the attestation circuit and wrote it up
    CAPSULE_AUDIT        you audited a capsule and wrote it up
    CAPSULE_BUILD        you built a capsule reproducibly; its proof trailer
                         is re-proved live before a receipt is issued

The emitter validates the artifact before issuing anything. A fleet
report with failures, a boot log with a failed attestation, a capsule
whose proof does not verify, or a verifying-key fingerprint that does not
match the ceremony key all refuse to produce a receipt.

## Individual steps

Emit a receipt for any kind of work:

    make nonos-mk-nox-receipt CONTRIB=0x<address> KIND=RUNTIME_BOOT \
        ARTIFACT=target/boot-test-desktop-gui.log EPOCH=1

Re-check any receipt against the raw artifacts (this is exactly what the
authorizer runs before accepting it; it prints the verifier hash):

    make nonos-mk-nox-verify RECEIPT=target/nox/receipts/RUNTIME_BOOT-epoch1.json \
        ARTIFACT=target/boot-test-desktop-gui.log

Ceremony contributors get their round receipt from the transcript:

    nox-zk-receipt --transcript ceremony_transcript.json \
        --verifying-key attestation_verifying_key.bin \
        --round <your round> --contributor 0x<address> --out receipt.json

Build the epoch claim tree from accepted receipts (maintainers):

    make nonos-mk-nox-merkle CLAIMS=allocations.json EPOCH=1

Export the circuit registry entry:

    make nonos-mk-nox-registry

## The loop in one picture

```
            you                          the project                Ethereum mainnet
   ----------------------       --------------------------    ------------------------
   do verifiable work
   (fleet run, boot
    witness, audit,
    ceremony round)
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

1. You produce work and a receipt. make nonos-mk-nox submits it for
   you, artifact included; the submission endpoint re-runs the same
   verification library before accepting anything and answers with
   your verifier hash. If the endpoint is unreachable the receipt
   stays local and one command resends it:

       make nonos-mk-nox-submit RECEIPT=path ARTIFACT=path

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

Verification is off-chain with open tooling; v1 contracts do not verify
Groth16 proofs on-chain. A receipt is evidence, not a payout promise:
acceptance into an epoch root is a human decision backed by a mechanical
re-check that anyone can reproduce.
