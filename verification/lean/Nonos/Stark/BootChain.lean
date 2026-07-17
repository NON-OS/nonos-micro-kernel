/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The measured boot chain. Each stage measures the next and extends a running boot
digest, so the final digest commits to every stage in order. The theorems below
show the digest is a function of the stage measurements, extending by an appended
tail is extending by the head then the tail, and, when the extend step is
injective, tampering with any single stage changes the final digest: this is why a
swapped bootloader, kernel, or capsule cannot reach the enrolled boot root.
-/

namespace Nonos.Stark.BootChain

/-- Extend the running boot digest with one stage measurement. -/
def extend (mix : Nat → Nat → Nat) (acc stage : Nat) : Nat := mix acc stage

/-- Fold a list of stage measurements into the final boot digest. -/
def digest (mix : Nat → Nat → Nat) (seed : Nat) (stages : List Nat) : Nat :=
  stages.foldl (extend mix) seed

/-- Injectivity of the extend step in both arguments. -/
def MixInjective (mix : Nat → Nat → Nat) : Prop :=
  ∀ a b c d, mix a b = mix c d → a = c ∧ b = d

/-- No stages leaves the seed as the digest. -/
theorem digest_nil (mix : Nat → Nat → Nat) (seed : Nat) : digest mix seed [] = seed := rfl

/-- One stage, then the rest. -/
theorem digest_cons (mix : Nat → Nat → Nat) (seed s : Nat) (rest : List Nat) :
    digest mix seed (s :: rest) = digest mix (extend mix seed s) rest := rfl

/-- Extending by an appended run extends by the head, then the tail. -/
theorem digest_append (mix : Nat → Nat → Nat) (seed : Nat) (p q : List Nat) :
    digest mix seed (p ++ q) = digest mix (digest mix seed p) q := by
  simp only [digest, List.foldl_append]

/-- The digest is a function of the stage measurements. -/
theorem digest_deterministic (mix : Nat → Nat → Nat) (seed : Nat) (stages : List Nat)
    (d₁ d₂ : Nat) (h₁ : digest mix seed stages = d₁) (h₂ : digest mix seed stages = d₂) :
    d₁ = d₂ := by rw [← h₁, ← h₂]

/-- With an injective mix, the digest is injective in the seed. -/
theorem digest_injective_seed (mix : Nat → Nat → Nat) (hm : MixInjective mix) :
    ∀ (stages : List Nat) (x y : Nat), digest mix x stages = digest mix y stages → x = y := by
  intro stages
  induction stages with
  | nil => intro x y h; simpa [digest] using h
  | cons s rest ih =>
    intro x y h
    rw [digest_cons, digest_cons] at h
    exact (hm _ _ _ _ (ih _ _ h)).1

/-- With an injective mix, the digest is injective in a single trailing stage. -/
theorem last_stage_injective (mix : Nat → Nat → Nat) (hm : MixInjective mix)
    (seed : Nat) (stages : List Nat) (s s' : Nat)
    (h : digest mix seed (stages ++ [s]) = digest mix seed (stages ++ [s'])) : s = s' := by
  rw [digest_append, digest_append] at h
  simp only [digest_cons, digest_nil, extend] at h
  exact (hm _ _ _ _ h).2

/-- Tampering with the trailing stage changes the boot digest. -/
theorem tampered_stage_fails (mix : Nat → Nat → Nat) (hm : MixInjective mix)
    (seed : Nat) (stages : List Nat) (s s' : Nat) (hne : s ≠ s') :
    digest mix seed (stages ++ [s]) ≠ digest mix seed (stages ++ [s']) := by
  intro h; exact hne (last_stage_injective mix hm seed stages s s' h)

/-- A boot chain reaching the enrolled root pins its final stage under injectivity. -/
theorem digest_pins_last (mix : Nat → Nat → Nat) (hm : MixInjective mix)
    (seed root : Nat) (stages : List Nat) (s s' : Nat)
    (h₁ : digest mix seed (stages ++ [s]) = root)
    (h₂ : digest mix seed (stages ++ [s']) = root) : s = s' := by
  apply last_stage_injective mix hm seed stages
  rw [h₁, h₂]

/-- A single stage folds directly into the seed. -/
theorem single_stage (mix : Nat → Nat → Nat) (seed s : Nat) :
    digest mix seed [s] = mix seed s := rfl

end Nonos.Stark.BootChain
