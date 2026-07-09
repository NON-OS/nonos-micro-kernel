// NONOS Operating System (AGPL-3.0-or-later)
// The real syscall id decode (ABI registry + number enum), self-contained.
#[path = "../../../../src/syscall/abi/mod.rs"]
pub mod abi;
#[path = "../../../../src/syscall/numbers/mod.rs"]
pub mod numbers;

// The capability checks and the syscall capability table (authorization gate).
pub mod caps;
pub mod contract;
