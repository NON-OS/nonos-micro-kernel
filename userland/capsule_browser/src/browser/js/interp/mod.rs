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

mod absorb;
mod add;
mod apply;
mod array_iter;
mod array_method;
mod array_mutate;
mod array_query;
mod array_reduce;
mod array_sort;
mod array_util;
mod attr_prop;
mod bound_member;
mod call_func;
mod class_build;
mod class_ctor;
mod class_instantiate;
mod class_methods;
mod classlist_method;
mod collect_text;
mod copy_children;
mod css_name;
mod ctx;
mod deliver_net;
mod document_member;
mod equals;
mod error_obj;
mod eval_args;
mod eval_array;
mod eval_assign;
mod eval_binary;
mod eval_call;
mod eval_expr;
mod eval_for;
mod eval_for_of;
mod eval_if;
mod eval_index;
mod eval_logical;
mod eval_member;
mod eval_new;
mod eval_object;
mod eval_stmt;
mod eval_try;
mod eval_unary;
mod eval_while;
mod exec;
mod flow;
mod globals;
mod graft_html;
mod hoist;
mod in_subtree;
mod map_method;
mod map_obj;
mod map_ops;
mod natives;
mod node_member;
mod node_method;
mod node_text;
mod obj;
mod promise_await;
mod promise_construct;
mod promise_make;
mod promise_then;
mod pump_timers;
mod regex_method;
mod regex_obj;
mod regex_replace;
mod regex_replace_util;
mod regex_split;
mod rel;
mod set_method;
mod set_node_prop;
mod set_text_content;
mod store;
mod str_method;
mod str_regex;
mod style_get;
mod style_set;
mod to_bool;
mod to_num;
mod to_str;
mod type_of;

pub use deliver_net::deliver_net;
pub use globals::install;
pub use pump_timers::pump_timers;
