<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# The NOX utility and the contracts to build

This page is the specification for the on-chain side of zKølang: the pay-to-prove
market settled in NOX. It is written for the smart-contract team. The language,
the prover, and the fee model are built and live in this crate; the contracts here
are what turn a proof into a paid, settled transaction. Every interface below is
concrete enough to build against, and the section at the end lists the real-world
gaps that must close before a public launch, with none of them papered over.

## What the token is for

NOX is the settlement token, and its utility is direct rather than narrative. A
proof is a service with a real cost, and NOX is the only accepted payment for it.
Three flows give the token its demand and its revenue:

1. Every proving job is paid in NOX. Proving traffic is NOX demand.
2. A protocol cut of each fee accrues to the NOX treasury. This is protocol
   revenue that scales with proving volume, not with token speculation.
3. Provers post a NOX bond to take jobs, which is returned on honest delivery and
   is the anti-spam and liveness stake. This locks supply proportional to market
   activity.

The fee itself is deterministic in the proving work, computed by `quote` in
`src/nox.rs`: a floor plus a rate on the trace area, split into a prover payment
and the protocol cut. The contracts settle exactly that split.

## The settlement flow

```
  buyer                     ProvingMarket                 prover
    |  postJob(stmt, maxFee) escrow NOX   |                   |
    |------------------------------------>|                   |
    |                                     |   claimJob(bond)  |
    |                                     |<------------------|
    |                                     |  submitProof(...) |
    |                                     |<------------------|
    |                                     |-- verify proof -->| ZkolangVerifier
    |                                     |   on true:        |
    |                                     |   FeeRouter.split(prover, treasury)
    |                                     |   return bond, record outputs
    |         outputs + receipt           |                   |
    |<------------------------------------|                   |
```

A buyer escrows NOX against a statement. A prover claims the job with a bond,
runs zKølang, and submits the proof and the public outputs. The market verifies
the proof on chain, and only on success releases the fee through the fee router
and returns the bond. If no valid proof arrives by the deadline, the buyer
reclaims the escrow and the bond is forfeit.

## The contracts

### 1. ZkolangVerifier

The trust root. It answers one question: for a committed program, these public
inputs, and these claimed public outputs, does this proof verify.

```solidity
interface IZkolangVerifier {
    /// Verify a proof that program `programCommit` on `publicInputs` produced
    /// `publicOutputs`. Returns true only if the STARK accepts.
    function verify(
        bytes32 programCommit,   // canonical hash of the compiled Op list
        uint256[] calldata publicInputs,   // Fp values, canonical little-endian
        uint256[] calldata publicOutputs,  // Fp values
        bytes calldata proof     // the succinct (recursed) proof
    ) external view returns (bool);
}
```

A full zKølang STARK is too large to verify directly in EVM gas. The verifier
checks a succinct proof produced by the recursive verifier already in the tree
(`nonos-stark`, the recursion assembly path), which compresses the STARK to a
constant-size object a Solidity verifier can check. The recursion binds the
program commitment and the public inputs and outputs into its own public inputs,
so the on-chain check is over exactly the statement the buyer paid for. See gap 1.

### 2. ProvingMarket

The escrow and matching contract. Holds the buyer's fee and the prover's bond,
calls the verifier, and settles.

```solidity
interface IProvingMarket {
    event JobPosted(uint256 indexed jobId, address indexed buyer,
                    bytes32 programCommit, uint256 maxFee, uint64 deadline);
    event JobClaimed(uint256 indexed jobId, address indexed prover);
    event JobSettled(uint256 indexed jobId, uint256 fee, uint256 protocolCut);
    event JobReclaimed(uint256 indexed jobId);

    /// Escrow `maxFee` NOX against a statement. `publicInputs` are the committed
    /// inputs the prover must run against.
    function postJob(bytes32 programCommit, uint256[] calldata publicInputs,
                     uint256 maxFee, uint64 deadline) external returns (uint256 jobId);

    /// Claim an open job by posting the prover bond.
    function claimJob(uint256 jobId) external;

    /// Submit the outputs and proof. On a valid proof the fee is routed and the
    /// bond returned; the actual fee is `quote(trace)` capped by `maxFee`.
    function submitProof(uint256 jobId, uint256[] calldata publicOutputs,
                         uint256 fee, bytes calldata proof) external;

    /// After the deadline with no valid proof, the buyer takes the escrow back
    /// and the claimant's bond is forfeit.
    function reclaim(uint256 jobId) external;
}
```

The `fee` a prover submits must equal `quote` for the trace it proved, and the
contract enforces `fee <= maxFee`. Pricing follows work, so the buyer bounds the
cost up front and the prover cannot overcharge.

### 3. FeeRouter

Already present in the NOX contract set. It splits a settled fee into the prover
payment and the protocol cut and moves the cut to the treasury. Reuse it; do not
build a second one. The market calls it with the prover payment and the cut
computed from `quote` (the cut is `protocol_fee_micronox` in `src/nox.rs`, five
percent today).

### 4. NOX ERC20

Already deployed. The payment and bond token. No change.

### 5. ProverRegistry (optional, phase two)

A registry of provers with staked NOX and a reputation score, so buyers can
target reliable provers and the market can prioritize claims. Not required for a
first launch, because a sound verifier makes a fraudulent proof impossible; the
registry is a liveness and quality-of-service layer, not a safety one.

## Binding the statement

The whole security of the market rests on the on-chain statement matching the
off-chain run. Three commitments carry it:

- The program commitment is a canonical hash of the compiled `Op` list (a stable
  serialization of the instruction stream). The buyer and the verifier agree on it,
  so the proof is about one exact program.
- The public inputs are supplied by the buyer at `postJob` and are the same values
  the AIR boundary pins in `src/air.rs`. The prover must run against them.
- The public outputs are submitted with the proof and are pinned by the AIR
  boundary too, so a prover cannot claim an output the run did not produce.

The verifier reconstructs the statement from the program commitment and the public
values, exactly as the in-process verifier reconstructs the AIR, so the on-chain
check has the same meaning as the local one.

## The real-world gaps to close before launch

These are the honest blockers. None is hidden.

1. On-chain verification. The full STARK does not fit in EVM gas. The launch path
   is the recursive verifier compressing a zKølang proof to a succinct proof the
   Solidity verifier checks. The recursion and a Solidity verifier exist in the
   tree for the custody use case; the work is to route a zKølang proof through the
   same recursion and to expose the program commitment and public inputs and
   outputs as the recursion's public inputs. Until this lands, verification is
   in-process only and settlement cannot be trustless on chain.

2. The public-input verifier seam. The prover has a public-binding path
   (`stark_prove_poseidon_ext_pub`) but its verifier counterpart is not yet in
   `nonos-stark`. The recursion must bind the same publics, so this seam must be
   completed and its transcript order frozen before proofs are portable to chain.

3. Canonical program serialization. A stable, versioned byte encoding of the `Op`
   list and a fixed hash, so the program commitment is reproducible across a
   compiler and across time.

4. Fee agreement. The contract prices from the trace the prover proved, but it
   cannot see the trace, only the proof. Either the trace length is exposed as a
   verified public input of the recursion (so the fee is checkable on chain), or
   the buyer's `maxFee` is treated as the price. The first is the honest design and
   needs the trace length added to the recursion's public inputs.

5. Market hygiene. Job-claim griefing, proof-submission races, and deadline
   selection are ordinary market-design problems that the contract must handle
   (single-claim locks, a claim timeout distinct from the proof deadline). These
   are contract-level, not proof-level.

6. Private inputs. Every input is public today, so the market proves statements
   about public data. Confidential proving (a hidden witness) is a later language
   feature and a later contract mode; do not design the first launch around it.

## The launchable slice

The smallest honest launch is: the recursion routes zKølang proofs (gap 1 and 2),
the program commitment and public values are the recursion's public inputs (gap 3
and 4), and the `ProvingMarket` escrows NOX and settles through the existing
`FeeRouter`. That is a real, useful, revenue-generating utility: anyone can pay
NOX to have a public computation proven and settled trustlessly, and the protocol
earns on every proof. The optional registry and confidential proving come after.
