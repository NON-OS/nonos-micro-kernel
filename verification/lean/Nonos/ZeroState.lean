/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The end of a boot (`src/security/zerostate/`).

ZeroState says the machine holds nothing once it stops. Whether that is true
comes down to one question: is there a path from a running system to a powered
off one that does not wipe on the way. Before this module there were two, and
the reboot path was the worse of them, since a warm reset keeps DRAM powered
and its rows readable.

The fix is structural rather than a reminder to call the wipe. `arch::power`
is crate private and `security::zerostate::terminate` is its only caller, so
the firmware cannot be reached except through the wipe. What follows models
that shape as a transition system and proves the property it buys: every
reachable powered off state has been wiped, and every reachable state whose
memory still holds secrets is one the machine has not left yet.
-/

namespace Nonos.ZeroState

/-- Where a boot can be. `Quiesced` is the window after the other CPUs have
    been stopped and before the wipe runs, which exists so that the wipe is a
    snapshot rather than a race against a core that is still scheduling. -/
inductive Phase where
  | Running
  | Quiesced
  | Wiped
  | Off
  deriving DecidableEq, Repr

open Phase

/-- Which way the machine is leaving. Both ends reach the same firmware call
    through the same wipe, so the proofs below never need to case on it. -/
inductive Exit where
  | Shutdown
  | Reboot
  deriving DecidableEq, Repr

/-- A step of the terminal path, mirroring `terminate` one line at a time.

    There is deliberately no constructor from `Running` or `Quiesced` to
    `Off`. That absence is the whole design: it is what makes reaching the
    firmware without wiping not merely discouraged but unrepresentable, and
    it holds in the code because `arch::power::enter` is crate private with a
    single caller. -/
inductive Step : Phase → Phase → Prop where
  /-- `broadcast_ipi(Ipi::Stop)`: the other cores stop scheduling. -/
  | stop : Step Running Quiesced
  /-- `zerostate_shutdown_wipe()`: process memory, kernel heap, key vault. -/
  | wipe : Step Quiesced Wiped
  /-- `enter(off)`: the firmware takes the machine and does not give it back. -/
  | power : Step Wiped Off

/-- Reachability from a running system, in the reflexive transitive sense. -/
inductive Reaches : Phase → Phase → Prop where
  | refl (p : Phase) : Reaches p p
  | tail {p q r : Phase} : Reaches p q → Step q r → Reaches p r

/-- Does this phase still hold anything worth reading out of DRAM? Running and
    quiesced systems do; a wiped one does not, and neither does a machine that
    has already gone, because the only way it got there was through the wipe.
    That last clause is a claim, and it is what `off_was_wiped` discharges. -/
def holdsSecrets : Phase → Bool
  | Running => true
  | Quiesced => true
  | Wiped => false
  | Off => false

/-! ### The theorems -/

/-- Nothing steps out of `Off`. Once the firmware has the machine there is no
    kernel left to run, which is why `terminate` is divergent. -/
theorem off_is_terminal (p : Phase) (h : Step Off p) : False := by
  cases h

/-- The only step into `Off` is from `Wiped`. Stated on a single step, and
    lifted to whole runs by `off_was_wiped` below. -/
theorem into_off_only_from_wiped (p : Phase) (h : Step p Off) : p = Wiped := by
  cases h; rfl

/-- **The ZeroState property.** A boot that starts running and ends powered
    off passed through the wipe to get there.

    Anchored at `Running` because that is the claim: a machine someone booted
    and then stopped. This is the statement that was false before this module
    existed, when the shutdown syscall returned `E_NOTSUP` and the reboot
    syscall called ACPI straight out of the router, so a running system
    reached the firmware with its memory intact. -/
theorem off_was_wiped (h : Reaches Running Off) : Reaches Running Wiped := by
  cases h with
  | tail hpq hqr => cases hqr; exact hpq

/-- No powered off machine still holds secrets. The corollary a user actually
    cares about: pull the plug and the DRAM has nothing left in it. -/
theorem off_holds_nothing : holdsSecrets Off = false := rfl

/-- Turned around: while memory still holds secrets, the machine has not left.
    An attacker who wants to read DRAM has to catch a system that is still
    running, which is a different and much louder attack than lifting the
    chips out of a box that was shut down a minute ago. -/
theorem secrets_imply_still_here (p : Phase) (h : holdsSecrets p = true) :
    p ≠ Off := by
  intro hoff; rw [hoff] at h; exact absurd h (by simp [holdsSecrets])

/-- The wipe cannot be skipped: `Running` does not reach `Off` in one step. -/
theorem no_shortcut_to_off (h : Step Running Off) : False := by
  cases h

/-- Nor can quiescing alone stand in for it. Stopping the other CPUs makes the
    wipe sound; it does not make it unnecessary. -/
theorem quiescing_is_not_wiping (h : Step Quiesced Off) : False := by
  cases h

end Nonos.ZeroState
