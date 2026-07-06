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

// Pick a loadable source from an @font-face src list. True type and open type
// load directly and woff unwraps to one; woff2 needs brotli and is passed
// over. Sources are ranked so a raw ttf/otf wins over a woff when both are
// offered.
pub(super) fn pick_src(src: &str) -> Option<String> {
    let mut woff: Option<String> = None;
    let mut rest = src;
    while let Some(pos) = rest.find("url(") {
        rest = &rest[pos + 4..];
        let end = rest.find(')')?;
        let raw = rest[..end].trim().trim_matches('"').trim_matches('\'');
        rest = &rest[end + 1..];
        let lower_end = raw.split('?').next().unwrap_or(raw).to_ascii_lowercase();
        if lower_end.ends_with(".ttf") || lower_end.ends_with(".otf") {
            return Some(String::from(raw));
        }
        if lower_end.ends_with(".woff") && woff.is_none() {
            woff = Some(String::from(raw));
        }
    }
    woff
}
