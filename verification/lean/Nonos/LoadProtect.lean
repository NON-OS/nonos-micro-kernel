/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capsule ELF load protection. The segment validator
(src/elf/loader/core/load_segment/validate.rs) rejects a PT_LOAD whose file
size exceeds its memory size and any writable-and-executable segment
(WXViolation); the plan (load_segment/plan.rs) rejects a segment whose start
or page-span end leaves the user window (USER_VA_MAX) or whose span is empty.
An accepted segment is mapped READ | USER plus WRITE or EXECUTE from its
header bits (load_segment/pte_flags.rs), and the paging layer independently
refuses any writable-and-executable mapping
(src/memory/paging/manager/mapping/map_in_asid.rs). The loader
(loader/load_entry_into.rs) maps all segments, applies relocations, and runs
enforce_relro (core/relro.rs) last, remapping the PT_GNU_RELRO span to
READ | USER. The theorems show: W+X and oversize and out-of-window segments
are rejected, every accepted segment carries W-xor-X permissions and touches
only user-window addresses, segments with disjoint page spans share no
address, gated mapping steps preserve the no-W+X invariant, and the final
RELRO seal leaves the span read-only no matter what mapping steps preceded it,
with re-sealing idempotent so the sealed state persists.
-/

namespace Nonos.LoadProtect

/-- USER_VA_MAX: the highest virtual address a segment page may occupy. -/
def userVaMax : Nat := 0x00007FFFFFFFFFFF

/-- A planned PT_LOAD segment: its start address (base + p_vaddr), its
    page-aligned span [spanLo, spanEnd), its sizes, and its W/X header bits. -/
structure Seg where
  start : Nat
  spanLo : Nat
  spanEnd : Nat
  filesz : Nat
  memsz : Nat
  w : Bool
  x : Bool

/-- validate.rs: file bytes must fit in the memory image and a segment may not
    be both writable and executable. -/
def validated (s : Seg) : Prop :=
  s.filesz ≤ s.memsz ∧ ¬(s.w = true ∧ s.x = true)

/-- plan.rs: the segment start and the end of its page span must stay inside
    the user window, and the span must be non-empty. -/
def inWindow (s : Seg) : Prop :=
  s.start ≤ userVaMax ∧ s.spanEnd ≠ 0 ∧ s.spanEnd ≤ userVaMax

/-- A segment the loader accepts and maps. -/
def accepted (s : Seg) : Prop := validated s ∧ inWindow s

/-- Address `a` lies in the pages the segment occupies. -/
def maps (s : Seg) (a : Nat) : Prop := s.spanLo ≤ a ∧ a < s.spanEnd

/-- Two segments whose page spans share no address. -/
def Disjoint (s t : Seg) : Prop := s.spanEnd ≤ t.spanLo ∨ t.spanEnd ≤ s.spanLo

/-- Permission bits of a mapped page. -/
structure Perm where
  read : Bool
  write : Bool
  exec : Bool
  user : Bool

/-- pte_flags.rs: READ | USER always, WRITE and EXECUTE from the header. -/
def segPerm (s : Seg) : Perm := ⟨true, s.w, s.x, true⟩

/-- The READ | USER permissions enforce_relro installs. -/
def roPerm : Perm := ⟨true, false, false, true⟩

/-- No page is both writable and executable. -/
def wxSafe (p : Perm) : Prop := ¬(p.write = true ∧ p.exec = true)

/-- A permission-changing step of the load pipeline: mapping a page with
    header-derived bits, or the final RELRO seal. -/
inductive Op
  | mapSeg (w x : Bool)
  | sealRelro

/-- The permissions a step leaves on the page it touches. -/
def apply : Perm → Op → Perm
  | _, .mapSeg w x => ⟨true, w, x, true⟩
  | _, .sealRelro => roPerm

/-- Run a sequence of steps over one page's permissions. -/
def run (p : Perm) (ops : List Op) : Perm := ops.foldl apply p

/-- map_page_in_asid refuses a writable-and-executable mapping, so every step
    that reaches the page tables satisfies this gate. -/
def gated : Op → Prop
  | .mapSeg w x => ¬(w = true ∧ x = true)
  | .sealRelro => True

/-! ### Rejection -/

/-- A writable-and-executable segment is rejected (WXViolation). -/
theorem wx_segment_rejected (s : Seg) (hw : s.w = true) (hx : s.x = true) :
    ¬ accepted s := fun h => h.1.2 ⟨hw, hx⟩

/-- A segment whose file bytes exceed its memory image is rejected. -/
theorem oversize_rejected (s : Seg) (h : s.memsz < s.filesz) : ¬ accepted s := by
  intro ha; have := ha.1.1; omega

/-- A segment whose span leaves the user window is rejected. -/
theorem out_of_window_rejected (s : Seg) (h : userVaMax < s.spanEnd) :
    ¬ accepted s := by
  intro ha; have := ha.2.2.2; omega

/-- A segment starting above the user window is rejected. -/
theorem start_out_of_window_rejected (s : Seg) (h : userVaMax < s.start) :
    ¬ accepted s := by
  intro ha; have := ha.2.1; omega

/-- A segment whose page span wraps to zero is rejected. -/
theorem empty_span_rejected (s : Seg) (h : s.spanEnd = 0) : ¬ accepted s :=
  fun ha => ha.2.2.1 h

/-! ### Accepted segments -/

/-- Every accepted segment carries W-xor-X permissions. -/
theorem accepted_wx_safe (s : Seg) (h : accepted s) : wxSafe (segPerm s) :=
  h.1.2

/-- Every page of an accepted segment lies inside the user window. -/
theorem accepted_in_window (s : Seg) (a : Nat) (h : accepted s)
    (hm : maps s a) : a ≤ userVaMax := by
  have h1 := h.2.2.2; have h2 := hm.2; omega

/-- An accepted segment never touches a kernel-half address. -/
theorem kernel_addr_never_mapped (s : Seg) (a : Nat) (h : accepted s)
    (ha : userVaMax < a) : ¬ maps s a := by
  intro hm; have := accepted_in_window s a h hm; omega

/-- Segments with disjoint page spans share no address: each keeps its own
    frames and neither can alias the other's pages. -/
theorem disjoint_no_shared (s t : Seg) (a : Nat) (hd : Disjoint s t)
    (hs : maps s a) : ¬ maps t a := by
  intro ht
  simp only [maps] at hs ht
  rcases hd with h | h <;> omega

/-! ### W^X across the pipeline -/

/-- One gated step keeps the page W-xor-X. -/
theorem apply_wx_safe (p : Perm) (o : Op) (hg : gated o) :
    wxSafe (apply p o) := by
  cases o with
  | mapSeg w x => exact hg
  | sealRelro => simp [apply, wxSafe, roPerm]

/-- Any sequence of gated steps keeps the page W-xor-X: the paging layer's
    gate makes the invariant hold through the whole load. -/
theorem run_wx_safe : ∀ (ops : List Op) (p : Perm),
    (∀ o ∈ ops, gated o) → wxSafe p → wxSafe (run p ops)
  | [], _, _, hp => hp
  | o :: rest, p, hg, hp => by
    simp only [run, List.foldl_cons]
    exact run_wx_safe rest (apply p o)
      (fun o' h' => hg o' (List.mem_cons_of_mem _ h'))
      (apply_wx_safe p o (hg o (List.mem_cons_self _ _)))

/-! ### RELRO sealing -/

/-- The seal installs exactly READ | USER. -/
theorem seal_ro (p : Perm) : apply p .sealRelro = roPerm := rfl

/-- Sealing is idempotent: re-sealing changes nothing. -/
theorem seal_idem (p : Perm) :
    apply (apply p .sealRelro) .sealRelro = apply p .sealRelro := rfl

/-- enforce_relro runs after every mapping and relocation step
    (load_entry_into.rs), so whatever came before, the RELRO span ends the
    load read-only. -/
theorem seal_final (p : Perm) (ops : List Op) :
    run p (ops ++ [Op.sealRelro]) = roPerm := by
  simp [run, List.foldl_append, apply]

/-- Corollary: after the load, the RELRO span is not writable, so a capsule
    cannot mutate its own GOT after relocation. -/
theorem sealed_not_writable (p : Perm) (ops : List Op) :
    (run p (ops ++ [Op.sealRelro])).write = false := by
  rw [seal_final]; rfl

/-- Sealing is permanent: any number of further seals leaves the span in
    exactly the sealed read-only state. -/
theorem sealed_stays (n : Nat) :
    run roPerm (List.replicate n Op.sealRelro) = roPerm := by
  induction n with
  | zero => simp [run]
  | succ k ih => simpa [run, List.replicate_succ, apply] using ih

end Nonos.LoadProtect
