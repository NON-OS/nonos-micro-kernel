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

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::computed::Computed;
use super::parse::parse;
use super::ua::ua_rules;
use super::walk::walk;

pub fn compute(dom: &Dom, author_css: &str) -> Vec<Computed> {
    let ua = ua_rules();
    let author = parse(author_css);
    let author = &author[..super::budget::rule_limit(dom.nodes.len(), author.len())];
    let mut styles = vec![Computed::root(); dom.nodes.len()];
    walk(dom, 0, Computed::root(), &ua, author, &mut styles, 0);
    styles
}
