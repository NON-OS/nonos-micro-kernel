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

use super::types::Store;

// Content matching is skipped past this size so one huge file cannot stall the
// synchronous reply; such a file is still name-matched.
const SEARCH_MAX_FILE_BYTES: usize = 1 << 20;
// A NUL in the leading window means binary, matching the file manager's sniff.
const SEARCH_SNIFF_BYTES: usize = 512;

const FLAG_NAMES: u32 = 1;
const FLAG_CONTENT: u32 = 2;
const FLAG_CASE: u32 = 4;

// Hit kinds, as the client decodes them: a file matched by name, a file
// matched by content, or a directory matched by name. Directories carry their
// own kind because a store path does not say which it is.
const KIND_NAME: u32 = 0;
const KIND_CONTENT: u32 = 1;
const KIND_DIR_NAME: u32 = 2;

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(SEARCH_SNIFF_BYTES).any(|b| *b == 0)
}

impl Store {
    // Store-wide search. Returns (kind, line, path): kind 0 is a file matched
    // by name and kind 2 a directory matched by name, both with line 0; kind 1
    // is a content match with a 1-based line. Name hits sort before content
    // hits so the result is deterministic.
    pub fn search(&self, query: &str, flags: u32, max_hits: usize) -> Vec<(u32, u32, &str)> {
        let needle: String =
            if flags & FLAG_CASE != 0 { String::from(query) } else { query.to_ascii_lowercase() };
        if needle.is_empty() || flags & (FLAG_NAMES | FLAG_CONTENT) == 0 {
            return Vec::new();
        }
        let fold = |s: &str| -> String {
            if flags & FLAG_CASE != 0 { String::from(s) } else { s.to_ascii_lowercase() }
        };
        let mut names = Vec::new();
        let mut content = Vec::new();
        for f in self.files.iter() {
            if flags & FLAG_NAMES != 0 && fold(&f.name).contains(needle.as_str()) {
                let kind = if f.is_dir { KIND_DIR_NAME } else { KIND_NAME };
                names.push((kind, 0u32, f.name.as_str()));
                continue;
            }
            if f.is_dir || flags & FLAG_CONTENT == 0 || f.data.len() > SEARCH_MAX_FILE_BYTES {
                continue;
            }
            if is_binary(&f.data) {
                continue;
            }
            let Ok(text) = core::str::from_utf8(&f.data) else { continue };
            if let Some(n) = text.lines().position(|l| fold(l).contains(needle.as_str())) {
                content.push((KIND_CONTENT, n as u32 + 1, f.name.as_str()));
            }
        }
        names.sort_by(|a, b| a.2.cmp(b.2));
        content.sort_by(|a, b| a.2.cmp(b.2));
        names.extend(content);
        names.truncate(max_hits);
        names
    }
}
