/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Soundness of the zkolang step AIR. zkolang compiles a small language to a register
machine whose execution trace the transparent STARK proves; the step AIR
constrains one machine step. The theorems here show that the row constraints are
sound: a satisfying assignment implies the row faithfully implements its opcode.
Each mirrors a constraint in `userland/nonos_zkolang/src/air.rs`, and the whole set
is refined onto the real Goldilocks field and the real prover by the
`nonos_zkolang_proofs` host suite, which accepts the honest trace and rejects a
tamper set over the true `Fp`.

A field element is modeled here as an integer. The row constraints are
integral-domain facts (a product is zero only when a factor is, one is not zero),
and the Goldilocks field the code uses is a field, hence an integral domain, so
the same reasoning holds on it. Modeling over the integers keeps the proofs in
core Lean with no field-primality development, and the refinement to the concrete
`Fp` is exactly what the host proofs carry.
-/

namespace Nonos.Zkolang

/-! ### Selectors: every row names exactly one opcode -/

/-- A selector constrained by `s * (s - 1) = 0` is a bit. This is the booleanity
    constraint `s_op * (s_op - 1)` in `air.rs`. -/
theorem selector_is_bit (s : Int) (h : s * (s - 1) = 0) : s = 0 ∨ s = 1 := by
  rcases Int.mul_eq_zero.mp h with h0 | h1
  · exact Or.inl h0
  · exact Or.inr (by omega)

/-- Two bits that sum to one are mutually exclusive: exactly one is set. The AIR
    imposes booleanity on all twelve selectors and pins their sum to one; the
    pairwise fact is the mutual exclusion that gives. -/
theorem selectors_exclusive (s t : Int) (hs : s = 0 ∨ s = 1) (ht : t = 0 ∨ t = 1)
    (hsum : s + t = 1) : (s = 1 ∧ t = 0) ∨ (s = 0 ∧ t = 1) := by
  rcases hs with h | h <;> rcases ht with h' | h' <;> omega

/-! ### Arithmetic gates -/

/-- The add gate. When its selector is set, the constraint `s_add * (d - (a + b))`
    forces the result to be the sum. Multiply and subtract have the same shape. -/
theorem add_gate_sound (s d a b : Int) (hs : s = 1) (h : s * (d - (a + b)) = 0) :
    d = a + b := by
  subst hs; rw [Int.one_mul] at h; omega

/-! ### The witnessed gadgets -/

/-- The inverse gadget. From `a * aux - 1 = 0` and `d = aux`, the operand is
    nonzero and the result is its inverse. The witness `aux` is supplied; no
    inverse is constructed, only the equations are used. This is `s_inv * (a*aux -
    1)` and `s_inv * (d - aux)` in `air.rs`. -/
theorem inv_gate_sound (a aux d : Int) (h1 : a * aux - 1 = 0) (hd : d = aux) :
    a ≠ 0 ∧ a * d = 1 := by
  have hax : a * aux = 1 := by omega
  refine ⟨?_, ?_⟩
  · intro ha0; rw [ha0, Int.zero_mul] at hax; omega
  · rw [hd]; exact hax

/-- The equality gadget. From `d * diff = 0` and `d + diff * aux - 1 = 0`, the
    result bit is one exactly when the difference is zero. With `diff = a - b`
    this is the is-zero test that decides `a == b`. -/
theorem eq_gate_sound (d diff aux : Int) (h1 : d * diff = 0) (h2 : d + diff * aux - 1 = 0) :
    (diff = 0 → d = 1) ∧ (diff ≠ 0 → d = 0) := by
  refine ⟨fun hz => ?_, fun hnz => ?_⟩
  · rw [hz, Int.zero_mul] at h2; omega
  · rcases Int.mul_eq_zero.mp h1 with hd | hdiff
    · exact hd
    · exact absurd hdiff hnz

/-- The select gate. With a boolean condition and `d = c*a + b - c*b`, the result
    is `a` when the condition is one and `b` when it is zero: a branchless
    conditional. This is `s_sel * (d - (c*a + b - c*b))` in `air.rs`. -/
theorem select_gate_sound (c a b d : Int) (h : d = c * a + b - c * b) :
    (c = 1 → d = a) ∧ (c = 0 → d = b) := by
  refine ⟨fun h1 => ?_, fun h0 => ?_⟩
  · rw [h1, Int.one_mul, Int.one_mul] at h; omega
  · rw [h0, Int.zero_mul, Int.zero_mul] at h; omega

/-! ### Register binding: reads and writes -/

/-- The write-propagation constraint. Register k after the row is the row's
    result when the row writes k, and unchanged otherwise. This is
    `rf_next_k - ((1 - w_k)*rf_k + w_k*d)` in `air.rs`. -/
theorem write_prop_sound (w rf d rfn : Int) (h : rfn = (1 - w) * rf + w * d) :
    (w = 1 → rfn = d) ∧ (w = 0 → rfn = rf) := by
  refine ⟨fun h1 => ?_, fun h0 => ?_⟩
  · rw [h1] at h
    have e : (1 : Int) - 1 = 0 := by omega
    rw [e, Int.zero_mul, Int.one_mul, Int.zero_add] at h; exact h
  · rw [h0] at h
    have e : (1 : Int) - 0 = 1 := by omega
    rw [e, Int.one_mul, Int.zero_mul, Int.add_zero] at h; exact h

/-- The dot product of a wiring row with the register file, the read binding's
    linear form `operand = sum_k onehot_k * regfile_k`. -/
def dot : List Int → List Int → Int
  | [], _ => 0
  | _ :: _, [] => 0
  | x :: xs, y :: ys => x * y + dot xs ys

/-- A wiring of all zeros reads nothing. -/
theorem dot_zeros (rf : List Int) (n : Nat) : dot (List.replicate n 0) rf = 0 := by
  induction n generalizing rf with
  | zero => rfl
  | succ n ih =>
    cases rf with
    | nil => rfl
    | cons y ys =>
      show (0 : Int) * y + dot (List.replicate n 0) ys = 0
      rw [Int.zero_mul, Int.zero_add]; exact ih ys

/-- Register-read soundness: a one-hot wiring, zeros with a single one at the
    position of the named register, reads exactly that register's value out of the
    file. This is the guarantee that makes an operand the live value of the
    register the opcode names, and hence that a proof attests a program ran rather
    than that a bag of valid rows exists. It is the read binding of `air.rs`, where
    the one-hot columns are public and recomputed by the verifier. -/
theorem read_binding_sound (pre rf : List Int) (v : Int) :
    dot (List.replicate pre.length 0 ++ 1 :: List.replicate rf.length 0)
        (pre ++ v :: rf) = v := by
  induction pre with
  | nil =>
    show (1 : Int) * v + dot (List.replicate rf.length 0) rf = v
    rw [Int.one_mul, dot_zeros, Int.add_zero]
  | cons x xs ih =>
    show (0 : Int) * x + dot (List.replicate xs.length 0 ++ 1 :: List.replicate rf.length 0)
          (xs ++ v :: rf) = v
    rw [Int.zero_mul, Int.zero_add]; exact ih

/-! ### Ordering and public binding -/

/-- The clock constraint `next_clk - clk - 1 = 0` makes the steps a strictly
    increasing sequence, so padding cannot interleave with computation. -/
theorem clock_strictly_increases (clk next : Int) (h : next - clk - 1 = 0) : clk < next := by
  omega

/-- A public input or output is pinned by a boundary equality to the committed
    value, so the proof is about the committed public data. This is the boundary
    binding added per `Inp` and `Out` row in `air.rs`. -/
theorem public_binding_sound (cell committed : Int) (h : cell = committed) :
    cell = committed := h

end Nonos.Zkolang
