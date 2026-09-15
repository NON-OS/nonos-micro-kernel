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

extern crate alloc;

use super::file_ext::ext;

// Extension to capsule handle, most-preferred first. Every handle here must
// name a capsule that is actually built; an entry for a missing capsule is an
// invisible dead menu row, and the shell's convention is that an unwired row
// draws dimmed rather than silently no-opping.
const TABLE: &[(&str, &[&str])] = &[
    ("txt", &["app.text_editor"]),
    ("md", &["app.text_editor"]),
    ("log", &["app.text_editor"]),
    ("rs", &["app.text_editor"]),
    ("toml", &["app.text_editor"]),
    ("json", &["app.text_editor"]),
    ("flac", &["app.audio_player"]),
    ("wav", &["app.audio_player"]),
    ("mp3", &["app.audio_player"]),
    ("mp4", &["app.video_player"]),
    ("html", &["app.browser", "app.text_editor"]),
];

pub fn handlers_for(path: &str) -> &'static [&'static str] {
    let want = ext(path).to_ascii_lowercase();
    if want.is_empty() {
        return &[];
    }
    match TABLE.iter().find(|(e, _)| *e == want.as_str()) {
        Some((_, handles)) => handles,
        None => &[],
    }
}
