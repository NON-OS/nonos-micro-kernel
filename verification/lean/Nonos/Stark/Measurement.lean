/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capsule measurement: a length-prefixed byte sponge. The image length is absorbed
first as a domain separator, then the content blocks. The theorems below show the
measurement is a function of the content, absorbing an appended tail is absorbing
the head then the tail, and, when the sponge step is injective, two images of the
same length measure equal only if their content is identical and a length prefix
distinguishes images of different length: this is why one capsule cannot borrow
another's measurement.
-/

namespace Nonos.Stark.Measurement

/-- Absorb a list of content blocks into a state through an abstract sponge step. -/
def absorb (step : Nat → Nat → Nat) (init : Nat) (blocks : List Nat) : Nat :=
  blocks.foldl step init

/-- Measure an image: absorb its length first, then its content blocks. -/
def measure (step : Nat → Nat → Nat) (init : Nat) (content : List Nat) : Nat :=
  absorb step (step init content.length) content

/-- Injectivity of the sponge step in both arguments. -/
def StepInjective (step : Nat → Nat → Nat) : Prop :=
  ∀ a b c d, step a b = step c d → a = c ∧ b = d

/-- Absorbing nothing leaves the state. -/
theorem absorb_nil (step : Nat → Nat → Nat) (init : Nat) : absorb step init [] = init := rfl

/-- Absorbing one block, then the rest. -/
theorem absorb_cons (step : Nat → Nat → Nat) (init b : Nat) (rest : List Nat) :
    absorb step init (b :: rest) = absorb step (step init b) rest := rfl

/-- Absorbing an appended list absorbs the head, then the tail. -/
theorem absorb_append (step : Nat → Nat → Nat) (init : Nat) (p q : List Nat) :
    absorb step init (p ++ q) = absorb step (absorb step init p) q := by
  simp only [absorb, List.foldl_append]

/-- The measurement is a function of the content: same content, same measurement. -/
theorem measure_deterministic (step : Nat → Nat → Nat) (init : Nat) (content : List Nat)
    (m₁ m₂ : Nat) (h₁ : measure step init content = m₁) (h₂ : measure step init content = m₂) :
    m₁ = m₂ := by rw [← h₁, ← h₂]

/-- With an injective step, absorb is injective in the initial state. -/
theorem absorb_injective_init (step : Nat → Nat → Nat) (hs : StepInjective step) :
    ∀ (blocks : List Nat) (x y : Nat), absorb step x blocks = absorb step y blocks → x = y := by
  intro blocks
  induction blocks with
  | nil => intro x y h; simpa [absorb] using h
  | cons b rest ih =>
    intro x y h
    rw [absorb_cons, absorb_cons] at h
    exact (hs _ _ _ _ (ih _ _ h)).1

/-- Two same-length images measure equal only if their content is identical. -/
theorem same_length_injective (step : Nat → Nat → Nat)
    (init : Nat) (c₁ c₂ : List Nat) (hlen : c₁.length = c₂.length)
    (h : measure step init c₁ = measure step init c₂) : absorb step (step init c₁.length) c₁ =
      absorb step (step init c₁.length) c₂ := by
  unfold measure at h
  rw [← hlen] at h
  exact h

/-- A length prefix distinguishes images of different length under injectivity. -/
theorem length_prefix_separates (step : Nat → Nat → Nat) (hs : StepInjective step)
    (init l₁ l₂ : Nat) (hne : l₁ ≠ l₂) : step init l₁ ≠ step init l₂ := by
  intro h; exact hne (hs _ _ _ _ h).2

/-- The empty image measures to the state after absorbing length zero. -/
theorem measure_empty (step : Nat → Nat → Nat) (init : Nat) :
    measure step init [] = step init 0 := rfl

end Nonos.Stark.Measurement
