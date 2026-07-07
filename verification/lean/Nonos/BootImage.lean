/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the boot image footer parser. The bootloader reads a footer
naming the kernel, signature, and proof regions of an attacker-supplied image;
the security property is that a region the parser hands back can only describe
bytes inside the buffer it was given. `nonos-bootloader/boot_proofs` discharges
this on the real `image_format` parser: `parse_footer_is_total_and_in_bounds`
(Kani, every 72-byte footer), `parse_never_escapes_the_buffer_over_crafted_footers`
(~125k adversarial footers), and `parse_never_panics_on_degenerate_input`. The
code proofs also rule out arithmetic wraparound in the machine integers, which
is what entitles this model to plain natural-number arithmetic.
-/

namespace Nonos.BootImage

/-- A region the footer names: a byte offset and length into the image. -/
structure Region where
  off : Nat
  len : Nat

/-- The bound the parser enforces before returning a region: it must end at or
    before the end of the `n`-byte buffer. -/
def inBounds (n : Nat) (r : Region) : Bool := r.off + r.len ≤ n

/-- The parser model: return the named regions only if every one lies inside
    the buffer, otherwise reject. The real parser checks each region the same
    way before slicing. -/
def parse (n : Nat) (rs : List Region) : Option (List Region) :=
  if rs.all (inBounds n) then some rs else none

/-- Acceptance is exactly the bounds check: the parser returns regions if and
    only if every region lies inside the buffer. -/
theorem accepted_iff_all_in_bounds (n : Nat) (rs : List Region) :
    parse n rs = some rs ↔ rs.all (inBounds n) = true := by
  unfold parse
  by_cases h : rs.all (inBounds n) = true <;> simp [h]

/-- Every region an accepted parse returns stays inside the buffer: no slice
    the bootloader takes from the footer can escape the image. -/
theorem accepted_region_stays_in_bounds (n : Nat) (rs out : List Region)
    (h : parse n rs = some out) : ∀ r ∈ out, r.off + r.len ≤ n := by
  unfold parse at h
  by_cases hall : rs.all (inBounds n) = true
  · simp [hall] at h
    subst h
    intro r hr
    have := List.all_eq_true.mp hall r hr
    simpa [inBounds] using this
  · simp [hall] at h

/-- A footer naming any region that escapes the buffer is rejected whole: one
    bad region poisons the parse, there is no partial acceptance. -/
theorem an_escaping_region_is_rejected (n : Nat) (rs : List Region)
    (r : Region) (hr : r ∈ rs) (hesc : n < r.off + r.len) :
    parse n rs = none := by
  unfold parse
  have hfalse : rs.all (inBounds n) = false := by
    cases hall : rs.all (inBounds n) with
    | false => rfl
    | true =>
      have hb : r.off + r.len ≤ n := by
        simpa [inBounds] using List.all_eq_true.mp hall r hr
      exact absurd hb (by omega)
  simp [hfalse]

end Nonos.BootImage
