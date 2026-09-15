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

//! The reply inbox name a process carries.
//!
//! This type replaced a `&'static str` that the runtime capsule loader could
//! only satisfy by leaking. The property that matters is that it never
//! silently shortens a name: a truncated inbox name is not a failed spawn, it
//! is replies delivered to whatever the prefix happens to match.

use crate::process::inbox_name::{InboxName, MAX_INBOX_NAME};

#[test]
fn a_name_survives_the_round_trip() {
    for name in ["a", "endpoint.app.nonos_wallet.1.reply", "proc.42"] {
        let n = InboxName::new(name).expect("fits");
        assert_eq!(n.as_str(), name);
    }
}

#[test]
fn the_longest_name_in_the_tree_fits_with_room_to_spare() {
    let longest = "endpoint.app.process_manager.2.reply";
    assert_eq!(longest.len(), 36);
    assert!(InboxName::new(longest).is_some());
    assert!(MAX_INBOX_NAME >= longest.len() * 2 - 8, "the cap should not be near the real names");
}

#[test]
fn exactly_the_cap_fits_and_one_more_does_not() {
    let at = "x".repeat(MAX_INBOX_NAME);
    assert_eq!(InboxName::new(&at).map(|n| n.as_str().len()), Some(MAX_INBOX_NAME));
    let over = "x".repeat(MAX_INBOX_NAME + 1);
    assert!(InboxName::new(&over).is_none(), "an overlong name must be refused, never cut");
}

#[test]
fn an_empty_name_is_refused() {
    assert!(InboxName::new("").is_none());
}

#[test]
fn no_name_is_ever_a_prefix_of_what_was_asked_for() {
    /*
     * The failure this guards against: a name cut to the buffer would still
     * compare equal to a shorter registered endpoint under a prefix match, so
     * a capsule could be handed another capsule's replies. Refusing is the
     * only safe answer, and this checks refusal rather than trusting it.
     */
    for len in [MAX_INBOX_NAME + 1, MAX_INBOX_NAME + 7, 512] {
        let long = "e".repeat(len);
        assert!(InboxName::new(&long).is_none(), "{len} bytes was accepted");
    }
}

#[test]
fn multi_byte_names_are_not_split_across_the_boundary() {
    // Two bytes each, so a cap of 64 lands mid-character if anything ever cut.
    let name = "\u{00e9}".repeat(MAX_INBOX_NAME / 2);
    assert_eq!(name.len(), MAX_INBOX_NAME);
    let n = InboxName::new(&name).expect("fits exactly");
    assert_eq!(n.as_str(), name);
    let over = "\u{00e9}".repeat(MAX_INBOX_NAME / 2 + 1);
    assert!(InboxName::new(&over).is_none());
}
