// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Microbenchmarks the kernel can run on itself.
//!
//! Off by default. These cost real time at boot and a production image has no
//! business spending it, so the whole suite sits behind
//! `nonos-bench-micro` and the benchmark lane in CI turns it on.
//!
//! What is here so far measures the hashing that capsule attestation spends
//! most of its time in. The costs still unmeasured, and worth more than this
//! one, are the IPC round trip, the syscall entry and exit, the capability
//! check, the signature verifications for both Ed25519 and ML-DSA-65, and the
//! STARK verify. Each of those needs a harness of its own rather than a line
//! added here.

mod hashes;

/// Run every microbenchmark and print the results on the boot channel.
pub fn run_all() {
    crate::sys::bench::mark(b"micro_suite_begin");
    hashes::run();
    crate::sys::bench::mark(b"micro_suite_end");
}
