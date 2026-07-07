# What NØNOS proves in Lean

Lean 4 machine-checked proofs of NØNOS's security theorems. Every property below
is proven in the Lean kernel with **no `sorry`, no `admit`, no added axioms**: a
green `lake build` means all 89 theorems typecheck, which in Lean is the same as
saying they are proven.

These are the top of a refinement chain. Lean proves the property about a
mathematical model; the code-level proofs elsewhere in the repository discharge
that same property on the implementation that actually runs, so the model is
connected to the code rather than standing apart from it.

```
Lean (this directory)      abstract security theorem        "the model is safe"
        |  refined onto
        v
Verus / Kani / extraction  the real Rust operations         "the code matches the model"
        |  rests on
        v
Runnable KATs / fuzz       the real primitives              "the primitives are correct"
```

## The whole-system theorem

`Secure` composes the per-operation lemmas into a single invariant over a
transition system whose steps are the security-relevant kernel operations:
capability attenuation, transfer and revocation, page mapping, user copies, and
boot. It conjoins three guarantees and proves they survive **every trace of
every length**, so no hostile sequence of operations can escape them:

- **Authority conservation** - no step manufactures a right nobody held.
- **Memory isolation** - no installed mapping is writable and executable, and
  every accepted user copy lies inside user space.
- **Anti-rollback** - the version floor never decreases.

```
theorem every_trace_is_secure (s0 : State) (tr : List Syscall)
    (h : Secure s0) : Secure (run s0 tr)
```

## The properties

| Area | What is proven (plainly) | Key theorem | Discharged on the code by |
|---|---|---|---|
| Capabilities | A right cannot be forged; attenuation only narrows authority; a revoked right cannot survive a prior grant | `attenuate_confines`, `grant_then_revoke_drops` | `verus/src/capabilities.rs` |
| Capability bits | The kernel's 64-bit `&`, `\|`, `& !bit` denote exactly the abstract grant/revoke/attenuate; a word chain never widens | `land_not_denotes_revoke`, `word_chain_never_widens` | `kernel_proofs` over `capabilities/bits.rs` |
| Authorization | No syscall is allowed without holding its required capability; the empty token is denied everything | `empty_token_denied`, `allowed_iff_grants` | `kernel_proofs` (`is_allowed`) |
| Anti-rollback | The version floor never lowers; once a version boots, every older version is rejected forever | `no_rollback_after_boot`, `update_never_lowers_floor` | `nonos-bootloader/boot_proofs` (Kani) |
| Isolation (W^X) | A page can never be writable and executable at once; an accepted user copy stays inside user space | `no_wx_page`, `accepted_stays_in_user_space` | `kernel_proofs` (`to_pte_flags`, `check_range`) |
| Paging lattice | A page mapping can only be narrowed under refinement, never widened; a non-W+X mapping stays non-W+X | `subset_trans`, `confined_preserves_no_wx` | `verus/src/page_permissions.rs` |
| Path safety | A path can never escape above its root through `..`, no matter how many | `all_up_stays_at_root`, `leading_dotdot_neutralized` | `fs_proofs` (Kani + runnable) |
| Crypto gate | A MAC/AEAD/signature check accepts exactly the genuine tag; a wrong or truncated tag is rejected | `wrong_tag_rejected`, `length_mismatch_rejected` | `crypto_proofs` `ct_eq` + KATs (#265) |
| IPC bounds | A zero-length or oversized message is rejected; the length bound is exact | `zero_length_rejected`, `boundary_is_exact` | Verus `ipc_lengths.rs` |
| Attestation | The kernel stamps a message's sender from the true caller; a capsule cannot impersonate another | `no_impersonation`, `sender_independent_of_claim` | `fs_proofs` (caller attestation) |
| ZeroState | A capsule's memory is wiped on teardown, so no secret survives to the next tenant of that memory | `no_secret_survives`, `reused_region_leaks_nothing` | kernel zeroization on teardown / page reclaim |
| Whole system | The three guarantees above hold together after every trace of every length | `every_trace_is_secure` | the per-step obligations above + `verification/extraction` |

## The proofs themselves

The `.lean` files in `Nonos/` are the proofs; there is nothing behind them. Each
file is a definition of the model followed by the theorems, in core Lean (no
mathlib), so any recent `leanprover/lean4` toolchain checks them.

| File | Lines | Theorems |
|---|---:|---:|
| `Nonos/Secure.lean` | 209 | 4 |
| `Nonos/Capability.lean` | 208 | 25 |
| `Nonos/AntiRollback.lean` | 129 | 9 |
| `Nonos/CapabilityBits.lean` | 105 | 5 |
| `Nonos/Paging.lean` | 98 | 8 |
| `Nonos/Authorization.lean` | 82 | 9 |
| `Nonos/Path.lean` | 70 | 6 |
| `Nonos/Isolation.lean` | 62 | 3 |
| `Nonos/Attestation.lean` | 61 | 5 |
| `Nonos/Zeroization.lean` | 53 | 6 |
| `Nonos/Ipc.lean` | 48 | 5 |
| `Nonos/Crypto.lean` | 46 | 4 |
| **Total** | **1171** | **89** |

## Reproduce

```sh
cd verification/lean
lake build          # 0 errors == all 89 theorems verified
```

The `lean` CI job runs this on every push. To confirm there are no escape
hatches:

```sh
grep -rE 'sorry|admit' Nonos.lean Nonos/    # no matches (the word appears only in prose)
```
