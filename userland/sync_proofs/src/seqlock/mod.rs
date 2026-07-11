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

// The real sequence discipline the seqlock drives. `read.rs` and `write.rs` in
// the kernel call exactly these predicates. The functions are exercised only by
// the test and kani targets, and the `% 2` parity is the real code's own style
// choice, so both lints are the included code's, not this crate's.
#[allow(dead_code, clippy::manual_is_multiple_of)]
#[path = "../../../../src/sys/sync/seqlock/pure.rs"]
pub mod pure;
