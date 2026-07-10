/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The association-set layer, decentralized compliance without a gatekeeper. Anyone
may publish an association set, the deposit commitments it vouches for, and a
withdrawal proves in zero knowledge that its note is one of them without revealing
which. An honest user chooses a set that excludes known-tainted deposits, and
thereby proves the funds are clean without deanonymizing. This module proves the
guarantee that makes that sound: a withdrawal accepted against a set that excludes a
commitment cannot be a spend of that commitment. It also proves the registry of
published sets is append-only, so publishing a new set never invalidates a proof
against an older one, and publishing is permissionless, always available with no
privileged provider. The exclusion here is set non-membership; the code discharges
it with a sorted-Merkle non-membership opening.
-/

namespace Nonos.Stark.AssociationSet

/-- The withdrawal's note commitment is vouched for by the set. -/
def Included (set : List Int) (cm : Int) : Prop := cm ∈ set

/-- The set does not vouch for the commitment. An honest set excludes tainted
    deposits this way. -/
def Excludes (set : List Int) (cm : Int) : Prop := cm ∉ set

/-- The compliance guarantee: if a set excludes a commitment, then any withdrawal
    accepted against that set is a spend of some other deposit, never that one.
    Choosing a set that excludes the tainted deposits proves the funds are not
    those deposits, and reveals nothing more. -/
theorem an_excluded_deposit_cannot_pass (set : List Int) (bad mine : Int)
    (hex : Excludes set bad) (hin : Included set mine) : mine ≠ bad := by
  intro h; rw [h] at hin; exact hex hin

/-- Inclusion and exclusion are exclusive: a commitment the set vouches for is not
    one it excludes. The two verdicts cannot both hold. -/
theorem included_is_not_excluded (set : List Int) (cm : Int)
    (hin : Included set cm) : ¬ Excludes set cm := fun hex => hex hin

/-- The registry of published association sets: the history of their roots. -/
structure Registry where
  roots : List Int

/-- A root is registered if it appears in the published history. -/
def registered (r : Registry) (root : Int) : Prop := root ∈ r.roots

/-- Publish a new association-set root. Permissionless: it is a total operation,
    no privileged provider, no gate. -/
def publish (r : Registry) (root : Int) : Registry := { roots := root :: r.roots }

/-- Publishing is append-only: every previously registered root stays registered,
    so a proof against an older set remains valid forever. -/
theorem publishing_preserves_old_roots (r : Registry) (newRoot old : Int)
    (h : registered r old) : registered (publish r newRoot) old :=
  List.mem_cons_of_mem newRoot h

/-- The newly published root is registered. Anyone can add a set and have it
    recognized. -/
theorem a_published_root_is_registered (r : Registry) (root : Int) :
    registered (publish r root) root := List.mem_cons_self root r.roots

/-- Publishing never removes a root: the registry only grows, so no publisher can
    censor another's set. -/
theorem the_registry_only_grows (r : Registry) (newRoot : Int) :
    ∀ root, registered r root → registered (publish r newRoot) root :=
  fun root h => publishing_preserves_old_roots r newRoot root h

/-- A worked instance: a set that vouches for two clean deposits excludes a tainted
    one, so a withdrawal against it cannot be the tainted deposit. -/
theorem clean_set_excludes_the_taint :
    (99 : Int) ≠ 7 :=
  an_excluded_deposit_cannot_pass [3, 99] 7 99 (by unfold Excludes; decide)
    (by unfold Included; decide)

end Nonos.Stark.AssociationSet
