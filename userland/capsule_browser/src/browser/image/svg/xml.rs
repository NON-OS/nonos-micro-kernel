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

// One markup tag: its name, raw attribute text, and whether it closes an
// element (</x>) or closes itself (<x/>).
pub(super) struct Tag<'a> {
    pub name: &'a str,
    pub attrs: &'a str,
    pub closing: bool,
    pub self_closing: bool,
}

// The next tag at or after byte `from`, with the offset just past it.
// Comments, doctype, CDATA and processing instructions are skipped.
pub(super) fn next_tag(s: &str, from: usize) -> Option<(Tag<'_>, usize)> {
    let mut i = from;
    loop {
        let open = i + s.get(i..)?.find('<')?;
        let rest = s.get(open..)?;
        if let Some(r) = rest.strip_prefix("<!--") {
            i = open + 4 + r.find("-->")? + 3;
            continue;
        }
        if rest.starts_with("<![CDATA[") {
            i = open + 9 + rest.get(9..)?.find("]]>")? + 3;
            continue;
        }
        if rest.starts_with("<!") || rest.starts_with("<?") {
            i = open + rest.find('>')? + 1;
            continue;
        }
        let close = rest.find('>')?;
        let inner = &rest[1..close];
        let end = open + close + 1;
        let closing = inner.starts_with('/');
        let inner = inner.trim_start_matches('/');
        let self_closing = inner.ends_with('/');
        let inner = inner.trim_end_matches('/');
        let name_end = inner.find(|c: char| c.is_whitespace()).unwrap_or(inner.len());
        let (name, attrs) = inner.split_at(name_end);
        return Some((Tag { name, attrs, closing, self_closing }, end));
    }
}
