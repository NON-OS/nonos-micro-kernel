/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the keyring caller-identity resolver
(`capsule_keyring/server/caller.rs`). The kernel router stamps `proc.<pid>` on
every capsule-originated message, so a sender pid of 0 can only be a
kernel-internal delivery. The resolver accepts a request only when the pid the
caller asserts equals the pid the kernel attributes to the message, and never
for sender 0. This closes the identity boundary that ownership rests on: no
message can speak for a pid other than its real sender.
-/

namespace Nonos.WalletCallerPid

/-- The resolver: reject sender 0, then require the asserted pid to match the
    kernel-attributed sender. -/
def resolve (payload sender : Nat) : Option Nat :=
  if sender = 0 then none
  else if payload = sender then some sender else none

/-- A sender pid of 0 is always rejected: kernel-internal deliveries can never
    claim ownership. -/
theorem sender_zero_rejected (payload : Nat) : resolve payload 0 = none := by
  unfold resolve; simp

/-- A message whose asserted pid differs from its real sender is rejected: no
    spoofing a different owner. -/
theorem mismatch_rejected (payload sender : Nat) (hs : sender ≠ 0)
    (h : payload ≠ sender) : resolve payload sender = none := by
  unfold resolve; simp [hs, h]

/-- A genuine caller (nonzero sender, matching assertion) is accepted as
    exactly that sender. -/
theorem genuine_accepted (sender : Nat) (hs : sender ≠ 0) :
    resolve sender sender = some sender := by
  unfold resolve; simp [hs]

/-- Whenever the resolver accepts, the resolved pid is the real sender, never
    the payload-asserted value taken on trust. -/
theorem resolved_is_sender (payload sender pid : Nat)
    (h : resolve payload sender = some pid) : pid = sender := by
  unfold resolve at h
  by_cases hs : sender = 0
  · simp [hs] at h
  · by_cases hm : payload = sender
    · simp [hs, hm] at h; omega
    · simp [hs, hm] at h

end Nonos.WalletCallerPid
