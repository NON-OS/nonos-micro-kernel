---
title: "Machine-Checked Security Verification of the NØNOS Microkernel"
subtitle: "A Layered Refinement Architecture from Lean Specifications to Mechanically-Extracted MIR, with a Transparent Post-Quantum Attestation Gate"
author: "NØNOS Contributors"
date: "2026"
abstract: |
  We present the verification architecture of NØNOS, a capability-based,
  RAM-resident microkernel written in Rust. Rather than claim total functional
  correctness, NØNOS proves a focused set of security-critical properties and,
  crucially, proves them over the code that actually runs. The architecture is
  layered by the strength of the proof-to-code link: a source-hygiene floor;
  runnable proofs that execute the real kernel source through Rust's path
  inclusion; bounded model checking with Kani; a specification corpus of 887
  machine-checked Lean theorems across 120 modules with zero `sorry`; a Verus
  refinement over the real Rust bit-operations; and, strongest, a mechanical
  extraction in which real kernel functions are lowered from the MIR that the
  compiler produces (via Charon) into pure Lean definitions (via Aeneas), with
  theorems proven directly on those extracted definitions. Every flagship
  theorem's axiom closure is machine-audited to contain only the three standard
  axioms of Lean and never the placeholder that a `sorry` would introduce. A
  transparent, post-quantum STARK gate re-verifies each capsule's attestation at
  spawn and the kernel's own measurement at boot; the whole verification surface
  is summarized in a reproducible, CI-checked evidence manifest. We give the
  formal statements of the load-bearing theorems, the semantics of the
  extraction pipeline, and an honest account of what is and is not established.
---

# Introduction

A proof is only as strong as the distance between the object proved and the
object executed. A system that carries a formal model and proves theorems about
that model has established something real about the model; whether the running
implementation *refines* the model is a separate question, and for most systems
that advertise "formal verification" it is a question left unanswered. That
unproven gap is precisely where defects survive.

The gold standard for closing the gap totally is seL4 [@klein2009sel4], which
proves full functional correctness of a minimal C kernel in Isabelle/HOL and
proves that the compiled C refines the abstract specification. NØNOS makes a
deliberately different trade. It does not attempt total correctness of a large
system. Instead it (i) fixes a set of *security-critical* properties, (ii)
proves them over the *real source*, and (iii) builds on a memory-safe language,
eliminating by construction the class of spatial and temporal memory-corruption
defects that dominate a C kernel's proof obligations. The contribution of this
paper is the *architecture* by which the second point is achieved: a spectrum of
proof techniques ordered by the tightness of their link to the running code,
culminating in mechanical extraction of the executable's own intermediate
representation.

All quantitative claims in this document are machine-derived from the source
tree and continuously checked in CI against a committed evidence manifest
(§7); at the time of writing the Lean corpus comprises **887 theorems across
120 modules with zero `sorry`**, of which **168** are axiom-profiled; **7**
kernel functions are mechanically extracted from real MIR; **5** Verus source
files, **65** Kani harnesses, and **32** runnable proof crates exercise the real
code.

# Preliminaries and Notation

We write $\mathbb{B} = \{\bot, \top\}$ for the Booleans and $\mathbb{N}$ for the
naturals. A machine word of width $w$ is an element of $\mathbb{Z}_{2^w}$; we use
$w = 64$ for addresses and treat words as elements of $\mathbb{N}$ bounded by
$2^{w}$, making overflow explicit. For $x \in \mathbb{N}$ and bit index
$i \in \mathbb{N}$, $\mathrm{testBit}(x, i) \in \mathbb{B}$ denotes the $i$-th
bit of the binary expansion of $x$; bitwise disjunction, conjunction and the
two-power are written $\lor$, $\land$ and $2^{i}$ in the usual way, lifted to
words. Page granularity is $P = 4096$; the page base of an address is
$\lfloor x \rfloor_P = x - (x \bmod P)$.

The Lean development is *core-only*: it depends on no external mathematical
library, so any conforming Lean 4 kernel checks it and no theorem can rest on an
unintended lemma. A Lean theorem $T$ has an **axiom closure** $\mathrm{Ax}(T)$,
the set of axioms its proof term ultimately depends on, computable by the kernel
itself. We write $\mathcal{S} = \{\mathsf{propext}, \mathsf{Classical.choice},
\mathsf{Quot.sound}\}$ for the three standard axioms of Lean's logic. A `sorry`
anywhere in a proof introduces the distinguished axiom $\mathsf{sorryAx}$ into
the closure.

**Definition 1 (Soundness policy).** A theorem $T$ is *admissible* iff
$\mathrm{Ax}(T) \subseteq \mathcal{S}$. In particular $\mathsf{sorryAx} \notin
\mathrm{Ax}(T)$, so every admissible theorem is fully proven.

The build gate `AxiomProfile.lean` enumerates the flagship theorems and prints
each $\mathrm{Ax}(T)$; CI rejects any run in which admissibility fails.

# The Verification Architecture

The layers below are ordered by the strength of the proof-to-code link, weakest
first. Each is re-checked on every push.

**Layer 0 (hygiene).** A static gate scans all production Rust under `src/` and
`userland/` and rejects panic paths (`unwrap`, `expect`, `panic!`), stub macros,
and dead-code markers. This is not a correctness proof; it is a machine-enforced
invariant that the production surface contains no panic path and no stubbed
logic.

**Layer 1 (runnable proofs over real source).** Host crates include the actual
kernel and capsule source unmodified through Rust's `#[path]` mechanism and
execute it under `cargo test`, asserting concrete guarantees: VFS store and
path-canonicalization invariants, protocol-codec robustness against hostile
input, caller-attestation rejecting impersonation, and differential and random
fuzz proofs establishing that parsers neither panic nor violate invariants over
millions of inputs. Cryptographic primitives are checked against published
known-answer vectors (FIPS 180-4, FIPS 202, RFC 8439, RFC 8032, and others),
each with tamper rejection.

**Layer 1b (bounded model checking).** Kani harnesses exhaustively check
functions over all inputs within a bound; notably, the untrusted
attestation-trailer deserializer is proven *total*, i.e. it terminates without
panic on every input up to the bound.

**Layer 2 (Lean specification).** The abstract security theorems (§4).

**Layer 2b (Verus refinement).** SMT-checked proofs over the real Rust
semantics that the bit-level implementation satisfies the Lean specification
(§5).

**Layer 2c (mechanical extraction).** Real functions lowered from compiler MIR
into Lean and proven there (§5), the tightest link.

# The Lean Specification Corpus

The corpus expresses each trusted-path invariant as an admissible theorem
(Definition 1). We state a representative selection formally; each is a theorem
in the named module, verified with $\mathrm{Ax}(T) \subseteq \mathcal{S}$.

## Capability delegation

Capabilities are single bits of an authority word. For words $x, m \in
\mathbb{N}$ define the *submask* relation $x \sqsubseteq m \iff \forall i,\;
\mathrm{testBit}(x,i) \Rightarrow \mathrm{testBit}(m,i)$, and grant
$\mathrm{add}(b, c) = b \lor 2^{c}$.

**Theorem 2 (Delegation confinement; `CapMask.subset_no_extra`).**
For all masks $\mathit{child}, \mathit{parent}$ and every capability index $c$,
$$
  \mathit{child} \sqsubseteq \mathit{parent} \;\wedge\;
  \neg\,\mathrm{has}(\mathit{parent}, c)
  \;\Longrightarrow\; \neg\,\mathrm{has}(\mathit{child}, c).
$$
A delegated authority never carries a capability its grantor lacks; combined
with transitivity of $\sqsubseteq$ (`subset_trans`), authority only ever narrows
along a delegation chain.

**Theorem 3 (Grant is monotone and non-aliasing; `CapMask.has_add_other`,
`add_monotone`).** For $c \neq d$, $\mathrm{has}(\mathrm{add}(b,d), c) =
\mathrm{has}(b, c)$, and $\mathrm{has}(b,d) \Rightarrow
\mathrm{has}(\mathrm{add}(b,c), d)$: granting one capability neither adds nor
removes any other, and never revokes.

## The user/kernel boundary

Let $U = \texttt{0x7FFF\_FFFF\_FFFF}$ be the canonical user ceiling and
$M = 2^{26}$ the maximum copy length. The range check $\mathrm{check}(a, \ell)$
mirrors `src/usercopy/policy.rs` and returns a page span or an error.

**Theorem 4 (Boundary confinement; `UserCopy.accepted_within_user`,
`accepted_nonzero_addr`, `accepted_aligned`).** If $\mathrm{check}(a, \ell)$
accepts with span $[s, e]$ (page bases), then $a \neq 0$, $s \equiv e \equiv 0
\pmod P$, and $e \le U$. Consequently every address the copy will touch lies
strictly inside user space: it can reach neither the null page below nor kernel
memory above.

## Anti-rollback, and a concrete refinement

Let a monotone floor $f \in \mathbb{N}$ record the minimum admissible version.

**Theorem 5 (No rollback; `AntiRollback.no_rollback_after_boot`).** Once a
version $v$ is accepted (so $v \neq 0$ and $v \ge f$) and the floor is raised to
$\max(f, v)$, every strictly smaller version $w < v$ is thereafter refused; the
floor never decreases over any trace (`floor_never_decreases_over_any_trace`).

The abstract statement is connected to the executable branch structure by a
refinement.

**Theorem 6 (Concrete refinement; `AntiRollbackState.refines_abstract`).** Let
$\mathrm{check}_{\mathrm{c}}$ be the four-branch `check_kernel_version` of the
bootloader, with its trust gating (uninitialized-and-no-TPM refuses all;
version zero refused). For an initialized state,
$$
  \mathrm{check}_{\mathrm{c}}(s, v) = \mathsf{ok}
  \;\Longleftrightarrow\;
  \mathrm{Accepts}(\pi(s), v),
$$
where $\pi$ projects the concrete state onto the abstract floor and
$\mathrm{Accepts}$ is the abstract predicate of Theorem 5. Thus the abstract
no-rollback theorem holds of the code's real control flow.

## The hardware broker

**Theorem 7 (Bounded, owned DMA; `DmaMap.accepted_*`).** An admitted DMA mapping
is page-aligned and non-empty, its device is claimed by exactly the calling
process, its claim epoch equals the request's (so a stale claim cannot be
replayed), and its page count is within the device class limit. No caller can
program a device to transfer outside a window it currently and legitimately
owns.

**Theorem 8 (MSI-X exclusion; `MsixExclusion.no_protected_byte_mapped`).** Let a
requested MMIO mapping be clamped against a protected region $[r_s, r_e)$ by
pulling its end down to $\lfloor r_s \rfloor_P$ on overlap. Then no address the
clamped mapping covers lies in $[r_s, r_e)$: a raw BAR mapping can never reach a
byte of the MSI-X table or PBA, whether the request straddled the region or
began inside it (in which case the safe length is zero).

## The loader and the W^X invariant

**Theorem 9 (Program-header bounds; `ElfPhdr.accepted_table_in_bounds`).** An
accepted non-empty program-header table satisfies $\mathit{off} + s \cdot n \le
|\mathit{image}|$ with $s$ the exact entry size and no address-type overflow, so
every one of its $n$ headers is an in-bounds read.

**Theorem 10 (Bounded relocation writes; `ElfReloc.writeSize_le_8`,
`admitted_no_oob`).** Every supported relocation type writes at most eight
bytes; an unsupported type is refused; and an admitted relocation either writes
nothing or writes wholly inside a segment.

## Resource allocation and dispatch

**Theorem 10a (Descriptor allocation; `FdAlloc.alloc_free`, `alloc_lowest`,
`alloc_none_full`).** The file-descriptor allocator returns the least free
descriptor at or above the floor: an allocated descriptor lies in the window and
was free, every descriptor from the floor up to it was in use, and the allocator
declines exactly when the whole window is occupied.

**Theorem 10b (Process identifiers; `PidAlloc.chosen_pid_ne_zero`,
`chosen_pid_inactive`).** The identifier the selector returns is never the
reserved value zero and is not already live, and the counter stored back never
wraps to zero, so two processes never share an identifier.

**Theorem 10c (Syscall dispatch; `SyscallRoute.route_unclaimed_is_enosys`,
`route_earlier_shadows`, `route_append_stable`).** Every system call has exactly
one owner, decided by the fixed order of the handler groups: an unclaimed number
is refused with the error sentinel, the first group to claim a number decides the
result, and appending a new subsystem cannot capture a number an existing one
already owns.

## The multisignature and the Wi-Fi trusted path

**Theorem 10d (Threshold authority; `MultiSig.valid_config`,
`threshold_backed_by_distinct_authorized`).** A multisignature configuration is
accepted only as a well-formed $k$-of-$n$ with $1 \le k \le n$ within the signer
cap, a signature is recorded only from an authorized signer who has not already
signed, and a met threshold is therefore backed by at least $k$ distinct
authorized signatures, never by duplicates or outsiders.

**Theorem 10e (WPA2 handshake and replay; `Wpa2Handshake.install_requires_valid_msg3`,
`CcmpReplay.no_redelivery`).** The Wi-Fi supplicant installs keys only on a
message 3 whose integrity code verifies under the derived key and whose nonce
repeats the bound authenticator nonce, and a received packet number is delivered
at most once: once processed, the same number is never fresh again, so a captured
frame replayed cannot advance the connection.

# Mechanical Extraction: From MIR to Lean

The layers of §4 state and prove properties of Lean models; the remaining gap is
the fidelity of each model to the Rust it transcribes. Layer 2c removes that
transcription step for selected functions.

## The pipeline

Let $f$ be a Rust function. The Rust compiler lowers $f$ to MIR, a control-flow
graph over typed places. **Charon** [@charon] reads the compiler's own MIR and
emits it as LLBC, a serialized, borrow-resolved low-level representation.
**Aeneas** [@aeneas] translates LLBC into a pure, total functional program in
Lean, modelling machine integers as bounded scalars in a `Result` monad that
makes overflow and failure explicit, and modelling the `?` operator as an
explicit control-flow branch. Write $\llbracket f \rrbracket$ for the resulting
Lean definition. Because the input is the compiler's MIR and not a human
transcription, $\llbracket f \rrbracket$ *is* the function, up to the (published,
audited) semantics of Charon and Aeneas.

The extraction crates under `verification/extraction/` include the real `src/`
files unmodified via `#[path]`; nothing is copied. The generated Lean is
regenerated in CI and diffed against the committed artifact, so it cannot
silently diverge from the source.

## Theorems on the extracted definitions

**Theorem 11 (Extracted revocation confinement;
`NonosExtraction.extracted_remove_confines`).** Let
$\llbracket \mathrm{remove\_capability} \rrbracket$ be the Aeneas extraction of
the kernel's real capability-clear function. For any word $b$, capability $c$
and index $x$, if the extracted function yields $r$ then
$$
  \mathrm{Grants}(\mathrm{caps}(r), x)
  \;\Longrightarrow\;
  \mathrm{Grants}(\mathrm{caps}(b), x),
$$
i.e. the *real* revoke never grants a capability the input lacked. The proof
proceeds through `remove_capability_spec`, which shows the extracted definition
computes exactly $b \land \overline{2^{c}}$ over 64-bit words, connecting the
executed bit-operation to the abstract lattice.

**Theorem 12 (Extracted boundary check; `NonosExtraction.check_range_spec`).**
The Aeneas extraction of `check_range` never fails, and accepts a non-empty copy
of length $\ell$ at address $a$ *exactly* when
$$
  a \neq 0 \;\wedge\; \ell \neq 0 \;\wedge\; \ell \le 2^{26}
  \;\wedge\; a + (\ell - 1) \le U,
$$
and the accepted result equals the abstract admission predicate
$\mathrm{Isolation.Accepts}(a, \ell)$ (`check_range_accepts_iff`). The proof
tracks the extracted control flow through the checked addition (whose overflow
case is handled explicitly) and the two-power page mask.

**Theorem 13 (Extracted W^X encoder;
`NonosExtraction.extracted_no_wx_page`).** The Aeneas extraction of the kernel's
page-permission encoder never emits a page-table entry that is simultaneously
writable and executable.

**Theorem 14 (Extracted MSI-X flag gate;
`NonosExtraction.Irq.unsupported_flags_rejected`).** The Aeneas extraction of
`validate_msix_request` refuses, with `UnsupportedFlags`, any request whose flag
word sets a bit outside the known set, before consulting any device state.

Where Aeneas models a `core`-library method as an opaque axiom (for instance
`Option::ok_or`), that axiom appears in the closure and is documented; it is a
faithful stand-in for the standard method, not a `sorry`, and is disjoint from
$\mathsf{sorryAx}$.

## The scalar and control-flow model

Aeneas does not translate Rust into a partial imperative language; it produces a
total functional program, and this shapes how the theorems are stated. Machine
integers become bounded scalars carrying their bit width, and every operation
that can overflow or fail returns a value in a result monad
$\mathsf{Result}\,\tau = \mathsf{ok}\,\tau \mid \mathsf{fail} \mid \mathsf{div}$.
An addition is therefore not $a + b$ but a computation that either yields
$\mathsf{ok}(a + b)$ when the sum fits the width or $\mathsf{fail}$ when it does
not, and the extracted function is a monadic bind over these steps. The checked
addition inside `check_range` appears in the extracted Lean exactly as this
branch, and Theorem 12 is proven by discharging both arms: the overflow arm
cannot yield an accepted range because it routes to an error, and the in-bounds
arm carries the arithmetic that the page mask then confines. Nothing about the
overflow behaviour is assumed; it is present in the extracted definition and
reasoned about.

The `?` operator, which Rust uses to propagate errors, becomes an explicit match
on a control-flow value with `Continue` and `Break` constructors. A proof over an
extracted function that uses `?` is therefore a proof over that match, and the
early-return semantics are visible rather than hidden. Borrows are resolved by
Charon before Aeneas sees the program, so the Lean carries no lifetime
machinery; what remains is a pure function of its inputs, which is the ideal
object for a theorem.

## A worked proof: the user and kernel boundary

The value of the extraction is clearest on a single function. The kernel copies
bytes across the user and kernel boundary only after clearing the request
through `check_range`, whose source is a sequence of guards over a `u64` address
and a `usize` length. Charon lowers it and Aeneas produces a Lean definition
whose shape is the source control flow: a null test, a size-cap test against
$2^{26}$, a zero-length short circuit, a checked addition of the address and the
length less one, and, when that addition does not overflow, a comparison of the
last byte against the user ceiling $U$ followed by two applications of the page
mask $x \mapsto x \land \overline{P - 1}$.

Theorem 12 states the admission condition of this extracted function exactly, as
a biconditional, and further identifies it with the abstract predicate
$\mathrm{Isolation.Accepts}$. The proof is a decision-tree walk that mirrors the
source: at each guard it establishes which branch the hypotheses force and
carries the arithmetic forward with a linear-arithmetic decision procedure. The
page mask is the one step that is not linear, and it is handled by observing
that $x \land \overline{P - 1} = \lfloor x \rfloor_P \le x$, so an accepted
range whose last byte is at most $U$ has a masked end also at most $U$. The
consequence, Theorem 4 restated over the extracted code, is that no address the
copy touches can escape user space. There is no model between this theorem and
the bytes the kernel moves; the theorem is about the compiler's own lowering of
the function that moves them.

## Proof methodology and the axiom audit

Three techniques recur across the corpus and are worth naming, because they are
what make the proofs both true and auditable.

**Characterization lemmas.** A validator with several guarded exits is first
proven to satisfy a single lemma that names, as a conjunction, every fact an
accepted input enjoys. The individual safety theorems are then short corollaries
that project one conjunct. `DmaMap`, `IrqBind` and `ElfPhdr` are each organized
this way, so the branch analysis is done once and the security consequences read
off it. This keeps the delicate part of each proof in one place and makes the
theorems that a reader cares about trivial to check against the lemma.

**Structural and fuel-bounded induction.** Allocators and scanners are proven by
induction. The lowest-free-descriptor allocator is modelled as a bounded upward
scan and its guarantee, that the returned descriptor is the least free one at or
above the floor, is an induction over the scan whose inductive step is the
observation that everything below the result was occupied. The syscall router is
proven by induction over the ordered list of handler groups, establishing that
the first group to claim a number decides the result and that appending a new
subsystem cannot capture a number an existing one already owns.

**Refinement.** Where an abstract theorem is the clean statement and the concrete
code carries extra structure, a refinement theorem connects them. Theorem 6 is
the archetype: the four-branch bootloader check, with its trust gating, is proven
to accept exactly what the abstract monotone-floor predicate accepts, so the
abstract no-rollback theorem lands on the real control flow. Verus (Layer 2b) and
the Aeneas extraction (Layer 2c) are the two mechanized forms of the same
discipline, one over the Rust semantics and one over the compiler's MIR.

The audit that ties these together is the axiom profile. Lean's kernel can
compute, for any theorem, the exact set of axioms its proof term depends on. The
project enumerates its flagship theorems in a single file and prints that set for
each, and the continuous-integration gate rejects any run in which a set contains
$\mathsf{sorryAx}$ or any axiom outside $\mathcal{S}$. A claimed theorem cannot,
therefore, silently rest on an unproven step: the placeholder that a `sorry`
would introduce is exactly the axiom the gate forbids. This is a mechanical
property of the proof terms, not a review convention, and it is the reason the
corpus can be trusted at the granularity of a single theorem.

# The Transparent STARK Attestation Gate

Admission to execution is governed by a proof, not by provenance. Each capsule
carries a transparent, post-quantum STARK establishing membership of its
measurement in an enrolled policy commitment; the kernel re-verifies it at every
spawn, before the process receives memory, and the bootloader verifies the
kernel's own measurement the same way before the jump.

## Structure of the proof system

The gate is a scalable transparent argument of knowledge, chosen because it needs
no trusted setup and rests only on the collision resistance of a hash. A
computation is expressed as an algebraic intermediate representation: an
execution trace laid out as a table over a finite field, together with a set of
polynomial constraints that hold on every row of a correct trace and fail
somewhere on an incorrect one. Membership of a measurement in the enrolled policy
is encoded as a fixed-depth Merkle authentication path, and the constraint system
asserts, row by row, that each step of the path hashes correctly to the next,
ending at the committed root. The prover commits to the trace and to the
low-degree extension of its constraint polynomials through Merkle trees over the
field, and proves proximity to a low-degree codeword with the FRI protocol, whose
folding rounds reduce a claim about a large codeword to a claim about a small one.
The verifier is made non-interactive by deriving its challenges from the
transcript with a Fiat-Shamir hash, so the same code produces and checks the
proof and the challenges cannot be ground against.

The measurement itself is a length-prefixed encoding of the capsule image hash,
the granted capability mask, the policy epoch and a domain separator, hashed into
the field. Length prefixing makes the encoding injective, which is what stops a
capsule from presenting a measurement that collides with another's under a
different field split; this injectivity is one of the Lean theorems. The trailer
that carries the proof through the image is parsed by untrusted code, so its
deserializer is proven total: it terminates without panic and reserves no buffer
larger than the bytes it actually holds, on every input, which closes the
denial-of-service and out-of-bounds surface a hostile trailer would otherwise
open.

## What is proven, and what is assumed

The attestation body adds 203 further Lean theorems (the `Nonos/Stark/` modules
with `SigningKey` and `KeyLifecycle`): soundness and collision-freedom of the
Merkle membership relation; injectivity of the length-prefixed measurement
encoding; binding of a proof to its capsule identity, capability mask, policy
epoch and domain; totality and bounds-safety of the untrusted trailer parser;
and the rollback, revocation and validity-window discipline of the signing keys.
These pin the model that the spawn gate refines; the runnable `stark_proofs`
crate and the Kani totality harness discharge the parser obligations on the real
code.

**Proposition 15 (Admission soundness, informal).** Under the soundness of the
FRI-based STARK and the collision-resistance of the measurement hash, a capsule
whose measurement is not enrolled under the policy commitment cannot produce a
trailer the gate accepts, except with negligible probability. The gate accepts
only the conjunction of the root, context and enrollment checks
(`stark_attestation.rs`, Verus-checked).

# The Axiom Policy and the Proof-Corpus Commitment

Admissibility (Definition 1) is enforced by `AxiomProfile.lean`, which the CI
Lean job runs as a gate. Beyond enforcement, the proven corpus is bound to a
single reproducible value.

**Definition 16 (Proof-corpus root).** Let $\Sigma$ be the sequence, in a fixed
canonical order, of the SHA-256 digests of every Lean source in the corpus, the
pinned toolchain identifier, and the textual axiom-profile output. The
proof-corpus root is $\rho = \mathrm{SHA\text{-}256}(\Sigma)$.

The generator refuses to emit $\rho$ unless the corpus builds and every profiled
theorem is admissible; a single `sorry` or a non-standard axiom aborts it. Thus
$\rho$ is a 32-byte commitment whose existence certifies that a specific corpus,
under a specific toolchain, is fully proven under $\mathcal{S}$. The design for
binding $\rho$ as a public input of the kernel's boot attestation, so that the
boot proof commits to the exact set of proven properties, is specified
separately; it is an engineering binding, not a proof of the Lean kernel itself,
and is stated as such.

# Reproducibility and the Evidence Manifest

Every claim is reproducible from a clean checkout: source hygiene, the runnable
and Kani proofs, `lake build` and `lake env lean AxiomProfile.lean` for the Lean
corpus and its axiom gate, and the Charon/Aeneas regeneration for the extraction.
The whole surface is inventoried in a machine-readable manifest,
`verification/evidence/EVIDENCE.json`, derived deterministically from the source and
diff-checked in CI, so the counts in the repository are never stale prose but a
continuously verified artifact.

# Threat Model and Honest Scope

We state the boundaries plainly, because an honest scope is the point.

- **Not total functional correctness.** The security-critical properties above
  are proven; arbitrary functional behavior outside them is not. A subsystem may
  satisfy every stated property and still contain a functional defect beyond
  their scope.
- **Kani is bounded.** Model-checked properties hold up to the harness bound, not
  for unbounded inputs.
- **Known-answer vectors are conformance.** Passing standard test vectors is
  strong evidence, not a universal proof of an algorithm on every input.
- **Side channels are out of scope of these proofs.** The proofs are functional
  and structural, not timing proofs; the crypto documentation states where a
  primitive is not constant-time.
- **Hardware below the IOMMU line is trusted.** With the IOMMU backend not
  engaged, DMA safety rests on the broker's software bounds (Theorem 7) plus
  non-malicious device hardware.
- **The extraction trusts Charon and Aeneas** and the Lean kernel; these are
  small, published, and audited, and are the trusted computing base of Layer 2c.

# Related Work

seL4 [@klein2009sel4] proves total functional correctness of a minimal C kernel
and its refinement to the compiled code; it is the strongest whole-kernel result
and a different point on the spectrum from NØNOS, which proves a focused property
set over a larger Rust system. CompCert [@leroy2009compcert] verifies a C
compiler, addressing the compile-time gap that seL4's C-refinement also confronts;
NØNOS sidesteps part of this for its extracted functions by reasoning over the
compiler's own MIR via Charon and Aeneas [@charon; @aeneas], the same toolchain
used to verify cryptographic and systems Rust. RustBelt [@jung2018rustbelt]
establishes the soundness of the Rust type system that NØNOS relies on for its
memory-safety baseline.

# Conclusion

NØNOS demonstrates that a substantial, memory-safe systems kernel can carry
machine-checked security proofs whose link to the running code is not left as an
act of faith: runnable proofs execute the real source, Verus reasons over the
real bit-operations, and mechanical extraction lowers the compiler's own MIR into
Lean for direct proof. The corpus is large (887 admissible theorems), audited to
the axiom (168 flagship closures, zero `sorry`), reproducible, and inventoried in
a continuously checked manifest, with a transparent post-quantum attestation gate
enforcing the proven admission discipline at runtime. The scope is narrower than
total correctness and is stated as such; within that scope, every claim is
machine-checked over the code that runs.

# References
