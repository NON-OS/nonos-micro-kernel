// NONOS Operating System (AGPL-3.0-or-later)
// The PCI window allocator's arithmetic, included from the kernel tree so the
// alignment and bounds proofs run against the code that programs real BARs.
#[path = "../../../../src/bus/pci/assign/carve.rs"]
mod carve;
pub use carve::*;
