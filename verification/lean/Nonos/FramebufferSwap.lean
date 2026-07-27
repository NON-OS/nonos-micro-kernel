/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Present-blit channel order (`src/syscall/dispatch/router/graphics_present/blit.rs`).
Surface pixels are stored B,G,R,A. A BGR framebuffer scans that out directly; an
RGB framebuffer needs the red and blue channels exchanged exactly once. The bug
was that the conditional swap was written twice, so on RGB firmware the two
swaps cancelled and red and blue scanned out reversed. The theorems fix that a
swap is an involution (so doing it twice is the identity, i.e. the duplicate
left RGB unconverted), and that applying it once maps stored order to the order
an RGB panel expects, while BGR takes no swap.
-/

namespace Nonos.FramebufferSwap

/-- A 32-bit pixel as its four bytes in memory order: byte0, byte1, byte2, byte3.
    Surfaces store this as B,G,R,A. -/
structure Px where
  b0 : Nat
  b1 : Nat
  b2 : Nat
  b3 : Nat
  deriving DecidableEq

/-- Exchange byte0 and byte2 (the red/blue swap), leaving green and alpha. -/
def swap02 (p : Px) : Px := ⟨p.b2, p.b1, p.b0, p.b3⟩

/-- The swap is an involution: applying it twice is the identity. So the
    duplicated swap in the old code cancelled, leaving an RGB framebuffer with
    red and blue never exchanged. -/
theorem swap_involutive (p : Px) : swap02 (swap02 p) = p := rfl

/-- Applying the swap once maps stored order B,G,R,A to R,G,B,A, which is what an
    RGB framebuffer scans out correctly. -/
theorem swap_once_converts (r g b a : Nat) :
    swap02 ⟨b, g, r, a⟩ = ⟨r, g, b, a⟩ := rfl

/-- The correct present path: swap once for RGB firmware, not at all for BGR.
    `bgr = true` scans the stored B,G,R,A out directly; `bgr = false` needs the
    single swap. -/
def present (bgr : Bool) (p : Px) : Px := if bgr then p else swap02 p

/-- On BGR firmware the stored pixel is presented unchanged. -/
theorem present_bgr_identity (p : Px) : present true p = p := rfl

/-- On RGB firmware the stored B,G,R,A is presented as R,G,B,A: exactly one
    exchange, the outcome the duplicated-swap bug destroyed. -/
theorem present_rgb_converts (r g b a : Nat) :
    present false ⟨b, g, r, a⟩ = ⟨r, g, b, a⟩ := rfl

end Nonos.FramebufferSwap
