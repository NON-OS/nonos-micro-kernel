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

use alloc::string::{String, ToString};

use crate::browser::dom::node::Node;
use crate::browser::dom::Dom;

use super::srcset::pick_srcset;

// Slot width the srcset ranking aims for; images rarely span the viewport.
const WANT_W: i32 = 1024;

// Effective source for an <img>: a decodable <source> in the enclosing
// <picture> wins, then the img's own srcset, then src, then the lazy-load
// data-* attributes many sites use behind an empty or tiny src. <source>
// elements carrying a media condition are art-direction variants we cannot
// evaluate, so they are passed over.
pub(super) fn img_src(dom: &Dom, img: &Node) -> String {
    if let Some(parent) = dom.nodes.get(img.parent) {
        if parent.tag == "picture" {
            for &sid in &parent.children {
                let Some(s) = dom.nodes.get(sid) else { continue };
                if s.tag != "source" || s.attr("media").is_some() {
                    continue;
                }
                if s.attr("type").is_some_and(|t| !decodable_type(t)) {
                    continue;
                }
                if let Some(u) = s.attr("srcset").and_then(|l| pick_srcset(l, WANT_W)) {
                    return u;
                }
            }
        }
    }
    if let Some(u) = img.attr("srcset").and_then(|l| pick_srcset(l, WANT_W)) {
        return u;
    }
    let src = img.attr("src").unwrap_or("").trim();
    if !src.is_empty() {
        return src.to_string();
    }
    for lazy in ["data-src", "data-lazy-src"] {
        if let Some(u) = img.attr(lazy) {
            let u = u.trim();
            if !u.is_empty() {
                return u.to_string();
            }
        }
    }
    if let Some(u) = img.attr("data-srcset").and_then(|l| pick_srcset(l, WANT_W)) {
        return u;
    }
    String::new()
}

fn decodable_type(t: &str) -> bool {
    let t = t.trim().to_ascii_lowercase();
    !(t.contains("webp") || t.contains("avif"))
}
