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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;
use crate::browser::dom::tree::Dom;
use crate::browser::html::flow::{Flow, Style};
use crate::browser::html::parse::flush::flush;

use super::walk::walk;

pub fn to_flows(dom: &Dom, styles: &[Computed]) -> Vec<Flow> {
    let mut out: Vec<Flow> = Vec::new();
    let mut buf = String::new();
    walk(dom, 0, Style::default(), None, &mut out, &mut buf, 0, styles);
    flush(&mut out, &mut buf, Style::default(), &None);
    out
}
