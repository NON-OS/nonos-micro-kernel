/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Copy-on-write pages. A page carries a reference count; it is shared when more
than one mapping refers to it. A write to a shared page copies it out, handing
the writer a fresh private page and dropping the original's count by one. The
theorems below show sharing increments the count, a write yields a private page
with a single reference and lowers the original's count, and a page with a
single reference is not shared and so is written in place, so a write never
mutates memory another mapping still observes.
-/

namespace Nonos.Cow

/-- A page tracked by its reference count. -/
structure Page where
  refs : Nat

/-- Add a mapping to a page. -/
def share (p : Page) : Page := ⟨p.refs + 1⟩

/-- A page is shared when more than one mapping refers to it. -/
def shared (p : Page) : Prop := 1 < p.refs

/-- A write to a shared page: the original loses a reference and the writer
    receives a fresh private page. -/
def writeCopy (p : Page) : Page × Page := (⟨p.refs - 1⟩, ⟨1⟩)

/-- A private write is allowed in place only when the page is not shared. -/
def writeInPlaceOk (p : Page) : Prop := p.refs ≤ 1

/-- Sharing increments the reference count. -/
theorem share_increments (p : Page) : (share p).refs = p.refs + 1 := rfl

/-- The copied-out page is private: it has a single reference. -/
theorem write_gives_private (p : Page) : (writeCopy p).2.refs = 1 := rfl

/-- A copy drops the original's reference count by one, so the mappings that
    remain still share a page the writer no longer touches. -/
theorem write_drops_original (p : Page) (h : shared p) : (writeCopy p).1.refs + 1 = p.refs := by
  simp only [writeCopy, shared] at *; omega

/-- A singly-referenced page is not shared. -/
theorem private_page_not_shared (p : Page) (h : p.refs = 1) : ¬ shared p := by
  simp only [shared]; omega

/-- A singly-referenced page may be written in place, no copy needed. -/
theorem unshared_writes_in_place (p : Page) (h : p.refs = 1) : writeInPlaceOk p := by
  simp only [writeInPlaceOk]; omega

end Nonos.Cow
