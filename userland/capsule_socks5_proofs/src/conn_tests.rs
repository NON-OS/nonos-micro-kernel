// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the SOCKS5 handshake state machine: a no-auth greeting is answered
//! and moves to the request phase; a CONNECT opens a tunnel to the parsed
//! destination and, once opened, replies success and relays; a failed open
//! replies host-unreachable and closes; an unsupported command is rejected then
//! closed; a greeting split across two reads still completes; a request pipelined
//! behind the greeting is picked up; and a non-SOCKS5 client is closed.

use crate::conn::{Conn, Dest, Event};
use crate::wire::{
    ATYP_DOMAIN, REP_CONN_REFUSED, REP_GENERAL_FAIL, REP_HOST_UNREACH, REP_NET_UNREACH, REP_OK, VER,
};

fn is_reply(ev: &Event, expect: &[u8]) -> bool {
    matches!(ev, Event::ToClient { buf, len } if &buf[..*len] == expect)
}

#[test]
fn no_auth_greeting_is_accepted_and_advances() {
    let mut c = Conn::new();
    let ev = c.on_client(&[0x05, 0x01, 0x00]); // VER 5, one method: no-auth
    assert!(is_reply(&ev, &[VER, 0x00]), "server selects no-auth");
    assert!(!c.is_closed());
}

#[test]
fn a_greeting_without_no_auth_is_rejected_and_closes() {
    let mut c = Conn::new();
    let ev = c.on_client(&[0x05, 0x01, 0x02]); // only user/pass offered
    assert!(is_reply(&ev, &[VER, 0xFF]), "no acceptable methods");
    assert!(c.is_closed());
}

#[test]
fn connect_opens_a_tunnel_then_replies_and_relays() {
    let mut c = Conn::new();
    let _ = c.on_client(&[0x05, 0x01, 0x00]);
    // CONNECT 93.184.216.34:443
    let ev = c.on_client(&[0x05, 0x01, 0x00, 0x01, 93, 184, 216, 34, 0x01, 0xBB]);
    match ev {
        Event::Open(Dest::V4([93, 184, 216, 34], 443)) => {}
        _ => panic!("expected an open to the IPv4 destination"),
    }
    assert!(c.is_relaying());
    let (buf, len) = c.opened(REP_OK);
    assert_eq!(&buf[..len], &[VER, REP_OK, 0x00, 0x01, 0, 0, 0, 0, 0, 0]);
    assert!(c.is_relaying(), "a successful open keeps relaying");
}

#[test]
fn connect_to_a_domain_carries_the_name_unresolved() {
    let mut c = Conn::new();
    let _ = c.on_client(&[0x05, 0x01, 0x00]);
    let host = b"nonos.systems";
    let mut req = vec![0x05, 0x01, 0x00, ATYP_DOMAIN, host.len() as u8];
    req.extend_from_slice(host);
    req.extend_from_slice(&443u16.to_be_bytes());
    match c.on_client(&req) {
        Event::Open(Dest::Domain { name, len, port }) => {
            assert_eq!(&name[..len as usize], host);
            assert_eq!(port, 443);
        }
        _ => panic!("expected an open to the domain"),
    }
}

#[test]
fn a_failed_open_replies_the_code_it_was_given_and_closes() {
    let mut c = Conn::new();
    let _ = c.on_client(&[0x05, 0x01, 0x00]);
    let _ = c.on_client(&[0x05, 0x01, 0x00, 0x01, 10, 0, 0, 1, 0x00, 0x50]);
    let (buf, len) = c.opened(REP_HOST_UNREACH);
    assert_eq!(buf[1], REP_HOST_UNREACH);
    assert_eq!(len, 10);
    assert!(c.is_closed());
}

/// Each way a tunnel can fail reaches the client as its own code. They were once
/// one boolean, and a reader who could only be told "rejected" had no way to tell
/// a mixnet that is not connected from an exit that will not carry the address.
#[test]
fn every_refusal_carries_its_own_reason() {
    for code in [REP_GENERAL_FAIL, REP_NET_UNREACH, REP_HOST_UNREACH, REP_CONN_REFUSED] {
        let mut c = Conn::new();
        let _ = c.on_client(&[0x05, 0x01, 0x00]);
        let _ = c.on_client(&[0x05, 0x01, 0x00, 0x01, 10, 0, 0, 1, 0x00, 0x50]);
        let (buf, len) = c.opened(code);
        assert_eq!(buf[1], code, "the reply must carry the code the open earned");
        assert_eq!(len, 10);
        assert!(c.is_closed(), "a refused open closes whatever the reason");
    }
}

#[test]
fn an_unsupported_command_is_rejected_then_closed() {
    let mut c = Conn::new();
    let _ = c.on_client(&[0x05, 0x01, 0x00]);
    // CMD 3 = UDP ASSOCIATE.
    let ev = c.on_client(&[0x05, 0x03, 0x00, 0x01, 1, 2, 3, 4, 0, 80]);
    assert!(matches!(ev, Event::ToClient { buf, .. } if buf[1] == 0x07), "command not supported");
    assert!(c.is_closed());
}

#[test]
fn a_greeting_split_across_reads_completes() {
    let mut c = Conn::new();
    assert!(matches!(c.on_client(&[0x05]), Event::NeedMore));
    assert!(matches!(c.on_client(&[0x01]), Event::NeedMore));
    let ev = c.on_client(&[0x00]);
    assert!(is_reply(&ev, &[VER, 0x00]));
}

#[test]
fn a_request_pipelined_behind_the_greeting_is_picked_up() {
    let mut c = Conn::new();
    // Greeting and the CONNECT arrive in one read.
    let mut data = vec![0x05, 0x01, 0x00];
    data.extend_from_slice(&[0x05, 0x01, 0x00, 0x01, 8, 8, 8, 8, 0, 53]);
    let ev = c.on_client(&data);
    // The first call consumes the greeting and replies; the request is buffered.
    assert!(is_reply(&ev, &[VER, 0x00]));
    // Feeding nothing more, the buffered request is decoded.
    match c.on_client(&[]) {
        Event::Open(Dest::V4([8, 8, 8, 8], 53)) => {}
        _ => panic!("the pipelined request should open the tunnel"),
    }
}

#[test]
fn a_non_socks5_client_is_closed() {
    let mut c = Conn::new();
    // An HTTP client on the SOCKS port: 'G' 'E' 'T'.
    assert!(matches!(c.on_client(b"GET"), Event::Close));
    assert!(c.is_closed());
}
