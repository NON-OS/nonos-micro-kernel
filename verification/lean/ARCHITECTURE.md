# NONOS Lean proof architecture

The `verification/lean` tree is the specification and proof layer of the NONOS
microkernel. It is a Lean 4 development, built with `lake`, that states the
kernel's security properties as theorems and proves them by machine. A clean
`lake build` means every theorem in the tree has been checked by Lean's kernel.
Nothing is left as `sorry`, and the axiom closure of every flagship theorem is
printed as evidence, so a reader can see exactly what each proof rests on.

This document describes how the layer is put together, what it proves, and what
it deliberately does not. The style follows the seL4 / l4v tradition: an
abstract specification, a model close to the running machine, and refinement
between them, with the trusted computing base named rather than hidden.


## The stack

```
        NONOS kernel            Rust, x86_64, bare metal, ~243k LOC
             |
             |   Aeneas          Rust -> Lean, for the capability core
             v
   +-------------------------+
   |  Lean specification     |   verification/lean/Nonos/*.lean
   |  state machines +       |   pure Nat / Prop, no mathlib
   |  predicates + operations|
   +-------------------------+
             |
             |   lake build      every theorem checked by Lean's kernel
             v
   +-------------------------+
   |  theorems               |   400+, zero sorry
   +-------------------------+
             |
             |   #print axioms   AxiomProfile.lean
             v
     propext    Classical.choice    Quot.sound
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     the only foundation any proof is allowed to use
```

The chain runs top to bottom. The kernel is Rust on real silicon. Aeneas
translates the capability code from Rust into Lean so the proofs can talk about
the real implementation, not a paraphrase of it. The specification layer models
each subsystem as a small state machine over `Nat` and `Prop`. The theorems are
the security properties, proved against those models. `#print axioms` reports
the exact set of axioms each proof depends on, and that set is never anything
but Lean's own three.


## Layers, in the l4v sense

```
   abstract spec        Nonos/Capability.lean, Authorization.lean, ...
        |                a capability is a membership function over ids;
        |                attenuate is meet; grant is join; ...
        v
   word-level model     Nonos/CapabilityBits.lean
        |                the same lattice, denoted onto the u64 the kernel
        |                executes: & is attenuate, | is grant, & !bit is revoke
        v
   extraction           verification/extraction/lean  (Aeneas)
        |                Caps.lean is generated from the Rust; Refinement.lean
        |                and PolicyRefinement.lean tie it to the abstract spec
        v
   implementation       src/ (Rust) and the *_proofs differential harnesses
```

The point of the middle layer is that the capability theorems are not proved
about a convenient abstraction and then hand waved onto the code. `attenuate`,
`grant` and `revoke` are proved to denote exactly the bitwise `&`, `|` and
`& !bit` on a 64 bit word, which is what the kernel runs. `word_chain_never_widens`
is the abstract non amplification theorem landed on that word.


## Guarantee families

The theorems group into families. Each family is a property the kernel must
hold, stated once as a named theorem and proved over the operation that
implements it.

```
   authority        no attenuation chain ever widens a capability, at the u64
                    Authorization, Capability, CapabilityBits, CapTable, Dispatch

   attestation      no unattested capsule ever runs, whatever the spawn order
                    Spawn, Assurance

   freshness        a booted version rejects every older one; nonces never
                    repeat; the RNG state never regresses
                    AntiRollback, Nonce, Rng, Assurance

   conservation     no page or resource is created or destroyed by any trace
                    MemGrant, Quota, Ring, Stark.Pool, Stark.FeeRouter

   isolation        disjoint regions share no address; DMA and MMIO stay inside
                    their grant; no W and X page survives any map trace
                    Interval, Iommu, Mmio, PageTable, Isolation, Paging

   spatial safety   no out of bounds access; no use after free or double free;
                    a freed page carries no residual secret
                    Bounds, Heap, Zeroize

   liveness order   round robin drops no task; time is monotone; priority is a
                    strict total order; a bounded ring never overruns
                    Scheduler, Timer, Priority, Ring, Tlb

   post quantum     capsule admission binds a classical and a post quantum
                    signature; a total break of Ed25519 alone cannot authorize
                    Assurance
```

The flagship of the set is `Assurance.only_ok_capsules_run`: over the exact
gated admission function, whatever order an attacker requests spawns in, every
capsule that runs is attested, rollback fresh, and post quantum authorized.


## Axiom discipline

Lean prints the axiom closure of a theorem with `#print axioms`. `AxiomProfile.lean`
runs it on every flagship theorem in the tree. The output names, for each one,
the axioms the proof depends on. The whole file is expected to name at most:

```
   propext              propositional extensionality
   Classical.choice     the axiom of choice
   Quot.sound           soundness of quotients
```

These are Lean's three standard axioms. Many theorems here depend on fewer, and
some on none. What never appears is `sorryAx`, the marker Lean inserts wherever
a `sorry` stands in for a missing proof. The continuous integration job greps
the profile output for `sorryAx` and fails the build if it finds one, so an
incomplete proof cannot land.

No cryptographic assumption is taken as an axiom in this tree. The STARK and FRI
soundness properties that a proof carrying design would ordinarily assume are
themselves proved here, from the standard axioms, in `Nonos/Stark/`.


## How it is checked

```
   .github/workflows/lean.yml
        |
        |  1. elan reads lean-toolchain, pins the Lean version
        |  2. lake build              0 errors iff every theorem checks
        |  3. lake env lean AxiomProfile.lean > axioms.txt
        |        grep sorryAx -> fail
        |  4. inventory + manifest    theorem counts, per module listing
        |  5. upload-artifact         lean-evidence/  (axioms, inventory, manifest)
        v
   a green run is a record: this commit, this toolchain, these theorems,
   on these axioms, with no sorry
```

The evidence is not a claim in a README. It is the output of Lean's own kernel,
captured per commit and kept as a build artifact.


## Reproducing it

```
   cd verification/lean
   lake build                       # every theorem, 0 errors
   lake env lean AxiomProfile.lean  # the axiom closure of each flagship theorem
```

The toolchain is pinned in `lean-toolchain`. The development uses only Lean's
core library, no mathlib, so any machine with the pinned Lean reproduces the
result without fetching a proof library.


## Scope, stated plainly

This is a suite of machine checked security properties over the trusted path of
a real bare metal kernel. It is not a full functional correctness proof of the
entire kernel in the seL4 sense, and it does not claim to be. A 243k line
kernel with real drivers, paging and interrupts is far larger than the ~10k
lines seL4 verified, and full functional correctness at that scale is a
multi year program on its own.

What this layer does provide is a growing body of properties, each proved by
machine on a minimal axiom base, over models tied to the code the kernel runs,
with the cryptographic soundness that a proof carrying system usually assumes
proved rather than assumed. The theorem count and the per module inventory are
published by the CI job on every commit.
