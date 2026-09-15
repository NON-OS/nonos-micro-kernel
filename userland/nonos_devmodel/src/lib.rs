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

//! Driving a real driver's register code with no device attached.
//!
//! The problem this exists for is stated plainly: 60,239 lines of driver code
//! ship in this system and, before this crate, 9% of it was reachable from a
//! host proof. The 91% was not untested by choice. It was the half that talks
//! to hardware, and nothing could talk to hardware without hardware, so it was
//! only ever exercised by booting a machine that had the part.
//!
//! That is a bad place to be for a system that intends to run on other
//! people's computers. It makes every driver claim rest on whichever cards the
//! authors happened to own, and it makes a regression in a device this project
//! cannot borrow invisible until a stranger's machine fails to boot.
//!
//! What makes the gap closable is that a driver's register window is reached
//! through a type built from a plain base address. Point it at [`FakeBar`] and
//! the driver's own code runs on the host, unchanged, against memory.
//!
//! What that does and does not prove, stated honestly:
//!
//! It proves the driver obeys a written contract. Bus specifications say what
//! a conforming device requires: which registers, in which order, holding
//! what. A driver that violates that order is wrong on every implementation of
//! the spec, and a test here catches it without owning any of them.
//!
//! It does not prove the part in front of you honours the same specification.
//! Silicon has errata, and this cannot find them. What it can do is make the
//! parts that are supposed to be portable actually portable, so the machine
//! that has to be borrowed is only borrowed for the questions that need it.

mod bar;
mod live;
mod observe;
mod present;

pub use bar::FakeBar;
pub use live::{run, LiveDevice};

#[cfg(test)]
mod tests;
