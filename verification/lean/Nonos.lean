/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

Specification-level security theorems, verified by Lean 4. Each is refined onto
the code that actually runs by the Verus / Kani / runnable proofs elsewhere in
`verification` and the `*_proofs` crates, so the abstract model is connected to
the implementation rather than standing apart from it.
-/

import Nonos.AntiRollback
import Nonos.Authorization
import Nonos.Capability
import Nonos.Isolation
