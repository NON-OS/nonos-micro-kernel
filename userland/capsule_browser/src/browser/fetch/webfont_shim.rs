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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

// The WebFont.load({ google: { families: [...] } }) loader is how a large
// share of sites pull Google fonts. Running the loader script is out of
// scope, but the families it names are right there in the page, so synthesize
// the stylesheet url the loader would have requested and let the ordinary
// @font-face path take it from there.
pub(super) fn webfont_css_urls(dom: &Dom) -> Vec<String> {
    let mut out = Vec::new();
    for node in &dom.nodes {
        if node.kind != NodeKind::Text {
            continue;
        }
        let t = node.text.as_str();
        let Some(pos) = t.find("WebFont.load") else { continue };
        let Some(fam_at) = t[pos..].find("families:") else { continue };
        let rest = &t[pos + fam_at..];
        let Some(open) = rest.find('[') else { continue };
        let Some(close) = rest[open..].find(']') else { continue };
        // Family entries are quoted strings and may hold commas inside the
        // weight list, so walk the quoted spans rather than splitting on
        // commas.
        let list = &rest[open + 1..open + close];
        let mut parts = list.split(['"', '\'']);
        let _ = parts.next();
        while let (Some(fam), next) = (parts.next(), parts.next()) {
            let fam = fam.trim();
            if !fam.is_empty() && out.len() < 4 {
                let mut url = String::from("https://fonts.googleapis.com/css?family=");
                for ch in fam.chars() {
                    url.push(if ch == ' ' { '+' } else { ch });
                }
                out.push(url);
            }
            if next.is_none() {
                break;
            }
        }
    }
    out
}
