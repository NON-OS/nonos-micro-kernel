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
import Nonos.Assurance
import Nonos.Attestation
import Nonos.Authorization
import Nonos.Barrier
import Nonos.Bitmap
import Nonos.BlockIO
import Nonos.BootImage
import Nonos.Bounds
import Nonos.Buddy
import Nonos.CapTable
import Nonos.Capability
import Nonos.CapabilityBits
import Nonos.Cow
import Nonos.Crypto
import Nonos.Dispatch
import Nonos.Endpoint
import Nonos.Epoch
import Nonos.Fd
import Nonos.Heap
import Nonos.Interval
import Nonos.Iommu
import Nonos.Ipc
import Nonos.Isolation
import Nonos.Loader
import Nonos.MemGrant
import Nonos.Mmio
import Nonos.Mutex
import Nonos.NetParse
import Nonos.Nonce
import Nonos.PageTable
import Nonos.Paging
import Nonos.Path
import Nonos.Priority
import Nonos.Quota
import Nonos.Reaper
import Nonos.Refcount
import Nonos.Ring
import Nonos.Rng
import Nonos.Scheduler
import Nonos.Secure
import Nonos.Semaphore
import Nonos.Seqlock
import Nonos.Signal
import Nonos.Spawn
import Nonos.Stark.AssociationSet
import Nonos.Stark.Attest
import Nonos.Stark.Commitment
import Nonos.Stark.Constraint
import Nonos.Stark.CopyConstraint
import Nonos.Stark.Field
import Nonos.Stark.FeeRouter
import Nonos.Stark.Fold
import Nonos.Stark.Extension
import Nonos.Stark.Fri
import Nonos.Stark.Instance
import Nonos.Stark.Lookup
import Nonos.Stark.Merkle
import Nonos.Stark.NullifierSet
import Nonos.Stark.Permutation
import Nonos.Stark.Pool
import Nonos.Stark.Polynomial
import Nonos.Stark.RootWindow
import Nonos.Stark.RunningSum
import Nonos.Stark.Staking
import Nonos.Stark.Transcript
import Nonos.Syscall
import Nonos.Ticket
import Nonos.Timer
import Nonos.Tlb
import Nonos.TokenBucket
import Nonos.UsbHid
import Nonos.Vfs
import Nonos.Zeroization
import Nonos.Zeroize
