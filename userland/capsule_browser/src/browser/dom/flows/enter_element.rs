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

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::browser::dom::node::Node;
use crate::browser::html::flow::{Flow, Style};

use super::is_block::is_block;
use super::is_heading::is_heading;

pub fn enter_element(out: &mut Vec<Flow>, node: &Node, style: &mut Style, link: &mut Option<String>) {
    let name = node.tag.as_str();
    match name {
        "li" => {
            out.push(Flow::Break);
            out.push(Flow::Text(String::from("*"), Style::default()));
        }
        "td" | "th" => out.push(Flow::Text(String::from("|"), Style::default())),
        "b" | "strong" => style.bold = true,
        "pre" | "code" => style.pre = true,
        "a" => *link = node.attr("href").map(|s| s.to_string()),
        "input" => {
            let label = node
                .attr("placeholder")
                .or_else(|| node.attr("value"))
                .or_else(|| node.attr("name"))
                .unwrap_or("input");
            out.push(Flow::Text(format!("[{}]", label), Style::default()));
        }
        "img" => out.push(Flow::Image(
            node.attr("src").unwrap_or("").to_string(),
            node.attr("alt").unwrap_or("").to_string(),
        )),
        _ => {
            if is_block(name) {
                out.push(Flow::Break);
            }
        }
    }
    if is_heading(name) {
        style.heading = name.as_bytes()[1] - b'0';
    }
}
