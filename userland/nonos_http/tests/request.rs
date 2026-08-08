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
//! Requests, laid out the way a server expects to read them.

use nonos_http::RequestBuilder;

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_get_names_its_host_and_closes() {
    let r = RequestBuilder::get("github.com", "/octocat/Hello-World.git/info/refs").build();
    let s = text(&r.bytes);
    assert!(s.starts_with("GET /octocat/Hello-World.git/info/refs HTTP/1.1\r\n"));
    assert!(s.contains("Host: github.com\r\n"));
    assert!(s.contains("Connection: close\r\n"));
    assert!(s.ends_with("\r\n\r\n"));
    // A GET has no body, so it states no length.
    assert!(!s.contains("Content-Length"));
}

#[test]
fn a_post_states_its_length_and_type() {
    let body = b"0009done\n";
    let r = RequestBuilder::post("github.com", "/x.git/git-upload-pack", "application/x-git", body)
        .build();
    let s = text(&r.bytes);
    assert!(s.contains("Content-Type: application/x-git\r\n"));
    assert!(s.contains("Content-Length: 9\r\n"));
    assert!(s.ends_with("\r\n\r\n0009done\n"));
}

#[test]
fn an_empty_post_still_states_a_length() {
    // Without this the server waits for a body that never comes.
    let r = RequestBuilder::post("h", "/t", "application/x-git", &[]).build();
    assert!(text(&r.bytes).contains("Content-Length: 0\r\n"));
}

#[test]
fn the_user_agent_can_be_set() {
    let r = RequestBuilder::get("h", "/t").user_agent("nonos-git/0.1").build();
    assert!(text(&r.bytes).contains("User-Agent: nonos-git/0.1\r\n"));
}
