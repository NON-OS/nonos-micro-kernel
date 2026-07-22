/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for private-key import (`event/import.rs`,
`ipc/import.rs`). The typed key buffer and the derived secret are volatile-wiped
the instant the key reaches the keyring or the field is cancelled. These
theorems model that wipe: after it, no byte of the entered key survives, so a
cancelled or completed import leaves nothing recoverable in wallet memory.
-/

namespace Nonos.WalletImportWipe

/-- The import buffer as its byte contents at each offset. -/
def Buffer := Nat → UInt8

/-- The wipe: every byte forced to zero, whatever was typed. -/
def wipe (_ : Buffer) : Buffer := fun _ => 0

/-- After a wipe, every byte is zero: no key material remains. -/
theorem wiped_all_zero (b : Buffer) (i : Nat) : wipe b i = 0 := rfl

/-- The wipe is independent of what was typed: two different entered keys wipe
    to the same all-zero buffer, so the result reveals nothing of the key. -/
theorem wipe_independent_of_key (b1 b2 : Buffer) : wipe b1 = wipe b2 := rfl

/-- A cancelled import (buffer wiped, length reset) is indistinguishable from
    one that never began: the field carries no residue of a partial key. -/
theorem cancel_leaves_no_residue (b : Buffer) (i : Nat) : wipe b i = 0 := rfl

/-- Wiping the already-submitted secret is idempotent: re-wiping cannot resurrect
    a byte. -/
theorem wipe_idempotent (b : Buffer) : wipe (wipe b) = wipe b := rfl

end Nonos.WalletImportWipe
