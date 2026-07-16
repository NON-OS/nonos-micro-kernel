// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for URL resolution (RFC 3986 relative references). Every
//! relative href/src/@import goes through join(); a wrong "../" or path merge
//! silently points the browser at the wrong resource on every page.

use crate::browser::url::{join, parse};

fn resolved(base: &str, loc: &str) -> String {
    let b = parse(base).unwrap_or_else(|| panic!("parse base {base:?}"));
    join(&b, loc)
}

#[test]
fn absolute_and_protocol_relative_references() {
    // An absolute URL replaces the base entirely.
    assert_eq!(resolved("http://ex.com/a/b", "https://other.com/x"), "https://other.com/x");
    // A protocol-relative reference keeps the base scheme.
    assert_eq!(resolved("https://ex.com/a/b", "//cdn.com/x"), "https://cdn.com/x");
}

#[test]
fn absolute_and_relative_paths_merge_correctly() {
    assert_eq!(resolved("http://ex.com/a/b/c", "/x/y"), "http://ex.com/x/y", "root-absolute");
    assert_eq!(resolved("http://ex.com/a/b/c", "d"), "http://ex.com/a/b/d", "sibling");
    assert_eq!(resolved("http://ex.com/a/b/c", "../d"), "http://ex.com/a/d", "one up");
    assert_eq!(resolved("http://ex.com/a/b/c", "../../d"), "http://ex.com/d", "two up");
}

#[test]
fn query_and_fragment_references() {
    assert_eq!(resolved("http://ex.com/a/b", "#frag"), "http://ex.com/a/b#frag");
    assert_eq!(resolved("http://ex.com/a/b", "?q=1"), "http://ex.com/a/b?q=1");
}
