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
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::matching::matches_selector;
use super::rule::Rule;

const MAX_VARS: usize = 2048;
const MAX_DEPTH: u32 = 8;

// Gather the custom properties (--name) defined across the sheets. Later
// definitions win, so lookups scan the list from the back. Rules whose
// selectors qualify the root element (html[data-theme=dark], html.dark)
// contribute only when the real root matches, which is how theme palettes
// stay on the theme actually in effect; other scopes keep the global
// flatten that covers the dominant :root token case.
pub(super) fn collect_vars(ua: &[Rule], author: &[Rule], dom: &Dom) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let roots = root_ids(dom);
    for rule in ua.iter().chain(author.iter()) {
        if !rule.decls.iter().any(|d| d.name.starts_with("--")) {
            continue;
        }
        let include = rule.selectors.iter().any(|sel| {
            let key = &sel.key;
            let is_root_tag = matches!(key.tag.as_deref(), Some("html") | Some("body"))
                && sel.ancestors.is_empty();
            if !is_root_tag {
                return true;
            }
            roots.iter().any(|&id| matches_selector(dom, id, sel))
        });
        if !include {
            continue;
        }
        for d in &rule.decls {
            if out.len() >= MAX_VARS {
                return out;
            }
            if d.name.starts_with("--") {
                out.push((d.name.clone(), d.value.trim().to_string()));
            }
        }
    }
    out
}

// The html and body element ids, for root-scoped variable rules.
fn root_ids(dom: &Dom) -> Vec<usize> {
    let mut out = Vec::new();
    let mut stack: Vec<usize> = alloc::vec![0];
    while let Some(id) = stack.pop() {
        let Some(n) = dom.nodes.get(id) else { continue };
        if n.kind == NodeKind::Element && (n.tag == "html" || n.tag == "body") {
            out.push(id);
        }
        if n.kind != NodeKind::Element || n.tag == "html" {
            for &ch in &n.children {
                stack.push(ch);
            }
        }
    }
    out
}

// Substitute every var(--name) / var(--name, fallback) in `value`, recursing
// since tokens nest. A var with no usable definition and no fallback poisons
// the whole value (None), matching the guaranteed-invalid contract that
// light-dark toggle polyfills rely on; a definition of `initial` counts as
// undefined the same way.
pub(super) fn resolve(value: &str, vars: &[(String, String)], depth: u32) -> Option<String> {
    if depth >= MAX_DEPTH || !value.contains("var(") {
        return Some(value.to_string());
    }
    let mut out = String::new();
    let mut cursor = 0;
    while let Some(rel) = value[cursor..].find("var(") {
        let p = cursor + rel;
        out.push_str(&value[cursor..p]);
        let inner_start = p + 4;
        match find_close(value, inner_start) {
            Some(close) => {
                let inner = &value[inner_start..close];
                let (name, fallback) = split_top_comma(inner);
                let name = name.trim();
                let defined = lookup(vars, name)
                    .filter(|v| !v.trim().eq_ignore_ascii_case("initial"))
                    .and_then(|v| resolve(v, vars, depth + 1));
                let sub = match defined {
                    Some(s) => s,
                    None => resolve(fallback?.trim(), vars, depth + 1)?,
                };
                out.push_str(&sub);
                cursor = close + 1;
            }
            None => {
                // Unbalanced parens: keep the remainder verbatim and stop.
                out.push_str(&value[p..]);
                return Some(out);
            }
        }
    }
    out.push_str(&value[cursor..]);
    Some(out)
}

// Rewrite every light-dark(a, b) to its light argument. This browser renders
// the light scheme only, so the function collapses at value-resolution time.
pub(crate) fn strip_light_dark(value: &str) -> String {
    if !value.contains("light-dark(") {
        return value.to_string();
    }
    let mut out = String::new();
    let mut cursor = 0;
    let mut hops = 0u32;
    while let Some(rel) = value[cursor..].find("light-dark(") {
        hops += 1;
        if hops > MAX_DEPTH {
            break;
        }
        let p = cursor + rel;
        out.push_str(&value[cursor..p]);
        let inner_start = p + 11;
        match find_close(value, inner_start) {
            Some(close) => {
                let (light, _) = split_top_comma(&value[inner_start..close]);
                // The light side may itself hold a nested light-dark().
                out.push_str(&strip_light_dark(light.trim()));
                cursor = close + 1;
            }
            None => {
                out.push_str(&value[p..]);
                return out;
            }
        }
    }
    out.push_str(&value[cursor..]);
    out
}

// Byte index of the ')' that closes the '(' opened just before `start`,
// tracking nested parens.
fn find_close(s: &str, start: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut depth = 1u32;
    let mut i = start;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

// Split "name, fallback" at the first top-level comma; parens in a fallback
// keep their own commas out of the split.
fn split_top_comma(inner: &str) -> (&str, Option<&str>) {
    let b = inner.as_bytes();
    let mut depth = 0u32;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => return (&inner[..i], inner.get(i + 1..)),
            _ => {}
        }
        i += 1;
    }
    (inner, None)
}

// Last definition wins; names compare case-insensitively since values keep
// their original case while definitions are lowercased at parse time.
fn lookup<'a>(vars: &'a [(String, String)], name: &str) -> Option<&'a str> {
    vars.iter().rev().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
}
