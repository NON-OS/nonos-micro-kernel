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

//! The evidence behind what this system says about itself.
//!
//! Everywhere else, an operating system's claims are marketing: the words sit in
//! an About box and the reader either trusts them or does not. The point here is
//! that they are checkable from inside the running machine, by a capsule with no
//! special authority, while the user watches.
//!
//! Evidence is split by who established it, and the split is the honest part.
//! `recorded` is what the bootloader measured and the kernel merely reports back:
//! its own syscall says outright that it re-verifies none of it, so presenting it
//! as a live check would be a lie. `live` is computed here, now.
//!
//! A check earns its place only if it can fail. Numbers that are facts rather
//! than tests are kept apart as a census, because a green tick against something
//! that could never be red is the fakery this screen exists to replace.

mod boot;
mod own_mask;
mod scan;
mod table;
mod table_read;
mod types;

pub use boot::recorded;
pub use scan::live;
pub use types::{Census, Check, Live, Recorded, Verdict};
