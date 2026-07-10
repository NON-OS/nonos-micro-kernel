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

//! Map a file path to the language shown in the status bar, from its extension.

// The line-comment marker for a path's language, used by Ctrl+/ toggling.
// Defaults to `//`, which is the most common and harmless when guessed wrong.
pub(super) fn comment_prefix(path: &str) -> &'static [u8] {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "py" | "sh" | "bash" | "zsh" | "toml" | "yaml" | "yml" | "rb" | "pl" => b"#",
        "sql" | "lua" | "hs" => b"--",
        _ => b"//",
    }
}

pub(super) fn language_name(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "Rust",
        "c" | "h" => "C",
        "cpp" | "cc" | "cxx" | "hpp" | "hh" => "C++",
        "py" => "Python",
        "js" | "mjs" | "cjs" => "JavaScript",
        "ts" | "tsx" => "TypeScript",
        "go" => "Go",
        "sh" | "bash" | "zsh" => "Shell",
        "toml" => "TOML",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "md" | "markdown" => "Markdown",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "java" => "Java",
        "kt" => "Kotlin",
        "rb" => "Ruby",
        "php" => "PHP",
        "lua" => "Lua",
        "sql" => "SQL",
        _ => "Text",
    }
}
