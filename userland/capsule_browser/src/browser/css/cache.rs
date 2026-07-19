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

use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::compute::{cascade, compute};
use super::parse::parse;
use super::rule::Rule;

// Parsed author rules kept alive between relayouts, tagged with a cheap
// signature of the CSS text they came from. JS-driven relayouts reuse the
// parse instead of re-parsing the whole sheet each time.
pub struct CssCache {
    len: usize,
    hash: u64,
    rules: Vec<Rule>,
}

// Cascade with a persistent parse cache. Re-parses only when the CSS text
// changed (length or content hash differ); otherwise the cached rules are
// reused. The index build still runs per call since it depends on the
// current tree, but it is cheap next to parsing.
pub fn compute_cached(
    dom: &Dom,
    author_css: &str,
    cache: &mut Option<CssCache>,
) -> super::compute::Styled {
    let len = author_css.len();
    let hash = fnv1a(author_css.as_bytes());
    let fresh = matches!(cache, Some(c) if c.len == len && c.hash == hash);
    if !fresh {
        *cache = Some(CssCache { len, hash, rules: parse(author_css) });
    }
    let rules: &[Rule] = match cache {
        Some(c) => &c.rules,
        // Parse should have populated the cache above; fall back to the plain
        // path rather than trusting an empty cache.
        None => return compute(dom, author_css),
    };
    cascade(dom, rules)
}

// FNV-1a over the CSS bytes. Paired with the length it makes a same-length
// content change invalidate the cache, so a stale parse cannot be reused.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
