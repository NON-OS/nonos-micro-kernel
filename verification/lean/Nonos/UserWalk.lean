/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The walk that decides whether userspace may touch an address.

`src/usercopy/walk/levels.rs` translates a user virtual address by reading
four levels through the directmap, and `access.rs` is the only caller: every
byte transfer goes through `translate_read` or `translate_write`, never the
walker directly. If this composition is wrong, `copy_from_user` reads kernel
memory into a capsule and `copy_to_user` writes capsule bytes into the kernel.
There is nothing downstream that would notice.

The subtlety is that the walk can terminate at three different levels, and
which checks apply depends on where it stops. A 1 GiB block ends the walk at
level 3, a 2 MiB block at level 2, otherwise at level 1. At each stop the
entry that terminates the walk is a leaf, not a table, so `table_grants_user`
must not be applied to it: its permission is `is_user`, which `access.rs`
applies. The checks that look skipped after a block descriptor are exactly the
ones that would be wrong to run.

What is proven here is the composition: whichever level a leaf comes from,
every table descriptor above it granted user, and a leaf that `access.rs`
accepts is itself user. So the whole path is user accessible, not just its
last step. `translate_write` additionally requires the leaf writable.

The model mirrors `walk` branch for branch and in its order. Two entry
predicates are kept separate because the architectures differ: x86_64
intersects permissions down the hierarchy so a table without the user bit
denies EL0 whatever the leaf says, while an aarch64 table descriptor restricts
nothing unless its hierarchical bits do. `tableUser` is the first, `user` the
second, matching `table_grants_user` and `is_user`.
-/

namespace Nonos.UserWalk

/-- A page-table entry, as the walk tests it. -/
structure Entry where
  present : Bool
  block : Bool
  /-- `is_user`: the leaf permission. -/
  user : Bool
  /-- `table_grants_user`: whether this table descriptor lets EL0 through. -/
  tableUser : Bool
  /-- `is_writable`. -/
  writable : Bool
  deriving Repr

/-- Where the walk stopped. -/
inductive Level where
  | l3
  | l2
  | l1
  deriving DecidableEq, Repr

/-- Why a walk failed, matching `UsercopyError`. -/
inductive Err where
  | notMapped
  | notUser
  | notWritable
  deriving DecidableEq, Repr

/-- A successful walk: the level it stopped at and the entry it returned. -/
structure Leaf where
  level : Level
  entry : Entry
  deriving Repr

/-- The four entries a walk reads for one address. -/
structure Path where
  e4 : Entry
  e3 : Entry
  e2 : Entry
  e1 : Entry
  deriving Repr

/-- `walk`, in the order `levels.rs` performs it: present then table-grants
    at level 4, present then block then table-grants at level 3, the same at
    level 2, present at level 1. -/
def walk (p : Path) : Except Err Leaf :=
  if !p.e4.present then .error .notMapped
  else if !p.e4.tableUser then .error .notUser
  else if !p.e3.present then .error .notMapped
  else if p.e3.block then .ok ⟨.l3, p.e3⟩
  else if !p.e3.tableUser then .error .notUser
  else if !p.e2.present then .error .notMapped
  else if p.e2.block then .ok ⟨.l2, p.e2⟩
  else if !p.e2.tableUser then .error .notUser
  else if !p.e1.present then .error .notMapped
  else .ok ⟨.l1, p.e1⟩

/-- `translate_read`: the walk, then the leaf's own user bit. -/
def translateRead (p : Path) : Except Err Leaf :=
  match walk p with
  | .error e => .error e
  | .ok leaf => if !leaf.entry.user then .error .notUser else .ok leaf

/-- `translate_write`: the same, and the leaf must be writable. -/
def translateWrite (p : Path) : Except Err Leaf :=
  match translateRead p with
  | .error e => .error e
  | .ok leaf => if !leaf.entry.writable then .error .notWritable else .ok leaf

/-- The table descriptors strictly above a leaf at `level`. This is the list
    the composition theorem quantifies over, and it is what changes with the
    stopping level. -/
def tablesAbove (p : Path) : Level → List Entry
  | .l3 => [p.e4]
  | .l2 => [p.e4, p.e3]
  | .l1 => [p.e4, p.e3, p.e2]

/-! ### Every table above a returned leaf granted user -/

/-- **The walk composition property.** A leaf the walk returns was reached
    through table descriptors that every one granted user, whichever of the
    three levels it stopped at.

    False if any `table_grants_user` check is dropped, and false if one is
    moved after the `is_block` test that returns the leaf. -/
theorem tables_above_grant_user (p : Path) (leaf : Leaf) (h : walk p = .ok leaf) :
    ∀ e ∈ tablesAbove p leaf.level, e.tableUser = true := by
  unfold walk at h
  split at h
  · exact absurd h (by simp)
  · split at h
    · exact absurd h (by simp)
    · split at h
      · exact absurd h (by simp)
      · split at h
        · injection h with h; subst h; simp_all [tablesAbove]
        · split at h
          · exact absurd h (by simp)
          · split at h
            · exact absurd h (by simp)
            · split at h
              · injection h with h; subst h; simp_all [tablesAbove]
              · split at h
                · exact absurd h (by simp)
                · split at h
                  · exact absurd h (by simp)
                  · injection h with h; subst h; simp_all [tablesAbove]

/-- Every entry the walk touches on the way to a leaf was present, so a leaf
    is never synthesised from an unmapped table. -/
theorem a_returned_leaf_is_present (p : Path) (leaf : Leaf) (h : walk p = .ok leaf) :
    leaf.entry.present = true := by
  unfold walk at h
  split at h
  · exact absurd h (by simp)
  · split at h
    · exact absurd h (by simp)
    · split at h
      · exact absurd h (by simp)
      · split at h
        · injection h with h; subst h; simp_all
        · split at h
          · exact absurd h (by simp)
          · split at h
            · exact absurd h (by simp)
            · split at h
              · injection h with h; subst h; simp_all
              · split at h
                · exact absurd h (by simp)
                · split at h
                  · exact absurd h (by simp)
                  · injection h with h; subst h; simp_all

/-! ### What `access.rs` adds -/

/-- **The read property.** A page `translate_read` accepts is user accessible
    the whole way down: every table above it grants user, and the leaf itself
    carries the user bit. This is the statement that `copy_from_user` cannot
    be pointed at kernel memory. -/
theorem read_path_is_user_accessible (p : Path) (leaf : Leaf)
    (h : translateRead p = .ok leaf) :
    leaf.entry.user = true ∧ ∀ e ∈ tablesAbove p leaf.level, e.tableUser = true := by
  unfold translateRead at h
  cases hw : walk p with
  | error e => rw [hw] at h; exact absurd h (by simp)
  | ok l =>
    rw [hw] at h
    by_cases hu : l.entry.user = true
    · simp [hu] at h
      subst h
      exact ⟨hu, tables_above_grant_user p l hw⟩
    · simp [hu] at h

/-- **The write property.** A page `translate_write` accepts is user
    accessible the whole way down and writable, so `copy_to_user` cannot be
    pointed at a read-only mapping or at kernel memory. -/
theorem write_path_is_user_writable (p : Path) (leaf : Leaf)
    (h : translateWrite p = .ok leaf) :
    leaf.entry.user = true ∧ leaf.entry.writable = true ∧
      ∀ e ∈ tablesAbove p leaf.level, e.tableUser = true := by
  unfold translateWrite at h
  cases hr : translateRead p with
  | error e => rw [hr] at h; exact absurd h (by simp)
  | ok l =>
    rw [hr] at h
    by_cases hw : l.entry.writable = true
    · simp [hw] at h
      subst h
      obtain ⟨hu, ht⟩ := read_path_is_user_accessible p l hr
      exact ⟨hu, hw, ht⟩
    · simp [hw] at h

/-- A write always satisfies everything a read does: the write path is the
    read path plus one test, so nothing is checked for reads and skipped for
    writes. -/
theorem write_implies_read (p : Path) (leaf : Leaf) (h : translateWrite p = .ok leaf) :
    translateRead p = .ok leaf := by
  unfold translateWrite at h
  cases hr : translateRead p with
  | error e => rw [hr] at h; exact absurd h (by simp)
  | ok l =>
    rw [hr] at h
    by_cases hw : l.entry.writable = true
    · simp [hw] at h; rw [h]
    · simp [hw] at h

/-! ### A leaf that must not be reachable -/

/-- A page with the user bit clear is never returned by `translate_read`,
    whatever the tables above it say. The leaf check is not redundant with the
    hierarchy: on aarch64 a table descriptor restricts nothing by default, so
    this is the test carrying the weight there. -/
theorem a_supervisor_leaf_is_never_read (p : Path) (leaf : Leaf)
    (h : translateRead p = .ok leaf) : leaf.entry.user ≠ false := by
  obtain ⟨hu, _⟩ := read_path_is_user_accessible p leaf h
  rw [hu]
  exact Bool.noConfusion

end Nonos.UserWalk
