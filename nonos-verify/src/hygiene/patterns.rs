pub fn line_violation(line: &str) -> Option<&'static str> {
    for (needle, label) in CODE_PATTERNS {
        if line.contains(needle) {
            return Some(label);
        }
    }
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("//!") {
        for (needle, label) in COMMENT_PATTERNS {
            if trimmed.contains(needle) {
                return Some(label);
            }
        }
    }
    None
}

const CODE_PATTERNS: &[(&str, &str)] = &[
    (".unwrap(", "unwrap"),
    (".expect(", "expect"),
    ("panic!", "panic"),
    ("todo!(", "todo"),
    ("unimplemented!(", "unimplemented"),
    ("unreachable!(", "unreachable"),
    ("#[allow(dead_code)]", "allow-dead-code"),
];

const COMMENT_PATTERNS: &[(&str, &str)] = &[
    ("TODO", "todo-comment"),
    ("FIXME", "fixme-comment"),
    ("for now", "temporary-comment"),
    ("placeholder", "placeholder-comment"),
];
