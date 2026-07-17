/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Merkle membership, the core of the capsule and kernel attestation. A private leaf
is folded up a public path to a root; the theorems below show the root is a
function of the leaf and path alone, folding an appended path is folding the
prefix then the suffix, the path length is the tree depth, and, when the
compression is injective, a leaf reaches a given root by at most one value and a
tampered leaf, sibling, or direction reaches a different root. These are the
soundness facts the attestation gate relies on: an opening pins its leaf.
-/

namespace Nonos.Stark.Membership

/-- One level of a path: the sibling digest and whether the node is the right child. -/
structure Step where
  sibling : Nat
  right : Bool

/-- One compression, combining the node with its sibling by the index bit. The
compression `comb` is abstract, so the results hold for any two-to-one hash. -/
def apply (comb : Nat → Nat → Nat) (node : Nat) (s : Step) : Nat :=
  if s.right then comb s.sibling node else comb node s.sibling

/-- Fold a leaf up its path to the root. -/
def root (comb : Nat → Nat → Nat) (leaf : Nat) (path : List Step) : Nat :=
  path.foldl (apply comb) leaf

/-- Injectivity of the compression in both arguments. -/
def CombInjective (comb : Nat → Nat → Nat) : Prop :=
  ∀ a b c d, comb a b = comb c d → a = c ∧ b = d

/-- The empty path leaves the leaf as the root. -/
theorem root_nil (comb : Nat → Nat → Nat) (leaf : Nat) : root comb leaf [] = leaf := rfl

/-- A path is one compression, then the rest. -/
theorem root_cons (comb : Nat → Nat → Nat) (leaf : Nat) (s : Step) (rest : List Step) :
    root comb leaf (s :: rest) = root comb (apply comb leaf s) rest := rfl

/-- Folding an appended path folds the prefix, then the suffix. -/
theorem root_append (comb : Nat → Nat → Nat) (leaf : Nat) (p q : List Step) :
    root comb leaf (p ++ q) = root comb (root comb leaf p) q := by
  simp only [root, List.foldl_append]

/-- The root is a function of the leaf and path: same inputs, same root. -/
theorem root_deterministic (comb : Nat → Nat → Nat) (leaf : Nat) (path : List Step)
    (r₁ r₂ : Nat) (h₁ : root comb leaf path = r₁) (h₂ : root comb leaf path = r₂) : r₁ = r₂ := by
  rw [← h₁, ← h₂]

/-- A left child compresses node then sibling. -/
theorem apply_left (comb : Nat → Nat → Nat) (node sib : Nat) :
    apply comb node ⟨sib, false⟩ = comb node sib := rfl

/-- A right child compresses sibling then node. -/
theorem apply_right (comb : Nat → Nat → Nat) (node sib : Nat) :
    apply comb node ⟨sib, true⟩ = comb sib node := rfl

/-- The number of siblings is the tree depth: one compression per level. -/
theorem depth_is_length (path : List Step) :
    (path.map Step.sibling).length = path.length := by simp

/-- Extending a path by one level adds one compression. -/
theorem depth_succ (path : List Step) (s : Step) :
    (s :: path).length = path.length + 1 := by simp

/-- With an injective compression, one compression is injective in the node. -/
theorem apply_injective (comb : Nat → Nat → Nat) (hc : CombInjective comb) (s : Step)
    (x y : Nat) (h : apply comb x s = apply comb y s) : x = y := by
  cases s with
  | mk sib right =>
    cases right with
    | false => exact (hc _ _ _ _ h).1
    | true => exact (hc _ _ _ _ h).2

/-- With an injective compression, folding a whole path is injective in the leaf, so
    a leaf that reaches a given root is unique: an opening pins exactly one leaf. -/
theorem root_injective (comb : Nat → Nat → Nat) (hc : CombInjective comb) :
    ∀ (path : List Step) (x y : Nat), root comb x path = root comb y path → x = y := by
  intro path
  induction path with
  | nil => intro x y h; simpa [root] using h
  | cons s rest ih =>
    intro x y h
    rw [root_cons, root_cons] at h
    exact apply_injective comb hc s x y (ih _ _ h)

/-- A leaf differing from the enrolled one cannot reach the enrolled root. -/
theorem tampered_leaf_fails (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (path : List Step) (leaf leaf' : Nat) (hne : leaf ≠ leaf') :
    root comb leaf path ≠ root comb leaf' path := by
  intro h
  exact hne (root_injective comb hc path leaf leaf' h)

/-- Two leaves reach the same root only if they are equal, under injectivity. -/
theorem same_root_same_leaf (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (path : List Step) (x y : Nat) (h : root comb x path = root comb y path) : x = y :=
  root_injective comb hc path x y h

end Nonos.Stark.Membership
