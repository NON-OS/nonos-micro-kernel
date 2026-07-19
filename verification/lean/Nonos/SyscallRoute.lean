/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The microkernel syscall router (`src/syscall/microkernel/dispatch/route.rs`).
`route` tries each handler group in a fixed order (ipc, process, capability,
device, irq, then mmio, dma, pio, debug); the first group that returns `Some`
answers the call and its result is returned, and a number no group claims falls
through to the sentinel `-1` (ENOSYS). The theorems below fix that dispatch
discipline: an unclaimed syscall number is always the error sentinel, the first
group that claims a number decides the result and shadows every later group, and
a claimed number never falls through to the unknown-syscall sentinel by default.
This is the routing contract the whole syscall surface rests on: every call has
exactly one owner, decided by order, and anything unowned is refused.
-/

namespace Nonos.SyscallRoute

/-- A handler group: given a syscall number it either claims the call with a
    result, or declines with `none`. Mirrors each `mod::handle(nr, args)`
    returning `Option<i64>`. -/
abbrev Handler := Nat → Option Int

/-- The ENOSYS sentinel `route` returns for an unclaimed number. -/
def enosys : Int := -1

/-- Try each handler in order; the first to claim the number answers, otherwise
    the sentinel. This is the `if let Some(r) = h(nr) { return r }` chain of
    `route`, in order, ending in `-1`. -/
def route : List Handler → Nat → Int
  | [], _ => enosys
  | h :: hs, nr =>
    match h nr with
    | some r => r
    | none => route hs nr

/-- With no handler at all, every number is refused. -/
theorem route_nil (nr : Nat) : route [] nr = enosys := rfl

/-- The first handler that claims a number decides the result: its answer is
    returned and no later handler is consulted. This is first-match precedence,
    the reason handler order is the routing contract. -/
theorem route_cons_claim (h : Handler) (hs : List Handler) (nr : Nat) (r : Int)
    (hc : h nr = some r) : route (h :: hs) nr = r := by
  simp [route, hc]

/-- A handler that declines passes the number to the rest, unchanged. -/
theorem route_cons_decline (h : Handler) (hs : List Handler) (nr : Nat)
    (hd : h nr = none) : route (h :: hs) nr = route hs nr := by
  simp [route, hd]

/-- A syscall number no handler claims is always the ENOSYS sentinel: there is
    no unclaimed number that resolves to anything but refusal. -/
theorem route_unclaimed_is_enosys (hs : List Handler) (nr : Nat)
    (h : ∀ g ∈ hs, g nr = none) : route hs nr = enosys := by
  induction hs with
  | nil => rfl
  | cons g gs ih =>
    have hg : g nr = none := h g (List.mem_cons_self g gs)
    rw [route_cons_decline g gs nr hg]
    exact ih (fun x hx => h x (List.mem_cons_of_mem g hx))

/-- Contrapositive, the useful direction at a call site: if `route` did not
    refuse a number, some handler claimed it. A non-error result is never
    manufactured out of nothing. -/
theorem route_not_enosys_has_claimer (hs : List Handler) (nr : Nat)
    (h : route hs nr ≠ enosys) : ∃ g ∈ hs, ∃ r, g nr = some r := by
  induction hs with
  | nil => exact absurd (route_nil nr) h
  | cons g gs ih =>
    cases hgn : g nr with
    | some r => exact ⟨g, List.mem_cons_self g gs, r, hgn⟩
    | none =>
      rw [route_cons_decline g gs nr hgn] at h
      obtain ⟨x, hx, r, hxr⟩ := ih h
      exact ⟨x, List.mem_cons_of_mem g hx, r, hxr⟩

/-- Prepending a handler that claims a number overrides whatever the rest would
    have done: an earlier group shadows a later group on any number both claim,
    so the order is a strict priority, not a suggestion. -/
theorem route_earlier_shadows (h : Handler) (hs : List Handler) (nr : Nat)
    (r : Int) (hc : h nr = some r) : route (h :: hs) nr = r :=
  route_cons_claim h hs nr r hc

/-- Appending handlers never changes an already-claimed number: once a prefix
    claims `nr`, the tail is irrelevant, so adding a new subsystem at the end
    cannot capture calls an existing subsystem already owns. -/
theorem route_append_stable (hs ts : List Handler) (nr : Nat) (r : Int)
    (h : route hs nr = r) (hclaim : ∃ g ∈ hs, g nr ≠ none) :
    route (hs ++ ts) nr = r := by
  induction hs with
  | nil => obtain ⟨g, hg, _⟩ := hclaim; exact absurd hg (List.not_mem_nil g)
  | cons g gs ih =>
    cases hgn : g nr with
    | some v =>
      rw [List.cons_append, route_cons_claim g (gs ++ ts) nr v hgn]
      rw [route_cons_claim g gs nr v hgn] at h
      exact h
    | none =>
      rw [List.cons_append, route_cons_decline g (gs ++ ts) nr hgn]
      rw [route_cons_decline g gs nr hgn] at h
      refine ih h ?_
      obtain ⟨x, hx, hxn⟩ := hclaim
      rcases List.mem_cons.mp hx with rfl | hx'
      · exact absurd hgn hxn
      · exact ⟨x, hx', hxn⟩

end Nonos.SyscallRoute
