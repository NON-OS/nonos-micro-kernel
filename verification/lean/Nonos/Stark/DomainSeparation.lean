/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Domain separation of the transcripts. The base proof and the money-grade extension
proof seed their transcripts with distinct labels, so a challenge drawn for one is
never a valid challenge for the other. The theorems below show that, under a
tag-injective seeding, distinct domain tags yield distinct seeds and a proof drawn
under one domain cannot be replayed under another: this is why base and extension
proofs, and the capsule and kernel domains, never cross.
-/

namespace Nonos.Stark.DomainSeparation

/-- Seed a transcript from a domain tag and a message through an abstract seeding. -/
def seed (mk : Nat → Nat → Nat) (tag msg : Nat) : Nat := mk tag msg

/-- The seeding is tag-injective: distinct tags seed distinct states on equal messages. -/
def TagInjective (mk : Nat → Nat → Nat) : Prop :=
  ∀ t₁ t₂ m, mk t₁ m = mk t₂ m → t₁ = t₂

/-- Seeding is a function of the tag and message. -/
theorem seed_deterministic (mk : Nat → Nat → Nat) (tag msg : Nat) (s₁ s₂ : Nat)
    (h₁ : seed mk tag msg = s₁) (h₂ : seed mk tag msg = s₂) : s₁ = s₂ := by
  rw [← h₁, ← h₂]

/-- Distinct domain tags produce distinct seeds under tag-injectivity. -/
theorem distinct_tags_distinct_seeds (mk : Nat → Nat → Nat) (ht : TagInjective mk)
    (t₁ t₂ msg : Nat) (hne : t₁ ≠ t₂) : seed mk t₁ msg ≠ seed mk t₂ msg := by
  intro h; exact hne (ht t₁ t₂ msg h)

/-- A proof pinned to one domain seed does not match another domain's seed. -/
theorem no_cross_domain (mk : Nat → Nat → Nat) (ht : TagInjective mk)
    (base ext msg : Nat) (hne : base ≠ ext) : seed mk base msg ≠ seed mk ext msg :=
  distinct_tags_distinct_seeds mk ht base ext msg hne

/-- Two equal seeds under the same message force equal tags. -/
theorem equal_seed_equal_tag (mk : Nat → Nat → Nat) (ht : TagInjective mk)
    (t₁ t₂ msg : Nat) (h : seed mk t₁ msg = seed mk t₂ msg) : t₁ = t₂ :=
  ht t₁ t₂ msg h

/-- The capsule and kernel attestation domains never share a seed, being distinct tags. -/
theorem capsule_kernel_separated (mk : Nat → Nat → Nat) (ht : TagInjective mk)
    (capsuleTag kernelTag msg : Nat) (hne : capsuleTag ≠ kernelTag) :
    seed mk capsuleTag msg ≠ seed mk kernelTag msg :=
  distinct_tags_distinct_seeds mk ht capsuleTag kernelTag msg hne

/-- Domain separation is symmetric: order of comparison does not matter. -/
theorem separation_symmetric (mk : Nat → Nat → Nat)
    (t₁ t₂ msg : Nat) (h : seed mk t₁ msg ≠ seed mk t₂ msg) : seed mk t₂ msg ≠ seed mk t₁ msg :=
  fun heq => h heq.symm

end Nonos.Stark.DomainSeparation
