# stark_proofs

Host-runnable proofs for the in-kernel STARK primitives. The real
`src/crypto/stark` source is included through `#[path]` and run on the host,
so the checks are about the code that ships.

## Poseidon-Goldilocks

The algebraic hash is a real Poseidon over the Goldilocks field, not a
demonstration instantiation: width 12, the `x^7` S-box, eight full rounds and
twenty-two partial rounds, with the published Plonky2 round constants and the
`circ + diag` MDS matrix. The permutation is checked against the four
reference test vectors from the Plonky2 suite (all zeros, the `0..12` range,
all `-1`, and a random state), which were themselves produced by the
hadeshash reference implementation. A single wrong constant, a wrong MDS
entry, or a wrong round split moves at least one output lane, so matching all
four byte for byte pins the parameters to the real set.

The sponge built on the permutation (rate 8, capacity 4) is checked for
determinism, for folding the input length into the capacity so a short input
and its zero-extension do not collide, and for depending on every input lane.
The two-to-one compression used by a Merkle tree is checked to use both
inputs and to be order sensitive.

## The Poseidon preimage STARK

The end-to-end proof of the stack is a STARK for knowledge of a Poseidon
preimage under the real permutation, not a stand-in. The trace is the
permutation state, one row per round; each transition enforces one genuine
Poseidon round, `next = MDS(sbox(state + rc))`, with the round constants and
the full-versus-partial S-box supplied as public periodic columns so the
constraint is the published permutation. The boundary pins the input capacity
lanes to zero (the sponge initialization) and the output rate lanes to a
public digest. The proofs check that an honest preimage verifies, and that a
wrong digest, a corrupted interior round, and a nonzero capacity seed are each
rejected. The verifier only reads the proof and never panics, so a forged
proof is refused rather than trusted.

Periodic columns are the framework mechanism that makes this expressible: a
public column, known to prover and verifier, interpolated over the trace
domain and evaluated at the same point as the trace, so a round-dependent
constraint needs no per-row index and stays low degree.

## Field, polynomial, Merkle, FRI, AIR

The Goldilocks field arithmetic, the Lagrange evaluation and low-degree
extension, the BLAKE3 Merkle commitment, the FRI low-degree test over a
Fiat-Shamir transcript, and the generic AIR prover and verifier are each
checked against their specifications, including that a tampered proof is
rejected. The engine is generic over the AIR, so one prover and verifier
handle the squaring, Fibonacci, S-box chain, two-element permutation, and
Poseidon preimage instances alike.

## Run

```sh
cd userland/stark_proofs
cargo test --release
```
