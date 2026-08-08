// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the SOCKS5 wire codec: the greeting accepts no-auth and rejects a
//! bad version, the CONNECT request parses IPv4, IPv6 and domain destinations
//! with the port in network order, a truncated request reports incomplete rather
//! than failing, unsupported commands and address types are rejected with the
//! right reply code, and the reply encodes to the fixed ten-byte form.

use crate::wire::{
    method_reply, offers_no_auth, parse_connect, reply, Host, Parsed, ATYP_DOMAIN, METHOD_NONE,
    METHOD_NO_ACCEPT, REPLY_LEN, REP_ADDR_UNSUPP, REP_CMD_UNSUPP, REP_OK, VER,
};

#[test]
fn greeting_detects_no_auth_and_rejects_a_bad_version() {
    // VER 5, two methods: GSSAPI(1) and no-auth(0).
    assert_eq!(offers_no_auth(&[0x05, 0x02, 0x01, 0x00]), Some(true));
    // Only user/password(2) offered: no-auth absent.
    assert_eq!(offers_no_auth(&[0x05, 0x01, 0x02]), Some(false));
    // Wrong version is not SOCKS5.
    assert_eq!(offers_no_auth(&[0x04, 0x01, 0x00]), None);
    // Truncated: fewer methods than NMETHODS claims.
    assert_eq!(offers_no_auth(&[0x05, 0x03, 0x00]), None);
}

#[test]
fn method_reply_selects_no_auth_or_none() {
    assert_eq!(method_reply(true), [VER, METHOD_NONE]);
    assert_eq!(method_reply(false), [VER, METHOD_NO_ACCEPT]);
}

#[test]
fn connect_parses_ipv4_with_network_order_port() {
    // CONNECT to 1.2.3.4:443.
    let req = [0x05, 0x01, 0x00, 0x01, 1, 2, 3, 4, 0x01, 0xBB];
    match parse_connect(&req) {
        Parsed::Connect(c) => {
            assert!(matches!(c.host, Host::V4([1, 2, 3, 4])));
            assert_eq!(c.port, 443);
        }
        _ => panic!("expected a CONNECT"),
    }
}

#[test]
fn connect_parses_a_domain_unresolved() {
    // CONNECT to "nonos.systems":80, so the exit resolves it, not the client.
    let host = b"nonos.systems";
    let mut req = vec![0x05, 0x01, 0x00, ATYP_DOMAIN, host.len() as u8];
    req.extend_from_slice(host);
    req.extend_from_slice(&80u16.to_be_bytes());
    match parse_connect(&req) {
        Parsed::Connect(c) => {
            assert!(matches!(c.host, Host::Domain(d) if d == host));
            assert_eq!(c.port, 80);
        }
        _ => panic!("expected a CONNECT"),
    }
}

#[test]
fn connect_parses_ipv6() {
    let mut req = vec![0x05, 0x01, 0x00, 0x04];
    let addr = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    req.extend_from_slice(&addr);
    req.extend_from_slice(&8080u16.to_be_bytes());
    match parse_connect(&req) {
        Parsed::Connect(c) => {
            assert!(matches!(c.host, Host::V6(a) if a == addr));
            assert_eq!(c.port, 8080);
        }
        _ => panic!("expected a CONNECT"),
    }
}

#[test]
fn a_truncated_request_is_incomplete_not_an_error() {
    // Domain length claims 13 bytes but only 4 are present.
    let req = [0x05, 0x01, 0x00, ATYP_DOMAIN, 13, b'n', b'o', b'n'];
    assert!(matches!(parse_connect(&req), Parsed::Incomplete));
    // Just the four-byte header is not enough to know the address type's length.
    assert!(matches!(parse_connect(&[0x05, 0x01, 0x00, 0x01]), Parsed::Incomplete));
}

#[test]
fn bind_and_udp_commands_are_rejected() {
    // CMD 2 is BIND, CMD 3 is UDP ASSOCIATE: a privacy proxy supports neither.
    assert!(matches!(
        parse_connect(&[0x05, 0x02, 0x00, 0x01, 1, 2, 3, 4, 0, 80]),
        Parsed::Rejected(REP_CMD_UNSUPP)
    ));
}

#[test]
fn an_unknown_address_type_is_rejected() {
    assert!(matches!(
        parse_connect(&[0x05, 0x01, 0x00, 0x09, 0, 0]),
        Parsed::Rejected(REP_ADDR_UNSUPP)
    ));
}

#[test]
fn reply_encodes_the_fixed_ten_byte_form() {
    let mut out = [0xAAu8; REPLY_LEN];
    let n = reply(REP_OK, &mut out);
    assert_eq!(n, REPLY_LEN);
    assert_eq!(out, [VER, REP_OK, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);
}
