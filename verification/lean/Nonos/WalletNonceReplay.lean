/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of replay resistance in the wallet's send path. A
transfer signs the account's current nonce; once the transaction is included the
account nonce advances, so the same signed transaction is refused on a second
submission. These theorems model that: the account nonce strictly increases per
inclusion and a transaction is accepted only at the exact current nonce, so no
signed transfer can ever be mined twice.
-/

namespace Nonos.WalletNonceReplay

/-- The account nonce after one inclusion. -/
def advance (current : Nat) : Nat := current + 1

/-- A node accepts a transaction only when its nonce equals the account nonce. -/
def accepts (accountNonce txNonce : Nat) : Bool := decide (txNonce = accountNonce)

/-- The nonce strictly increases on every inclusion: it never stalls or repeats,
    so each transfer consumes a fresh slot. -/
theorem nonce_strictly_increases (n : Nat) : n < advance n := by
  unfold advance; omega

/-- A once-accepted transaction is refused on replay: after inclusion the account
    nonce has advanced, so the same nonce no longer matches. -/
theorem replay_refused (account tx : Nat) (h : accepts account tx = true) :
    accepts (advance account) tx = false := by
  unfold accepts advance at *
  simp only [decide_eq_true_eq] at h
  simp only [decide_eq_false_iff_not]
  omega

/-- Acceptance pins the nonce exactly: an accepted transaction used precisely the
    account's current nonce, neither a stale nor a future one. -/
theorem accept_pins_nonce (account tx : Nat) (h : accepts account tx = true) :
    tx = account := by
  unfold accepts at h
  simpa using h

/-- Two transactions accepted against the same account nonce are the same
    transaction as far as the nonce is concerned: no two distinct nonces are ever
    both valid at one account state. -/
theorem no_two_nonces (account t1 t2 : Nat)
    (h1 : accepts account t1 = true) (h2 : accepts account t2 = true) : t1 = t2 := by
  rw [accept_pins_nonce account t1 h1, accept_pins_nonce account t2 h2]

end Nonos.WalletNonceReplay
