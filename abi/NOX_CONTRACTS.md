# NOX mainnet contracts, the definitive brief

This is the handoff document for the Solidity team. It states the economic
model, the exact on-chain surface, the byte-exact interface to the off-chain
tooling in this repo, and what we do and do not claim. Treat every constant
in section 4 as a frozen interface: the Rust tooling in
`nonos-bootloader/tools/nonos-attestation-circuit` produces these bytes today.

## 1. The model in one paragraph

NOX is a closed loop, not a faucet. Contributors do verifiable security work
for NONOS (ceremony rounds, fleet verification runs, boot witnesses, circuit
and capsule audits, capsule builds) and receive a canonical off-chain receipt
proving the work. Accepted receipts are aggregated into an epoch Merkle root
and paid from reward pools. The pools are funded only by demand-side revenue:
capsule publish fees, marketplace cut, attestation-feed subscriptions and
slashed publisher bonds. There is no minting and no passive yield. Usage of
NONOS finances the people who secure NONOS.

## 2. Hard rules

- NOX already exists as an ERC20 UUPS token. Reuse its address. Never
  redeploy it or change its tokenomics.
- No minting anywhere in this system. Pools pay only what they were funded.
- v1 verification is off-chain with open tooling. Contracts store hashes and
  settle from authorizer-signed Merkle roots or EIP-712 authorizations. Do
  not claim on-chain ZK verification in v1. (v2 may add optional BLS12-381
  pairing checks via the EIP-2537 precompile; design for it, do not build it.)
- New infra is non-upgradeable by default. UUPS only where explicitly
  justified, and then with storage gaps, initializer guards and a
  storage-layout regression test.
- Solidity ^0.8.24, OpenZeppelin v5 primitives, custom errors, full NatSpec,
  events on every privileged or state-changing action, no tx.origin, no
  unbounded loops on claim paths.

## 3. Contract set

Earn side:

1. NoxZkCircuitRegistry. Circuit metadata, maintainer-gated.
   Record: circuitId (bytes32), sourceTreeHash, cargoLockHash,
   publicInputLayoutHash, verifyingKeyHash, ceremonyTranscriptHash,
   policyEpoch (uint64), uri, status (Active or Deprecated).
   register and deprecate, MAINTAINER only. No deletion.

2. NoxContributionRegistry. Immutable accepted receipts.
   Record: receiptId (bytes32), circuitId, contributor (address),
   contributionType (enum below), evidenceHash, artifactHash, verifierHash,
   epoch (uint64), submittedAt, uri.
   Reverts on duplicate receiptId or unknown circuitId. AUTHORIZER submits.
   Nothing is ever mutated or removed.

3. NoxRewardRootManager. Epoch reward roots.
   publishRoot(epoch, poolId, root) by AUTHORIZER; a root may be replaced
   only before claims open; finalize(epoch, poolId) freezes it forever.
   Events for publish, replace, finalize.

4. NoxRewardPool. Pays existing NOX against finalized roots.
   Funded only by NOX transfer in. Claims by Merkle proof or EIP-712
   authorization. Each claim binds contributor, receiptId, circuitId,
   amount, epoch, poolId. Double-claim prevention by (epoch, poolId, leaf).
   Pausable, ReentrancyGuard, SafeERC20. Invariant: totalClaimed never
   exceeds totalFunded. NOX leaves only via claims or governance emergency
   path; recovery of non-NOX tokens is allowed.

Demand side:

5. NoxCapsuleRegistry. Publisher registers capsuleHash, policyRoot, vkHash
   and identity. Fee in NOX routed to NoxTreasury.

6. NoxPublisherBond. Publishers bond NOX to publish; provers and verifiers
   stake to be claim-eligible. SLASHER (timelock) slashes to NoxTreasury.
   Events on bond, unbond, slash.

7. NoxMarketplaceSettlement. Capsule purchase in NOX with a builder and
   treasury split.

8. NoxTreasury. Fee router. Receives publish fees, marketplace cut and
   slashed bonds, and routes them into NoxRewardPool. This contract closes
   the loop; its accounting must make the demand-to-earn flow auditable
   from events alone.

Roles: DEFAULT_ADMIN is a timelock or multisig, plus MAINTAINER, AUTHORIZER,
PAUSER, SLASHER. AccessControl everywhere.

## 4. Frozen interface to the off-chain tooling

The Rust tools in this repo are the source of truth for every identifier.
Do not invent alternatives; consume these.

Contribution types (enum order matters, keep it):

    CEREMONY_ROUND        trusted-setup contribution, from the transcript
    FLEET_VERIFICATION    full-fleet pairing-check run (57/57 report)
    RUNTIME_BOOT          QEMU boot log with enforced [ZK-ATTEST] ok lines
    HARDWARE_BOOT         same evidence from real hardware
    CIRCUIT_AUDIT         written audit of the attestation circuit
    CAPSULE_AUDIT         written audit of a capsule
    CAPSULE_BUILD         reproducible capsule build with a verified trailer

Identifier derivations (blake3 derive-key, 32-byte outputs):

    circuit_id  = blake3_derive_key("NONOS:NOX:ZK:CIRCUIT:v1",
                    circuit_name || vk_sha256 || transcript_sha256
                    || be64(policy_epoch))
    evidence    = ceremony: blake3_derive_key("NONOS:NOX:ZK:RECORD:v1",
                    contribution_record_json)
                  work: blake3_derive_key("NONOS:NOX:ZK:EVIDENCE:v1",
                    kind_string || artifact_bytes)
    receipt_id  = ceremony: blake3_derive_key("NONOS:NOX:ZK:RECEIPT:v1",
                    circuit_id || evidence || lowercase_address || be32(round))
                  work: blake3_derive_key("NONOS:NOX:ZK:RECEIPT:v1",
                    circuit_id || evidence || lowercase_address
                    || kind_string || be64(epoch))
    verifier_hash = blake3_derive_key("NONOS:NOX:ZK:VERIFIER:v1",
                    receipt_id || evidence || "PASS")

Claim leaf (must match nox-merkle byte for byte):

    leaf = keccak256(keccak256(abi.encode(
        address contributor,
        bytes32 receiptId,
        bytes32 circuitId,
        uint256 amount,
        uint256 epoch,
        bytes32 poolId)))

Proofs are OpenZeppelin sorted-pair keccak256 (MerkleProof.verify
compatible). The double keccak is the OZ standard-tree leaf convention.

Receipt schemas: abi/nox_zk_contribution_receipt.schema.json (ceremony) and
abi/nox_zk_work_receipt.schema.json (all other kinds). chain_id is 1.

## 5. Tests the suite must contain (Foundry)

- circuit registry: register, duplicate rejected, deprecate, unknown access
  rejected per role.
- contribution registry: submit, duplicate receiptId rejected, unknown
  circuitId rejected, immutability.
- root manager: publish, replace before open, finalize, mutate after
  finalize rejected.
- pool: fund, successful Merkle claim from a claims.json produced by this
  repo's nox-merkle tool, double-claim rejected, wrong proof rejected,
  wrong signer rejected, pause blocks claims, invariant fuzz on
  totalClaimed <= totalFunded.
- bond: bond, unbond, slash by SLASHER only, slashed funds reach treasury.
- settlement and treasury: purchase splits correctly, fees route to pool,
  the full publish, bond, purchase, treasury, pool loop runs end to end.

## 6. Deliverables

Contracts, interfaces, Foundry tests, deploy scripts, example receipts
consumed from this repo's schemas, a Merkle generation script that
reproduces nox-merkle output, and a README covering: what is rewarded, how
evidence is verified off-chain, the NOX flow diagram, the admin and upgrade
model, security assumptions, and an explicit section titled "What is NOT
claimed" stating that v1 does no on-chain proof verification and that
reward pools pay only deposited funds.

## 7. Deployment metadata, not protocol constants

Groth16 over BLS12-381, 192-byte proofs, 7 public inputs. Current verifying
key fingerprint 6cd2015037ea6181 (first 16 hex chars of vk sha256). Current
live results: 59/59 fleet pairing checks, 40/40 enforced runtime
attestations against a 5-contributor external ceremony. These describe
today's artifacts; the contracts must not hardcode them.

## 8. Mainnet deployment (Ethereum, chainId 1)

Deployed, role handoff complete, every address read back from chain.
Custody: the 3/5 Safe holds DEFAULT_ADMIN and every operational role on
all eight contracts; the deployer EOA holds nothing.

    Safe (3/5)                 0x3a52ea60F61036Afbbec25F46a64485Ac4477Ccc
    NOX token (pre-existing)   0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA
    NoxZkCircuitRegistry       0x6DFfb5D99cd0dcD3B37B1C62E8eF6C4C4142b2E7
    NoxContributionRegistry    0x6d7e47c3f5ba68C3eD92FDca2D1818ef5Fd67CD4
    NoxRewardRootManager       0xfc2dC45fa4273bedF3FBfDD6C9992e21DbdcA9f0
    NoxRewardPool              0xF31540565074a12E3327145592aB6118029880fD
    NoxTreasury                0x056cD20Fb4ec1f5A4b14eD227ba1908D4340734c
    NoxCapsuleRegistry         0x05527a86Fe152839935F0BE163F304dD330BD228
    NoxPublisherBond           0x2bA3b28Fa3765Bcb568FEfB25789A31861F42E9e
    NoxMarketplaceSettlement   0xEb84F1a40512fF91978baAa43750eb8fdAA51C90

Merkle byte parity is proven both directions: the contracts repo settles
claims generated by its JS tool against the pool logic, and this repo's
nox-merkle produces the identical root for the same claims. Before any
NOX is funded the Safe runs the contracts-repo handoff checklist
(fee exemptions, unbonding period vs slasher delay, parameters, fund and
route, first epoch root) and registers the canonical circuit entry from
target/nox/circuit.json in NoxZkCircuitRegistry.
