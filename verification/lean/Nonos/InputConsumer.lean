/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Input authority admission (`src/capabilities/token/types/authority_broker.rs`
and the drain/wait gate in `src/syscall/contract/cap_table/mk.rs`). Posting an
input event and consuming (draining) the global input ring are two different
rights. Posting is open to an input source, an interrupt owner, or admin;
consuming must not accept a bare interrupt owner, because every device driver
holds `Irq`, so if draining accepted `Irq` any driver could read the focused
application's keystrokes. The fix splits the predicate: `can_input_source`
(post) still accepts `Irq`, `can_input_consumer` (drain/wait) does not. The
theorems fix that consuming is strictly more restricted than posting and that an
interrupt-only driver may post but can never drain.
-/

namespace Nonos.InputConsumer

/-- The input-relevant rights a capability token may hold. -/
structure Rights where
  inputSource : Bool
  irq : Bool
  admin : Bool

/-- `can_input_source`: may POST an input event. Accepts an input source, any
    interrupt owner, or admin. -/
def canPost (r : Rights) : Bool := r.inputSource || r.irq || r.admin

/-- `can_input_consumer`: may DRAIN or WAIT on the input ring. Accepts an input
    source or admin only, never a bare interrupt owner. -/
def canDrain (r : Rights) : Bool := r.inputSource || r.admin

/-- Draining is strictly more restricted than posting: anyone who may drain may
    also post, so narrowing the drain gate never widens the post gate. -/
theorem drain_implies_post (r : Rights) (h : canDrain r = true) : canPost r = true := by
  obtain ⟨s, i, a⟩ := r
  unfold canDrain canPost at *
  cases s <;> cases i <;> cases a <;> simp_all

/-- Draining ignores the interrupt right entirely: flipping `irq` never changes
    the drain verdict. -/
theorem drain_ignores_irq (s a : Bool) :
    canDrain ⟨s, true, a⟩ = canDrain ⟨s, false, a⟩ := rfl

/-- An interrupt-only driver (holds `Irq`, is not an input source, not admin)
    may post input events but can never drain them: the keylogging path is
    closed. -/
theorem irq_only_cannot_drain (r : Rights)
    (h1 : r.inputSource = false) (h2 : r.admin = false) (h3 : r.irq = true) :
    canPost r = true ∧ canDrain r = false := by
  unfold canPost canDrain; rw [h1, h2, h3]; simp

end Nonos.InputConsumer
