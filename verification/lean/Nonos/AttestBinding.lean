/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Binding the transparent STARK attestation gate to the conjoined security machine.

The conjoined invariant of `Nonos.Secure` carries an abstract attestation oracle
`attest : Nat -> Bool` and proves that only an attested capsule is ever admitted.
The STARK modules prove, separately, that the gate accepts an attestation exactly
when it opens to the enrolled policy root under the bound context with an enrolled
measurement. This module instantiates the abstract oracle with the STARK gate's
own accept decision and joins the two results: from a clean start, after any
adversarial trace, every admitted capsule's measured image is one that was
enrolled in the policy. Nothing runs whose image the gate did not accept, and the
gate accepts only enrolled images, so nothing runs whose image was not enrolled.
-/

import Nonos.Secure
import Nonos.Stark.AttestSoundness

namespace Nonos.AttestBinding

open Nonos.Secure
open Nonos.Stark.AttestSoundness

/-- The security machine's attestation oracle, instantiated by the transparent
    STARK gate: a capsule is attested exactly when its attestation opens to the
    enrolled policy root, under the bound context, with an enrolled measurement.
    `attOf` names the attestation each capsule presents. -/
def starkAttest (pr bc : Nat) (pol : List Nat) (attOf : Nat → Attestation) : Nat → Bool :=
  fun cap =>
    let a := attOf cap
    decide (a.root = pr ∧ a.context = bc ∧ a.measurement ∈ pol)

/-- The instantiated oracle returns `true` exactly when the STARK gate accepts. -/
theorem starkAttest_true_iff (pr bc : Nat) (pol : List Nat)
    (attOf : Nat → Attestation) (cap : Nat) :
    starkAttest pr bc pol attOf cap = true ↔ accept pr bc pol (attOf cap) := by
  unfold starkAttest accept
  simp [decide_eq_true_eq]

/-- No transition changes the attestation oracle: it is fixed policy, not mutable
    state. -/
theorem step_preserves_attest (s : State) (sc : Syscall) :
    (step s sc).attest = s.attest := by
  cases sc with
  | attenuate pid mask => rfl
  | transfer src dst b => simp only [step]; split <;> rfl
  | revoke pid b => rfl
  | mapPage page p => simp only [step]; split <;> rfl
  | userCopy addr len => simp only [step]; split <;> rfl
  | boot v => rfl
  | spawn cap => simp only [step]; split <;> rfl
  | dmaMap r c => simp only [step]; split <;> rfl
  | loadElf e => simp only [step]; split <;> rfl
  | bindMsix i => simp only [step]; split <;> rfl
  | acquire pid n => rfl

/-- The attestation oracle is the same after any trace as before it. -/
theorem attest_locality (s : State) (tr : List Syscall) :
    (run s tr).attest = s.attest := by
  induction tr generalizing s with
  | nil => rfl
  | cons sc rest ih =>
    calc (run s (sc :: rest)).attest
        = (run (step s sc) rest).attest := rfl
      _ = (step s sc).attest := ih (step s sc)
      _ = s.attest := step_preserves_attest s sc

/-- Bridge, at a fixed state: if the machine runs the STARK-instantiated oracle,
    every admitted capsule's attestation is accepted by the gate, so its measured
    image is enrolled in the policy. -/
theorem admitted_is_enrolled (pr bc : Nat) (pol : List Nat) (attOf : Nat → Attestation)
    (s0 s : State) (hsec : Secure s0 s)
    (hattest : s.attest = starkAttest pr bc pol attOf)
    (c : Nat) (hc : c ∈ s.admitted) :
    (attOf c).measurement ∈ pol := by
  have ht : s.attest c = true := hsec.admitted_attested c hc
  rw [hattest] at ht
  exact accept_enrolled pr bc pol (attOf c)
    ((starkAttest_true_iff pr bc pol attOf c).mp ht)

/-- The end-to-end theorem: from a clean start whose attestation oracle is the
    STARK gate, after any adversarial trace of any length, every admitted
    capsule's measured image was enrolled in the policy. This joins the conjoined
    safety invariant to the gate's soundness: nothing runs whose image the gate
    did not accept, and the gate accepts only enrolled images. -/
theorem admitted_enrolled_after_trace (pr bc : Nat) (pol : List Nat)
    (attOf : Nat → Attestation) (s0 : State) (tr : List Syscall)
    (hattest0 : s0.attest = starkAttest pr bc pol attOf)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) (hadmit : s0.admitted = [])
    (hdma : s0.dma = []) (helf : s0.elf = []) (hirq : s0.irq = [])
    (hquota : ∀ p, Quota.ok (s0.quota p))
    (c : Nat) (hc : c ∈ (run s0 tr).admitted) :
    (attOf c).measurement ∈ pol := by
  have hsec := every_trace_is_secure s0 tr hwx hcopies hadmit hdma helf hirq hquota
  have hattest : (run s0 tr).attest = starkAttest pr bc pol attOf := by
    rw [attest_locality s0 tr, hattest0]
  exact admitted_is_enrolled pr bc pol attOf s0 (run s0 tr) hsec hattest c hc

/-- The full gate verdict for every admitted capsule, end to end: from a clean
    start under the STARK oracle, after any adversarial trace, every admitted
    capsule's attestation was accepted by the gate. Everything the gate's
    soundness lemmas conclude about an accepted attestation then holds of every
    capsule that ran. -/
theorem admitted_accepted_after_trace (pr bc : Nat) (pol : List Nat)
    (attOf : Nat → Attestation) (s0 : State) (tr : List Syscall)
    (hattest0 : s0.attest = starkAttest pr bc pol attOf)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) (hadmit : s0.admitted = [])
    (hdma : s0.dma = []) (helf : s0.elf = []) (hirq : s0.irq = [])
    (hquota : ∀ p, Quota.ok (s0.quota p))
    (c : Nat) (hc : c ∈ (run s0 tr).admitted) :
    accept pr bc pol (attOf c) := by
  have hsec := every_trace_is_secure s0 tr hwx hcopies hadmit hdma helf hirq hquota
  have ht : (run s0 tr).attest c = true := hsec.admitted_attested c hc
  rw [attest_locality s0 tr, hattest0] at ht
  exact (starkAttest_true_iff pr bc pol attOf c).mp ht

/-- No cross-policy or cross-identity replay: after any adversarial trace from a
    clean start, every admitted capsule's attestation opened to exactly the
    enrolled policy root under exactly the bound context. A proof drawn against
    another policy root, or bound to another identity, admits nothing: it can
    never carry a capsule into execution. -/
theorem no_cross_policy_replay (pr bc : Nat) (pol : List Nat)
    (attOf : Nat → Attestation) (s0 : State) (tr : List Syscall)
    (hattest0 : s0.attest = starkAttest pr bc pol attOf)
    (hwx : ∀ page pm, s0.mapped page = some pm →
      ¬(pm.write = true ∧ pm.execute = true))
    (hcopies : s0.copies = []) (hadmit : s0.admitted = [])
    (hdma : s0.dma = []) (helf : s0.elf = []) (hirq : s0.irq = [])
    (hquota : ∀ p, Quota.ok (s0.quota p))
    (c : Nat) (hc : c ∈ (run s0 tr).admitted) :
    (attOf c).root = pr ∧ (attOf c).context = bc := by
  have hacc := admitted_accepted_after_trace pr bc pol attOf s0 tr hattest0 hwx hcopies hadmit hdma helf hirq hquota c hc
  exact ⟨accept_root pr bc pol (attOf c) hacc, accept_context pr bc pol (attOf c) hacc⟩

end Nonos.AttestBinding
