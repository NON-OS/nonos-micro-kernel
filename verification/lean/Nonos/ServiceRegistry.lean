/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The service registry admission (`src/services/registry.rs::register_endpoint`).
A capsule registers a named service endpoint on a port. Re-registering the exact
same name, port and pid is idempotent; a request that reuses a name or a port
already held by another entry is refused; and the table is capped. The theorems
below fix the registry's key invariant: a name and a port each identify at most
one endpoint, and registration preserves that, so a capsule can never hijack a
name or a port another service already owns, nor grow the table past its cap.
-/

namespace Nonos.ServiceRegistry

/-- A registered endpoint: its name, port and owning pid, as ids. -/
structure Ep where
  name : Nat
  port : Nat
  pid : Nat
  deriving DecidableEq

/-- The request already exists exactly (same name, port and pid): the
    idempotent case. -/
def hasExact (eps : List Ep) (n p pd : Nat) : Prop :=
  ∃ e ∈ eps, e.name = n ∧ e.port = p ∧ e.pid = pd

/-- Some existing endpoint clashes on the name or the port. -/
def clash (eps : List Ep) (n p : Nat) : Prop :=
  ∃ e ∈ eps, e.name = n ∨ e.port = p

instance (eps : List Ep) (n p pd : Nat) : Decidable (hasExact eps n p pd) := by
  unfold hasExact; exact inferInstance

instance (eps : List Ep) (n p : Nat) : Decidable (clash eps n p) := by
  unfold clash; exact inferInstance

/-- The outcome of `register_endpoint`. -/
inductive Outcome where
  | ok (eps : List Ep)
  | alreadyExists
  | full
  deriving DecidableEq

/-- `register_endpoint`, branch for branch. -/
def register (eps : List Ep) (n p pd max : Nat) : Outcome :=
  if hasExact eps n p pd then .ok eps
  else if clash eps n p then .alreadyExists
  else if eps.length ≥ max then .full
  else .ok (⟨n, p, pd⟩ :: eps)

/-- Names identify at most one endpoint. -/
def NamesUnique (eps : List Ep) : Prop := eps.Pairwise (fun a b => a.name ≠ b.name)

/-- Ports identify at most one endpoint. -/
def PortsUnique (eps : List Ep) : Prop := eps.Pairwise (fun a b => a.port ≠ b.port)

/-- Re-registering the exact same endpoint is a no-op: the table is unchanged. -/
theorem idempotent (eps : List Ep) (n p pd max : Nat) (h : hasExact eps n p pd) :
    register eps n p pd max = .ok eps := by
  unfold register; rw [if_pos h]

/-- A request that reuses a name or a port already held (and is not an exact
    re-register) is refused. -/
theorem clash_rejected (eps : List Ep) (n p pd max : Nat)
    (hne : ¬ hasExact eps n p pd) (hc : clash eps n p) :
    register eps n p pd max = .alreadyExists := by
  unfold register; rw [if_neg hne, if_pos hc]

/-- Registration preserves name uniqueness: after any successful registration no
    two endpoints share a name. A capsule cannot register a name another
    endpoint already owns. -/
theorem register_preserves_names (eps eps' : List Ep) (n p pd max : Nat)
    (hu : NamesUnique eps) (h : register eps n p pd max = .ok eps') :
    NamesUnique eps' := by
  unfold register at h
  by_cases he : hasExact eps n p pd
  · rw [if_pos he] at h; injection h with h'; rw [← h']; exact hu
  · rw [if_neg he] at h
    by_cases hc : clash eps n p
    · rw [if_pos hc] at h; exact absurd h (by simp)
    · rw [if_neg hc] at h
      by_cases hf : eps.length ≥ max
      · rw [if_pos hf] at h; exact absurd h (by simp)
      · rw [if_neg hf] at h
        injection h with h'
        rw [← h']
        refine List.pairwise_cons.mpr ⟨?_, hu⟩
        intro e he'
        intro hname
        exact hc ⟨e, he', Or.inl hname.symm⟩

/-- Registration preserves port uniqueness: after any successful registration no
    two endpoints share a port. -/
theorem register_preserves_ports (eps eps' : List Ep) (n p pd max : Nat)
    (hu : PortsUnique eps) (h : register eps n p pd max = .ok eps') :
    PortsUnique eps' := by
  unfold register at h
  by_cases he : hasExact eps n p pd
  · rw [if_pos he] at h; injection h with h'; rw [← h']; exact hu
  · rw [if_neg he] at h
    by_cases hc : clash eps n p
    · rw [if_pos hc] at h; exact absurd h (by simp)
    · rw [if_neg hc] at h
      by_cases hf : eps.length ≥ max
      · rw [if_pos hf] at h; exact absurd h (by simp)
      · rw [if_neg hf] at h
        injection h with h'
        rw [← h']
        refine List.pairwise_cons.mpr ⟨?_, hu⟩
        intro e he'
        intro hport
        exact hc ⟨e, he', Or.inr hport.symm⟩

/-- Registration never grows the table past its cap: a successful result stays
    within `max` whenever the table did. -/
theorem register_within_cap (eps eps' : List Ep) (n p pd max : Nat)
    (hb : eps.length ≤ max) (h : register eps n p pd max = .ok eps') :
    eps'.length ≤ max := by
  unfold register at h
  by_cases he : hasExact eps n p pd
  · rw [if_pos he] at h; injection h with h'; rw [← h']; exact hb
  · rw [if_neg he] at h
    by_cases hc : clash eps n p
    · rw [if_pos hc] at h; exact absurd h (by simp)
    · rw [if_neg hc] at h
      by_cases hf : eps.length ≥ max
      · rw [if_pos hf] at h; exact absurd h (by simp)
      · rw [if_neg hf] at h
        injection h with h'
        rw [← h']
        simp only [List.length_cons]
        omega

end Nonos.ServiceRegistry
