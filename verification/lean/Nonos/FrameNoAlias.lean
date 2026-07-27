/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Physical frame allocation single-source (`src/memory/frame_alloc/types/ops.rs`).
The old allocator kept a bump fallback over a range the `phys` bitmap already
owned, so once the bitmap filled it handed out frames that were already live,
and free routed the wrong bitmap bit. The fix removes the fallback: `phys` is
the sole source, and exhaustion returns none, which every caller treats as an
allocation failure. The theorems fix that the bump fallback can alias an
already-allocated frame (the bug), that drawing only from the free set never
aliases and yields distinct frames across allocations (the fix), and that the
fixed allocator returns none on exhaustion rather than a frame from an
overlapping pool.
-/

namespace Nonos.FrameNoAlias

/-- The bump fallback returns `base + counter` without consulting the bitmap. -/
def bumpFrame (base counter : Nat) : Nat := base + counter

/-- The bug: because the bump range overlaps the bitmap's own pool, the fallback
    can return a frame that is already allocated. Here frame 16 is live and the
    bump path hands it out a second time (double-alloc), and a later free would
    clear the wrong owner's bit (double-free). -/
theorem bump_aliases :
    ∃ (used : List Nat) (base counter : Nat), bumpFrame base counter ∈ used :=
  ⟨[16], 16, 0, by simp [bumpFrame]⟩

/-- The fixed allocator draws only a frame the bitmap reports free. -/
def allocOne (used : List Nat) (f : Nat) (_h : f ∉ used) : Nat × List Nat := (f, f :: used)

/-- The frame handed out is not among those already allocated: no aliasing. -/
theorem alloc_no_alias (used : List Nat) (f : Nat) (h : f ∉ used) :
    (allocOne used f h).1 ∉ used := h

/-- Two successive allocations hand out distinct frames. -/
theorem two_allocs_distinct (used : List Nat) (f g : Nat)
    (_hf : f ∉ used) (hg : g ∉ f :: used) : g ≠ f := by
  intro he; apply hg; rw [he]; simp

/-- The fixed allocator: whatever the bitmap reports (`some` free frame or
    `none`) is the whole answer, with no fallback appended. -/
def fixedAlloc (free : Option Nat) : Option Nat := free

/-- The old allocator: on an empty bitmap it fell through to a bump frame. -/
def oldAlloc (free : Option Nat) (bump : Nat) : Option Nat :=
  match free with
  | some f => some f
  | none => some bump

/-- On exhaustion the fixed allocator returns none: there is no second source. -/
theorem fixed_none_on_empty : fixedAlloc none = none := rfl

/-- On exhaustion the old allocator handed out a bump frame, and that frame can
    be one already in use (`bump_aliases`): this is exactly the aliasing the fix
    removes. -/
theorem old_hands_bump_on_empty (bump : Nat) : oldAlloc none bump = some bump := rfl

end Nonos.FrameNoAlias
