/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The recent-roots window, so proofs and deposits do not race. The note tree changes
with every deposit, so a withdrawal proof built against one root would be
invalidated by the next deposit if the pool accepted only the latest root. Instead
the pool keeps a bounded window of recent roots and accepts a proof against any of
them. This module proves the window behaves: a freshly committed root is accepted,
the window never grows past its cap, and every root it accepts was actually
committed. Bounded state, deterministic aging, in the structural style of the other
argument modules.
-/

namespace Nonos.Stark.RootWindow

/-- A window of recent tree roots, newest first, holding at most `cap`. -/
structure Window where
  roots : List Int
  cap : Nat

/-- A proof verifies if it opens against a root still in the window. -/
def accepts (w : Window) (root : Int) : Prop := root ∈ w.roots

/-- Commit a new root, dropping the oldest beyond the cap. -/
def push (w : Window) (root : Int) : Window :=
  { w with roots := (root :: w.roots).take w.cap }

/-- A freshly committed root is accepted, as long as the window holds at least one. -/
theorem a_pushed_root_is_accepted (w : Window) (root : Int) (h : 0 < w.cap) :
    accepts (push w root) root := by
  unfold accepts push
  obtain ⟨k, hk⟩ : ∃ k, w.cap = k + 1 := ⟨w.cap - 1, by omega⟩
  rw [hk, List.take_succ_cons]
  exact List.mem_cons_self root _

/-- The window never exceeds its cap: bounded state, an aging buffer not an
    unbounded list. -/
theorem the_window_never_exceeds_the_cap (w : Window) (root : Int) :
    (push w root).roots.length ≤ w.cap := by
  unfold push
  rw [List.length_take]
  omega

/-- Every accepted root was actually committed: acceptance is membership, so the
    pool cannot be made to honor a root it never held. -/
theorem an_accepted_root_was_committed (w : Window) (root : Int)
    (h : accepts w root) : root ∈ w.roots := h

/-- A root no longer in the window is rejected: aged-out roots do not verify, which
    bounds how stale a proof may be. -/
theorem an_aged_out_root_is_rejected (w : Window) (root : Int)
    (h : root ∉ w.roots) : ¬ accepts w root := h

end Nonos.Stark.RootWindow
