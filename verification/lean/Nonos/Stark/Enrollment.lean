/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Policy enrollment: the set of measurements the gate will admit. A measurement is
enrolled exactly when it is a member of the enrolled list, enrolling one image
never un-enrolls another, an empty policy admits nothing, and a measurement absent
from the policy is refused. Composed with the measurement injectivity theorems,
this is why only images whose measurement was enrolled can attest, and an
unenrolled capsule is turned away at the gate.
-/

namespace Nonos.Stark.Enrollment

/-- A measurement is enrolled when it is a member of the enrolled set. -/
def enrolled (policy : List Nat) (m : Nat) : Prop := m ∈ policy

/-- The gate admits a measurement exactly when it is enrolled. -/
def admits (policy : List Nat) (m : Nat) : Prop := enrolled policy m

/-- The empty policy enrolls nothing. -/
theorem empty_enrolls_nothing (m : Nat) : ¬ enrolled [] m := by
  simp [enrolled]

/-- A measurement in the policy is enrolled. -/
theorem enrolled_of_mem (policy : List Nat) (m : Nat) (h : m ∈ policy) : enrolled policy m := h

/-- A measurement absent from the policy is not enrolled. -/
theorem not_enrolled_absent (policy : List Nat) (m : Nat) (h : m ∉ policy) :
    ¬ enrolled policy m := h

/-- Enrolling an image admits exactly that measurement. -/
theorem enroll_admits (policy : List Nat) (m : Nat) : enrolled (m :: policy) m := by
  simp [enrolled]

/-- Enrolling one image never un-enrolls another already enrolled. -/
theorem enroll_monotone (policy : List Nat) (m m' : Nat) (h : enrolled policy m) :
    enrolled (m' :: policy) m := by
  simp only [enrolled] at *; exact List.mem_cons_of_mem m' h

/-- A measurement enrolled in a prefix is enrolled in the concatenation. -/
theorem enroll_append_left (p q : List Nat) (m : Nat) (h : enrolled p m) :
    enrolled (p ++ q) m := by
  simp only [enrolled] at *; exact List.mem_append_left q h

/-- A measurement enrolled in a suffix is enrolled in the concatenation. -/
theorem enroll_append_right (p q : List Nat) (m : Nat) (h : enrolled q m) :
    enrolled (p ++ q) m := by
  simp only [enrolled] at *; exact List.mem_append_right p h

/-- An unenrolled measurement is refused by the gate. -/
theorem unenrolled_refused (policy : List Nat) (m : Nat) (h : m ∉ policy) :
    ¬ admits policy m := h

/-- Enrollment in a singleton policy pins the one image. -/
theorem singleton_enrolls_only (m x : Nat) (h : enrolled [m] x) : x = m := by
  simpa [enrolled] using h

/-- Removing all copies of a measurement leaves it unenrolled. -/
theorem filtered_out_not_enrolled (policy : List Nat) (m : Nat) :
    ¬ enrolled (policy.filter (· ≠ m)) m := by
  simp [enrolled, List.mem_filter]

/-- Enrollment is decidable: the gate can always answer admit or refuse. -/
theorem enrolled_decidable (policy : List Nat) (m : Nat) :
    enrolled policy m ∨ ¬ enrolled policy m := by
  by_cases h : enrolled policy m
  · exact Or.inl h
  · exact Or.inr h

end Nonos.Stark.Enrollment
