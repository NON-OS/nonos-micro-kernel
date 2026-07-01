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

mod args;
mod array;
mod binary;
mod block;
mod body;
mod expr;
mod func_decl;
mod object;
mod parse_for;
mod parse_if;
mod parse_return;
mod parse_while;
mod params;
mod parser;
mod postfix;
mod prec;
mod primary;
mod program;
mod statement;
mod unary;
mod var;

pub use parser::Parser;
pub use program::program;
