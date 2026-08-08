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

Ordering alone is not the whole claim. A wipe that runs in the right place but
covers the wrong memory satisfies every reachability theorem here, which is
what happened: the heap erase addressed a layout constant nothing maps, and
the kernel stacks had no mechanism at all. So the regions a running system
holds are enumerated, each is tied to the call that clears it, and what a
wiped machine still holds is derived from that table rather than declared. A
region without a mechanism now fails a proof instead of passing silently.
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

/-- A place a running kernel keeps material worth reading out of DRAM.

    Ordering matters and the wipe respects it: the key vault and the
    filesystem caches are walked while the heap is still readable because they
    live in it, and the heap erase runs last because it takes the allocator's
    own free list with it. -/
inductive Region where
  /-- User pages of every process, translated in the owning address space. -/
  | processMemory
  /-- The allocator's arena: IPC payloads, loader scratch, cached objects. -/
  | kernelHeap
  /-- Per-process kernel stacks, which come from the page allocator and so are
      reached by neither the heap erase nor the process wipe. -/
  | kernelStacks
  /-- The VFS and cryptofs caches. -/
  | fsCaches
  /-- The key vault. -/
  | keyVault
  deriving DecidableEq, Repr

/-- Everything a running system holds. A constructor added above without a
    mechanism below breaks `wipe_covers_every_region`, which is the point of
    listing them rather than asserting the wipe is complete. -/
def liveRegions : List Region :=
  [.processMemory, .kernelHeap, .kernelStacks, .fsCaches, .keyVault]

/-- What `zerostate_shutdown_wipe` clears, one entry per call it makes:
    `sanitize_process_memory` for every process, `wipe_kernel_stacks`,
    `fs::clear_caches`, `vault::zeroize_all_keys`, and `dod_5220_erase` over
    the allocator's extent. A region this answers `false` for is one the
    shutdown path does not reach; it then survives in `residue` instead of
    being quietly assumed gone. -/
def wipeCovers : Region → Bool
  | .processMemory => true
  | .kernelHeap => true
  | .kernelStacks => true
  | .fsCaches => true
  | .keyVault => true

/-- What still holds material in each phase: before the wipe everything, after
    it exactly the regions the wipe does not cover. -/
def residue : Phase → List Region
  | Running => liveRegions
  | Quiesced => liveRegions
  | Wiped => liveRegions.filter (fun r => !wipeCovers r)
  | Off => liveRegions.filter (fun r => !wipeCovers r)

/-- Does this phase still hold anything worth reading out of DRAM? Derived
    from `residue` rather than declared, so what a wiped machine holds depends
    on what the wipe actually covers. -/
def holdsSecrets (p : Phase) : Bool := !(residue p).isEmpty

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

/-- Every region a running system holds is one the wipe covers. This is the
    coverage half of the claim, and it is the half that used to be a
    definition: `holdsSecrets Wiped` was declared `false` rather than derived,
    so a region the wipe missed could not make any theorem fail. Two did, and
    the model said nothing: the heap erase addressed a window nothing maps, and
    the kernel stacks had no mechanism at all. -/
theorem wipe_covers_every_region (r : Region) (h : r ∈ liveRegions) :
    wipeCovers r = true := by
  cases r <;> rfl

/-- Nothing is left after the wipe. Now a consequence of the coverage above
    rather than a definition. -/
theorem wiped_residue_empty : residue Wiped = [] := by decide

/-- No powered off machine still holds secrets. The corollary a user actually
    cares about: pull the plug and the DRAM has nothing left in it. -/
theorem off_holds_nothing : holdsSecrets Off = false := by decide

/-- Turned around: while memory still holds secrets, the machine has not left.
    An attacker who wants to read DRAM has to catch a system that is still
    running, which is a different and much louder attack than lifting the
    chips out of a box that was shut down a minute ago. -/
theorem secrets_imply_still_here (p : Phase) (h : holdsSecrets p = true) :
    p ≠ Off := by
  intro hoff; rw [hoff] at h; exact absurd h (by decide)

/-- The wipe cannot be skipped: `Running` does not reach `Off` in one step. -/
theorem no_shortcut_to_off (h : Step Running Off) : False := by
  cases h

/-- Nor can quiescing alone stand in for it. Stopping the other CPUs makes the
    wipe sound; it does not make it unnecessary. -/
theorem quiescing_is_not_wiping (h : Step Quiesced Off) : False := by
  cases h

end Nonos.ZeroState
