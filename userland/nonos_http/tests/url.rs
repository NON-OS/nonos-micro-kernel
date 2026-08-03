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
//! URLs, including the ones that must be refused.
//!
//! The host and path are written straight into request headers, and the last
//! segment becomes a directory a clone writes into. Both come from whoever
//! typed the url, so both are checked.

use nonos_http::parse_url;

#[test]
fn an_ordinary_url_parses() {
    let u = parse_url("https://github.com/octocat/Hello-World.git").expect("parse");
    assert_eq!(u.host, "github.com");
    assert_eq!(u.path, "/octocat/Hello-World.git");
    assert_eq!(u.last_segment(), Some("Hello-World"));
}

#[test]
fn plain_http_is_refused() {
    assert!(parse_url("http://github.com/a/b.git").is_none());
    assert!(parse_url("git://github.com/a/b.git").is_none());
    assert!(parse_url("/etc/passwd").is_none());
    assert!(parse_url("https://github.com").is_none());
}

#[test]
fn a_url_that_could_forge_headers_is_refused() {
    // A carriage return in the host ends the Host line, and whatever follows
    // is read as headers of its own.
    assert!(parse_url("https://evil.com\r\nX-Injected: 1/a/b.git").is_none());
    assert!(parse_url("https://evil.com\nX-Injected: 1/a/b.git").is_none());
    // The same in the path splits the request line into two requests.
    assert!(parse_url("https://h.com/a\r\nGET /x HTTP/1.1\r\n\r\n/b.git").is_none());
    assert!(parse_url("https://h.com/a b.git").is_none());
    assert!(parse_url("https://h.com/a\x00b.git").is_none());
}

#[test]
fn a_port_is_refused_rather_than_ignored() {
    // Callers connect on 443. Taking a port and disregarding it would connect
    // somewhere the user did not ask for.
    assert!(parse_url("https://github.com:8443/a/b.git").is_none());
}

#[test]
fn a_url_naming_no_directory_is_refused() {
    // Each of these would have a clone write beside the working directory
    // rather than inside it.
    assert_eq!(parse_url("https://h.com/a/..").expect("parse").last_segment(), None);
    assert_eq!(parse_url("https://h.com/a/.").expect("parse").last_segment(), None);
    assert_eq!(parse_url("https://h.com/a/.git").expect("parse").last_segment(), None);
}

#[test]
fn an_overlong_url_is_refused() {
    let long = format!("https://h.com/{}", "a".repeat(4096));
    assert!(parse_url(&long).is_none());
}
