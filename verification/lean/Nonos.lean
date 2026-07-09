/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

Specification-level security theorems, verified by Lean 4. Each is refined onto
the code that actually runs by the Verus / Kani / runnable proofs elsewhere in
`verification` and the `*_proofs` crates, so the abstract model is connected to
the implementation rather than standing apart from it. `Secure` composes the
per-operation lemmas into one invariant that holds after every trace.
-/

import Nonos.AntiRollback
import Nonos.Attestation
import Nonos.Authorization
import Nonos.BlockIO
import Nonos.BootImage
import Nonos.Capability
import Nonos.CapabilityBits
import Nonos.Crypto
import Nonos.Ipc
import Nonos.Isolation
import Nonos.Loader
import Nonos.NetParse
import Nonos.Paging
import Nonos.Path
import Nonos.Secure
import Nonos.Spawn
import Nonos.Stark.Attest
import Nonos.Stark.CopyConstraint
import Nonos.Stark.Field
import Nonos.Stark.Fri
import Nonos.Stark.Lookup
import Nonos.Stark.Merkle
import Nonos.Stark.Permutation
import Nonos.Stark.Transcript
import Nonos.Syscall
import Nonos.UsbHid
import Nonos.Zeroization
