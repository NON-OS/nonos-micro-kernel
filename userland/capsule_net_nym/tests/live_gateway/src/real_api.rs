//! The parser against a response captured from the live API.
//!
//! The other vectors use fixtures written by hand, which is how a parser
//! passes its tests and still fails against the network: the fixture agrees
//! with the parser rather than with the server.

use crate::api::{node_objects, parse_node};
use crate::topology::Role;

const REAL: &[u8] = include_bytes!("real_mixnodes.json");

#[test]
fn the_live_mixnode_response_yields_nodes() {
    let found = node_objects(REAL, 128);
    assert!(!found.is_empty(), "no objects were split out of the response");

    let nodes: Vec<_> = found.iter().filter_map(|o| parse_node(o, Role::Mix)).collect();
    assert!(!nodes.is_empty(), "{} objects parsed to zero nodes", found.len());
}

#[test]
fn the_live_response_covers_every_mix_layer() {
    let found = node_objects(REAL, 128);
    let nodes: Vec<_> = found.iter().filter_map(|o| parse_node(o, Role::Mix)).collect();
    for layer in 1u8..=3 {
        let n = nodes.iter().filter(|n| n.layer == layer).count();
        assert!(n > 0, "layer {layer} has no nodes, so no route can be built");
    }
}

#[test]
fn a_parsed_node_carries_what_a_route_needs() {
    let found = node_objects(REAL, 128);
    let node = found.iter().find_map(|o| parse_node(o, Role::Mix)).expect("at least one node");
    assert_ne!(node.ip, [0, 0, 0, 0], "a hop with no address routes nowhere");
    assert_eq!(node.port, 1789, "mix hops are reached on the mix port");
    assert_ne!(node.packet_key, [0u8; 32], "a hop with no packet key cannot be sealed to");
}
