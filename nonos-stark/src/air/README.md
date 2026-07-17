# The AIR engine and the monolithic FRI verifier

This module is the algebraic intermediate representation (AIR) a STARK proves,
and the machinery built on top of it that fuses a whole FRI verification into a
single constant-size proof.

## The AIR trait

A computation is a trace of `trace_width` columns. An `Air` (`spec.rs`) exposes:

- `log_trace_len`, `trace_width`, `window_size` - the trace shape,
- `constraint_degree` - the highest polynomial degree among the transition
  constraints, in the interpolated columns; the engine sizes the evaluation
  domain and the low-degree bound from it,
- `periodic_columns` - public per-row values (round constants, selectors),
- `transition(window, periodic)` - constraints that must vanish on every trace
  row but the last `window_size - 1`,
- `boundary` - `(column, row, value)` pins.

`prove.rs` / `verify.rs` are the DEEP STARK prover and verifier, generic over
any `Air`; `composition.rs` turns the constraints into one composition
polynomial and sizes the domain. One engine proves any AIR.

## Why a monolith

A FRI proof is checked by re-deriving the fold challenges from the transcript,
opening two values per query per layer under committed roots, folding those
openings with the challenges, and checking the final layer is the constant the
folds land on. Done as separate proofs, the verifier's work is
`O(queries x layers)`. The monolith fuses all of it into one trace so the
verification is a single STARK whose cost does not grow with the number of
queries or layers.

It is assembled from four pieces:

| File | Role |
|---|---|
| `fused.rs` (`Fused`) | stack heterogeneous-width AIRs into one trace, a per-row selector activating one region's transitions at a time, verified as one STARK |
| `wired.rs` (`Wired`) | `Fused` plus a grand-product column running a Plonk copy constraint over chosen columns: a public wiring forces cells in different regions to hold equal values |
| `trace_fold.rs` (`TraceFold`) | a FRI fold whose folding challenge is witnessed in trace column zero, so the wiring can bind it |
| `multi_membership.rs` (`opened_cells`) | the `(row, column)` of each opening's committed scalar, so the wiring can bind it |

The lookup argument (`lookup.rs`, `TupleLookup`) range-checks the query indices.

## Trace layout

`Wired` stacks the regions in the first half of the trace and runs the grand
product over the region span, pinned to one at the midpoint checkpoint. For the
whole-proof monolith over two queries (each an opening plus a fold):

```
              col 0      col 1  col 2   ...        col W  (= z, the running product)
 row
   0  open_0  leaf[0]    ...                        z
  ..                (Merkle-path region, width WIDTH)
  16  fold_0  beta_0     a_0    b_0                  z
  17          beta_1     a_1    b_1                  z
  ..                (in-circuit fold, width 3)
  24  open_1  leaf[0]    ...                        z
  40  fold_1  beta_0     a_0    b_0                  z
  ..
  47                (regions end; rows 48..63 padding, product carried)
  64  ...     0          0      0                    1   <- checkpoint z = 1 at span
  ..                (inert tail to 2*span, final row free)
```

The region rows are rounded up to a power of two (the `span`); the grand product
accumulates over the span and is pinned to one at row `span`, so a broken cycle
leaves it not equal to one there.

Each wired cell is numbered `row * k + j` for the `j`-th wired column
(`wired_cols = [0, 1]` here, `k = 2`), and `sigma` permutes those numbers; cells
in one cycle are forced equal. The wirings are:

- per query, opening to fold: `open_q.leaf[0]` (col 0) is wired to `fold_q.a`
  (col 1), so the fold folds exactly the value the opening committed;
- across queries, the shared challenge: `fold_q.beta[m]` (col 0) is wired, for
  every layer `m`, into one cycle over all queries, so every query folds on the
  same challenge set.

`stark_prove` then produces one proof; `stark_verify` checks it. A fold that
runs on a value its opening did not commit, or on a challenge set another query
did not share, breaks a cycle: the running product no longer closes to one at
the checkpoint, and the proof is rejected.

## What is proven, and what is wired

The monolith proves, in one STARK, for every query at once:

- **openings are committed** - each opened value verifies as a Poseidon Merkle
  path to the committed root (the `MultiMembership` region),
- **folds are consistent** - each `v = (a + b)/2 + beta * (a - b)/(2x)` holds and
  reaches the committed final layer (the `TraceFold` region),
- **the fold used the opened value** - `open_q.leaf[0] = fold_q.a`, by wiring,
- **all queries share one challenge set** - `fold_q.beta[m]` equal across `q`, by
  wiring.

The honesty boundary worth stating plainly:

1. The shared challenge set is currently **wired-equal** across queries, not yet
   **proven-derived** from the transcript. The monolith proves every query used
   *one* set; it does not yet prove that set is the Fiat-Shamir squeeze of the
   committed roots. Closing this is a single region substitution: source the
   betas from the in-circuit transcript region (`FriTranscript`, whose
   `beta_rows()` already places each squeezed challenge in column zero) instead
   of from a common witness, and wire `beta_rows() -> fold.beta`. The wiring
   machinery is unchanged; only the source region changes.

2. As with any STARK here, Fiat-Shamir is sound in the random-oracle model
   (BLAKE3 / Poseidon as the oracle), the FRI soundness bound is assumed rather
   than proven, and the rate `1/2` with the query count sets the security margin.
   See `../README.md` for that boundary in full.

## Reproduce

The host proofs run the whole pipeline on real code, honest and dishonest:

```sh
cd userland/stark_proofs
cargo test --release       # the fused, wired, fold, and monolith proofs
```

The monolith checks are `the_whole_fri_verification_is_one_stark` and its two
rejection cases (`the_monolith_rejects_an_uncommitted_fold`,
`the_monolith_rejects_a_split_challenge_set`), plus the per-query verifier and
the multi-query fan-out that build up to them.
