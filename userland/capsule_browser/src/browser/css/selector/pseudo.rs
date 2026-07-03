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

use alloc::boxed::Box;

use super::simple::Simple;

// A pseudo-class on a compound. Structural ones evaluate against the DOM;
// `Never` stands for state and unsupported pseudos, which cannot hold in a
// static render, so the compound fails closed instead of over-matching.
pub enum Pseudo {
    FirstChild,
    LastChild,
    OnlyChild,
    FirstOfType,
    LastOfType,
    // :nth-child(An+B), 1-based among element siblings.
    NthChild(i32, i32),
    Empty,
    Not(Box<Simple>),
    Never,
}
