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

//! DOM host callbacks the QuickJS binding layer calls into. The engine carries
//! a raw pointer to the page DOM as its context opaque; these functions read and
//! mutate that tree. Script execution is synchronous, so the DOM is not touched
//! elsewhere while a callback runs. Strings returned to C are malloc'd with the
//! engine allocator and freed by the C side.

use alloc::string::String;
use core::ffi::c_void;
use core::ptr;
use core::slice;

use crate::browser::css;
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

extern "C" {
    fn malloc(n: usize) -> *mut u8;
}

unsafe fn dom<'a>(host: *mut c_void) -> &'a mut Dom {
    &mut *(host as *mut Dom)
}

unsafe fn cstr(p: *const u8) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    String::from_utf8_lossy(slice::from_raw_parts(p, n)).into_owned()
}

unsafe fn cdup(s: &str) -> *mut u8 {
    let b = s.as_bytes();
    let p = malloc(b.len() + 1);
    if p.is_null() {
        return p;
    }
    ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
    *p.add(b.len()) = 0;
    p
}

fn collect_text(d: &Dom, node: usize, out: &mut String) {
    if let Some(n) = d.nodes.get(node) {
        out.push_str(&n.text);
        for &c in &n.children {
            collect_text(d, c, out);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_create_element(host: *mut c_void, tag: *const u8) -> i32 {
    dom(host).create(NodeKind::Element, cstr(tag)).map(|i| i as i32).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_append(host: *mut c_void, parent: i32, child: i32) {
    if parent >= 0 && child >= 0 {
        dom(host).attach(parent as usize, child as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_remove_child(host: *mut c_void, _parent: i32, child: i32) {
    if child >= 0 {
        dom(host).detach(child as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_attr(host: *mut c_void, node: i32, k: *const u8, v: *const u8) {
    if node >= 0 {
        dom(host).set_attr(node as usize, &cstr(k), cstr(v));
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_attr(host: *mut c_void, node: i32, k: *const u8) -> *mut u8 {
    if node < 0 {
        return ptr::null_mut();
    }
    let key = cstr(k);
    match dom(host).nodes.get(node as usize) {
        Some(n) => match n.attrs.iter().find(|(a, _)| *a == key) {
            Some((_, val)) => cdup(val),
            None => ptr::null_mut(),
        },
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_text(host: *mut c_void, node: i32, text: *const u8) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    d.nodes[node as usize].children.clear();
    if let Some(tid) = d.create(NodeKind::Text, String::new()) {
        d.nodes[tid].text = cstr(text);
        d.attach(node as usize, tid);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_text(host: *mut c_void, node: i32) -> *mut u8 {
    if node < 0 {
        return ptr::null_mut();
    }
    let mut s = String::new();
    collect_text(dom(host), node as usize, &mut s);
    cdup(&s)
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_tag(host: *mut c_void, node: i32) -> *mut u8 {
    match dom(host).nodes.get(node as usize) {
        Some(n) => cdup(&n.tag),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_by_id(host: *mut c_void, id: *const u8) -> i32 {
    let want = cstr(id);
    for (i, n) in dom(host).nodes.iter().enumerate() {
        if n.attrs.iter().any(|(k, v)| k == "id" && *v == want) {
            return i as i32;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_query(host: *mut c_void, sel: *const u8) -> i32 {
    css::select(dom(host), &cstr(sel), 1).first().map(|i| *i as i32).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_query_all(
    host: *mut c_void,
    sel: *const u8,
    out: *mut i32,
    max: i32,
) -> i32 {
    let cap = max.max(0) as usize;
    let hits = css::select(dom(host), &cstr(sel), cap);
    let n = hits.len().min(cap);
    for (i, &id) in hits.iter().take(n).enumerate() {
        *out.add(i) = id as i32;
    }
    n as i32
}

const MAX_GRAFT_DEPTH: u32 = 64;

fn graft(dst: &mut Dom, src: &Dom, src_id: usize, dst_id: usize, depth: u32) {
    if depth > MAX_GRAFT_DEPTH {
        return;
    }
    let children = match src.nodes.get(src_id) {
        Some(n) => n.children.clone(),
        None => return,
    };
    for c in children {
        let (kind, tag, text, attrs) = match src.nodes.get(c) {
            Some(n) => (n.kind, n.tag.clone(), n.text.clone(), n.attrs.clone()),
            None => continue,
        };
        let Some(nid) = dst.push(dst_id, kind, tag) else {
            return;
        };
        dst.nodes[nid].text = text;
        dst.nodes[nid].attrs = attrs;
        graft(dst, src, c, nid, depth + 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_inner_html(host: *mut c_void, node: i32, html: *const u8) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    let frag = crate::browser::dom::parse(cstr(html).as_bytes());
    d.nodes[node as usize].children.clear();
    graft(d, &frag, 0, node as usize, 0);
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_set_style(host: *mut c_void, node: i32, prop: *const u8, val: *const u8) {
    if node < 0 {
        return;
    }
    let d = dom(host);
    let mut style = d
        .nodes
        .get(node as usize)
        .and_then(|n| n.attrs.iter().find(|(k, _)| k == "style").map(|(_, v)| v.clone()))
        .unwrap_or_default();
    style.push_str(&kebab(&cstr(prop)));
    style.push(':');
    style.push_str(&cstr(val));
    style.push(';');
    d.set_attr(node as usize, "style", style);
}

// camelCase CSS keys as frameworks emit them (borderRadius) to the kebab-case
// the stylesheet parser expects (border-radius).
fn kebab(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            out.push('-');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn njs_dom_body(host: *mut c_void) -> i32 {
    for (i, n) in dom(host).nodes.iter().enumerate() {
        if n.tag == "body" {
            return i as i32;
        }
    }
    0
}
