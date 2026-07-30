/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The authority a capsule is allowed to start with.

`Spawn.lean` proves that only attested capsules run, over an uninterpreted
oracle: it says nothing about how much authority the ones that do run get.
That is decided by `src/security/capsule_manifest/verify/caps_bits.rs`, three
bit operations sitting between a publisher's signed NØNOS ID certificate and
the capability word installed on the PCB.

The chain is: `check_ceiling` rejects a manifest asking above the certificate,
`check_grant` rejects a grant outside the manifest, and `install_caps` is what
flows through `preflight` and `verified` into `proc_caps::install_spawn`
unchanged. Nothing narrows it again on the way.

Capabilities are modelled as sets of bit positions rather than as a `Nat`
bitmask, because containment is what the theorems are about and set inclusion
states it directly. `mechanism_proofs` carries the other half: the real Rust,
included by path, against a spec restated bit by bit, under Kani for every
64-bit input.

The subtlety worth stating explicitly is that `install_caps` is deliberately
not confined by the grant. Required capabilities pass regardless of it, so a
theorem bounding the result by `granted` would be false. The certificate
ceiling is the only real upper bound, which is exactly why it is the one
worth proving.
-/

namespace Nonos.SpawnCaps

/-- A capability set. -/
abbrev Caps := Nat → Bool

/-- `a` grants nothing outside `b`. -/
def Within (a b : Caps) : Prop := ∀ i, a i = true → b i = true

/-- Union, as `|` on the words. -/
def union (a b : Caps) : Caps := fun i => a i || b i

/-- Intersection, as `&` on the words. -/
def inter (a b : Caps) : Caps := fun i => a i && b i

/-- `within_ceiling`: everything the manifest may end up holding, required and
    optional alike, is permitted by the certificate. -/
def withinCeiling (required optional ceiling : Caps) : Prop :=
  Within (union required optional) ceiling

/-- `grant_within_manifest`: the caller cannot grant what the manifest never
    declared. -/
def grantWithinManifest (required optional granted : Caps) : Prop :=
  Within granted (union required optional)

/-- `install_caps`: every required capability, plus the optional ones the
    grant asked for. -/
def installCaps (required optional granted : Caps) : Caps :=
  union required (inter optional granted)

/-! ### The ceiling holds all the way to the PCB -/

/-- **The spawn ceiling property.** A capsule installs no authority its
    publisher's certificate does not permit.

    The grant check is not needed for this: `install_caps` cannot exceed the
    manifest whatever is granted, so the ceiling holds on the strength of
    `check_ceiling` alone. That is worth knowing, because it says the ceiling
    survives a bug in `check_grant`. What `check_grant` buys is separate and
    stated in `authority_only_narrows`.

    False under the bugs worth worrying about: returning
    `required | optional | granted`, testing `& ceiling` instead of
    `& !ceiling`, or dropping the optional set from the ceiling check. -/
theorem installed_within_ceiling (required optional granted ceiling : Caps)
    (hc : withinCeiling required optional ceiling) :
    Within (installCaps required optional granted) ceiling := by
  intro i hi
  apply hc i
  simp only [installCaps, union, inter, Bool.or_eq_true, Bool.and_eq_true] at hi
  simp only [union, Bool.or_eq_true]
  rcases hi with hreq | ⟨hopt, _⟩
  · exact Or.inl hreq
  · exact Or.inr hopt

/-- The installed set never exceeds what the manifest declares, with no
    hypothesis at all: the grant can only narrow the optional half. -/
theorem installed_within_manifest (required optional granted : Caps) :
    Within (installCaps required optional granted) (union required optional) := by
  intro i hi
  simp only [installCaps, union, inter, Bool.or_eq_true, Bool.and_eq_true] at hi
  simp only [union, Bool.or_eq_true]
  rcases hi with hreq | ⟨hopt, _⟩
  · exact Or.inl hreq
  · exact Or.inr hopt

/-- Required capabilities are installed whatever the grant says. Stated so the
    asymmetry is on the record rather than discovered later: this is why the
    ceiling, not the grant, is the bound that matters. -/
theorem required_always_installed (required optional granted : Caps) :
    Within required (installCaps required optional granted) := by
  intro i hi
  simp only [installCaps, union, Bool.or_eq_true]
  exact Or.inl hi

/-- An empty grant still installs exactly the required set. -/
theorem empty_grant_installs_required (required optional : Caps) (i : Nat) :
    installCaps required optional (fun _ => false) i = required i := by
  simp [installCaps, union, inter]

/-! ### The checks compose -/

/-- Passing both checks bounds the grant itself by the ceiling, not just what
    gets installed. This is the part `check_grant` earns: without it a caller
    could hand over authority outside the manifest, and while that authority
    would not reach the PCB through `install_caps`, it would still have been
    accepted as a grant. -/
theorem authority_only_narrows (required optional granted ceiling : Caps)
    (hc : withinCeiling required optional ceiling)
    (hg : grantWithinManifest required optional granted) :
    Within granted ceiling ∧ Within (installCaps required optional granted) ceiling :=
  ⟨fun i hi => hc i (hg i hi), installed_within_ceiling required optional granted ceiling hc⟩

end Nonos.SpawnCaps
