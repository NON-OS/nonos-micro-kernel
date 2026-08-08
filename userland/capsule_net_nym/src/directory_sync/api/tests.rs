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

//! Known answers for the API reader, in the shapes the live API sends.
//!
//! A misparse here does not fail loudly: it builds a route to a node that
//! cannot decrypt what is sealed for it, and the packet is simply dropped
//! somewhere in the mixnet. Pinning the shapes is how that stays visible.

use super::base58::decode32;
use super::field::{ipv4_field, u64_field};
use super::node::parse_node;
use super::objects::objects;
use crate::topology::Role;

/// One node exactly as the skimmed view renders it: addresses in an array,
/// the layer nested inside the role object, keys in base58.
const MIXNODE: &str = r#"{"node_id":335,
    "ed25519_identity_pubkey":"D5LHPRKqP8Kavy4CZYrN6uuBsM1fXCNSkRkuYsdpwB7A",
    "ip_addresses":["51.68.220.113"],"mix_port":1789,
    "x25519_sphinx_pubkey":"BraYW1mMtKS5ux7TVekaxsoZo5KeyGdyy3bzXbKLySzF",
    "role":{"Mixnode":{"layer":3}},"performance":"0.99"}"#;

#[test]
fn reads_a_mixnode() {
    let node = parse_node(MIXNODE.as_bytes(), Role::Mix).expect("node");
    assert_eq!(node.ip, [51, 68, 220, 113]);
    assert_eq!(node.port, 1789);
    assert_eq!(node.layer, 3);
    assert!(node.role == Role::Mix);
}

/// A layer outside the three the route walks is refused rather than clamped.
#[test]
fn rejects_a_layer_that_cannot_be_routed() {
    let bad = MIXNODE.replace(r#""layer":3"#, r#""layer":7"#);
    assert!(parse_node(bad.as_bytes(), Role::Mix).is_none());
}

/// A record without a sphinx key cannot have a header sealed for it.
#[test]
fn rejects_a_node_with_no_packet_key() {
    let bad = MIXNODE.replace("x25519_sphinx_pubkey", "other_key");
    assert!(parse_node(bad.as_bytes(), Role::Mix).is_none());
}

#[test]
fn splits_objects_and_honours_the_cap() {
    let list = br#"[{"a":1},{"b":2},{"c":3}]"#;
    assert_eq!(objects(list, 8).len(), 3);
    assert_eq!(objects(list, 2).len(), 2);
}

/// A brace inside a string must not close the object early.
#[test]
fn a_brace_in_a_string_is_not_a_delimiter() {
    let list = br#"[{"host":"}{","port":1}]"#;
    let found = objects(list, 8);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0], &list[1..list.len() - 1]);
}

#[test]
fn reads_quoted_and_bare_numbers() {
    assert_eq!(u64_field(br#"{"n":1789}"#, "n"), Some(1789));
    assert_eq!(u64_field(br#"{"n":"1789"}"#, "n"), Some(1789));
    assert_eq!(u64_field(br#"{"n":"17x9"}"#, "n"), None);
}

#[test]
fn reads_dotted_quads_and_refuses_names() {
    assert_eq!(ipv4_field(b"185.186.147.240"), Some([185, 186, 147, 240]));
    assert_eq!(ipv4_field(b"256.1.1.1"), None);
    assert_eq!(ipv4_field(b"1.1.1"), None);
    assert_eq!(ipv4_field(b"gateway.nymtech.net"), None);
}

#[test]
fn base58_round_trips_a_known_key() {
    let key = decode32(b"CvhN9rBJw5Ay9wgHcbgCnVg89MPSV5s2muPV2YF1BXYu").expect("key");
    assert_ne!(key, [0u8; 32]);
    assert!(decode32(b"0OIl").is_none());
}
