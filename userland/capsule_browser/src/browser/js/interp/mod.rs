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

mod add;
mod apply;
mod call_func;
mod collect_text;
mod ctx;
mod drain;
mod equals;
mod eval_args;
mod eval_array;
mod eval_assign;
mod eval_binary;
mod eval_call;
mod eval_expr;
mod eval_for;
mod eval_if;
mod eval_index;
mod eval_logical;
mod eval_member;
mod eval_object;
mod eval_stmt;
mod eval_unary;
mod eval_while;
mod exec;
mod flow;
mod globals;
mod hoist;
mod natives;
mod node_member;
mod node_text;
mod obj;
mod rel;
mod set_node_prop;
mod set_text_content;
mod store;
mod to_bool;
mod to_num;
mod to_str;
mod type_of;

pub use ctx::Ctx;
pub use drain::drain;
pub use exec::exec;
pub use globals::install;
