/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of syscall id decoding. Userspace hands the kernel an untrusted
64-bit id; the decoder must be total (an unknown id is an error value, never a
panic) and faithful to the registry (a known id decodes to exactly its own
entry). `userland/kernel_proofs` discharges this on the real decoder:
`syscall_decode_is_total` and `syscall_id_decode_and_registry_agree_for_all_ids`
(Kani, all `u64`), and `decode_is_total_and_agrees_with_the_name_table`,
`decode_is_deterministic`, `known_syscalls_round_trip_through_their_numeric_id`
in `syscall_tests.rs`.
-/

namespace Nonos.Syscall

/-- The registry: a table of syscall ids. Decoding is a lookup; ids are the
    key, so a well-formed registry never repeats one. -/
def decode (table : List Nat) (id : Nat) : Option Nat :=
  if id ∈ table then some id else none

/-- Decoding is total by case: every id, over the whole 64-bit range, either
    decodes to itself as a known syscall or to the explicit unknown value.
    There is no third outcome to panic through. -/
theorem decode_is_total (table : List Nat) (id : Nat) :
    decode table id = some id ∨ decode table id = none := by
  unfold decode
  by_cases h : id ∈ table <;> simp [h]

/-- A known id round-trips: decoding gives back exactly the id looked up,
    never a different registry entry. -/
theorem known_ids_round_trip (table : List Nat) (id : Nat)
    (h : id ∈ table) : decode table id = some id := by
  simp [decode, h]

/-- The decoder and the registry agree everywhere: an id decodes if and only
    if the registry lists it. An id outside the table can never reach a
    handler, and a listed id can never be turned away. -/
theorem decode_agrees_with_the_registry (table : List Nat) (id : Nat) :
    (decode table id).isSome ↔ id ∈ table := by
  unfold decode
  by_cases h : id ∈ table <;> simp [h]

end Nonos.Syscall
