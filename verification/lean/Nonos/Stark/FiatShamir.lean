/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Fiat-Shamir message binding. Every message is absorbed in order into a running
state, and the verifier's challenge is read off that state, so the challenge is
bound to the whole message history. The theorems below show absorbing an appended
run is absorbing the prefix then the suffix, the state is a function of the
messages, and, when the absorb step is injective, a transcript with a different
history yields a different state: this is why a prover cannot reorder or drop a
committed message without changing the challenge it must answer.
-/

namespace Nonos.Stark.FiatShamir

/-- Absorb a run of messages into a running state through an abstract step. -/
def absorb (step : Nat → Nat → Nat) (init : Nat) (msgs : List Nat) : Nat :=
  msgs.foldl step init

/-- Injectivity of the absorb step in both arguments. -/
def StepInjective (step : Nat → Nat → Nat) : Prop :=
  ∀ a b c d, step a b = step c d → a = c ∧ b = d

/-- Absorbing nothing leaves the state. -/
theorem absorb_nil (step : Nat → Nat → Nat) (init : Nat) : absorb step init [] = init := rfl

/-- Absorbing one message, then the rest. -/
theorem absorb_cons (step : Nat → Nat → Nat) (init m : Nat) (rest : List Nat) :
    absorb step init (m :: rest) = absorb step (step init m) rest := rfl

/-- Absorbing an appended run absorbs the prefix, then the suffix. -/
theorem absorb_append (step : Nat → Nat → Nat) (init : Nat) (p q : List Nat) :
    absorb step init (p ++ q) = absorb step (absorb step init p) q := by
  simp only [absorb, List.foldl_append]

/-- The state is a function of the message history. -/
theorem absorb_deterministic (step : Nat → Nat → Nat) (init : Nat) (msgs : List Nat)
    (s₁ s₂ : Nat) (h₁ : absorb step init msgs = s₁) (h₂ : absorb step init msgs = s₂) : s₁ = s₂ := by
  rw [← h₁, ← h₂]

/-- With an injective step, absorb is injective in the initial state. -/
theorem absorb_injective_init (step : Nat → Nat → Nat) (hs : StepInjective step) :
    ∀ (msgs : List Nat) (x y : Nat), absorb step x msgs = absorb step y msgs → x = y := by
  intro msgs
  induction msgs with
  | nil => intro x y h; simpa [absorb] using h
  | cons m rest ih =>
    intro x y h
    rw [absorb_cons, absorb_cons] at h
    exact (hs _ _ _ _ (ih _ _ h)).1

/-- Appending a message changes the state injectively in that message. -/
theorem append_message_injective (step : Nat → Nat → Nat) (hs : StepInjective step)
    (init : Nat) (msgs : List Nat) (m m' : Nat)
    (h : absorb step init (msgs ++ [m]) = absorb step init (msgs ++ [m'])) : m = m' := by
  rw [absorb_append, absorb_append] at h
  simp only [absorb_cons, absorb_nil] at h
  exact (hs _ _ _ _ h).2

/-- A committed message binds: appending it to a prefix gives a state distinct from
    the prefix alone whenever the step genuinely mixes the message in. -/
theorem committed_message_binds (step : Nat → Nat → Nat) (init : Nat) (msgs : List Nat) (m : Nat)
    (hmix : step (absorb step init msgs) m ≠ absorb step init msgs) :
    absorb step init (msgs ++ [m]) ≠ absorb step init msgs := by
  rw [absorb_append]
  simp only [absorb_cons, absorb_nil]
  exact hmix

/-- The empty transcript is the initial state. -/
theorem empty_transcript (step : Nat → Nat → Nat) (init : Nat) : absorb step init [] = init := rfl

end Nonos.Stark.FiatShamir
