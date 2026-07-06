/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the IPC message-length gate. The real kernel
`IpcMessage::new` bound and the Verus theorems in
`verification/verus/src/ipc_lengths.rs` implement it. Lean proves the property;
the Verus proof discharges it on the u64 length check that runs.
-/

namespace Nonos.Ipc

/-- The maximum IPC message payload (1 MiB), matching the kernel. -/
def maxMessageSize : Nat := 1024 * 1024

/-- A message length is valid iff it is non-empty and within the bound. -/
def ValidLen (len : Nat) : Prop := 0 < len ∧ len ≤ maxMessageSize

/-- A zero-length message is rejected. -/
theorem zero_length_rejected : ¬ ValidLen 0 := by
  unfold ValidLen; omega

/-- A message longer than the bound is rejected. -/
theorem oversized_rejected (len : Nat) (h : len > maxMessageSize) : ¬ ValidLen len := by
  unfold ValidLen; omega

/-- An accepted message is non-empty and within the bound. -/
theorem accepted_is_bounded (len : Nat) (h : ValidLen len) :
    0 < len ∧ len ≤ maxMessageSize := h

end Nonos.Ipc
