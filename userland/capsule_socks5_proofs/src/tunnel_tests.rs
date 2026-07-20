// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the Nym `Socks5Request`/`Socks5Response` framing (protocol 3): a
//! connect request renders IPv4, IPv6 and domain destinations as the ASCII
//! host:port a Nym exit parses, with the big-endian id and no return address; a
//! send request carries the id, close flag, sequence and data; a response decodes
//! its id, sequence, closed flag and stream bytes; a connection error decodes as a
//! closed, empty connection; and an undersized output buffer is refused.

use crate::conn::Dest;
use crate::tunnel::{
    decode_response, encode_connect, encode_send, PROTOCOL_VERSION, REQ_CONNECT, REQ_SEND,
    RESP_CONNECTION_ERROR, RESP_NETWORK_DATA,
};

// Parse the fixed connect prefix: version, flag, big-endian id, and the u16
// big-endian address length. Returns the rendered address bytes.
fn connect_addr(out: &[u8], n: usize) -> (u64, &[u8]) {
    assert_eq!(out[0], PROTOCOL_VERSION);
    assert_eq!(out[1], REQ_CONNECT);
    let conn_id = u64::from_be_bytes(out[2..10].try_into().unwrap());
    let addr_len = u16::from_be_bytes(out[10..12].try_into().unwrap()) as usize;
    assert_eq!(12 + addr_len, n, "length names exactly the rendered address");
    (conn_id, &out[12..12 + addr_len])
}

#[test]
fn connect_renders_an_ipv4_host_port() {
    let mut out = [0u8; 64];
    let n = encode_connect(0x0102_0304_0506_0708, &Dest::V4([192, 168, 1, 4], 443), &mut out)
        .unwrap();
    let (id, addr) = connect_addr(&out, n);
    assert_eq!(id, 0x0102_0304_0506_0708);
    assert_eq!(addr, b"192.168.1.4:443");
}

#[test]
fn connect_renders_a_domain_host_port() {
    let host = b"nonos.systems";
    let mut name = [0u8; 255];
    name[..host.len()].copy_from_slice(host);
    let dest = Dest::Domain { name, len: host.len() as u8, port: 80 };
    let mut out = [0u8; 64];
    let n = encode_connect(7, &dest, &mut out).unwrap();
    let (_, addr) = connect_addr(&out, n);
    assert_eq!(addr, b"nonos.systems:80");
}

#[test]
fn connect_renders_a_bracketed_ipv6_host_port() {
    // ::1 loopback.
    let addr = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let mut out = [0u8; 64];
    let n = encode_connect(1, &Dest::V6(addr, 8080), &mut out).unwrap();
    let (_, rendered) = connect_addr(&out, n);
    assert_eq!(rendered, b"[0:0:0:0:0:0:0:1]:8080");
}

#[test]
fn send_carries_id_close_flag_sequence_and_data() {
    let mut out = [0u8; 64];
    let n = encode_send(0xAABB, 42, false, b"hello", &mut out).unwrap();
    assert_eq!(out[0], PROTOCOL_VERSION);
    assert_eq!(out[1], REQ_SEND);
    assert_eq!(u64::from_be_bytes(out[2..10].try_into().unwrap()), 0xAABB);
    assert_eq!(out[10], 0, "not closed");
    assert_eq!(u64::from_be_bytes(out[11..19].try_into().unwrap()), 42);
    assert_eq!(&out[19..n], b"hello");

    // A closing send is empty with the flag set.
    let n = encode_send(0xAABB, 43, true, &[], &mut out).unwrap();
    assert_eq!(out[10], 1, "closed");
    assert_eq!(n, 19);
}

#[test]
fn network_data_decodes_to_id_sequence_closed_and_data() {
    let mut buf = Vec::new();
    buf.push(PROTOCOL_VERSION);
    buf.push(RESP_NETWORK_DATA);
    buf.extend_from_slice(&0x1234u64.to_be_bytes());
    buf.push(1); // closed
    buf.extend_from_slice(&7u64.to_be_bytes()); // seq
    buf.extend_from_slice(b"payload");
    let r = decode_response(&buf).unwrap();
    assert_eq!(r.conn_id, 0x1234);
    assert_eq!(r.seq, 7);
    assert!(r.closed);
    assert_eq!(r.data, b"payload");
}

#[test]
fn a_connection_error_decodes_as_a_closed_empty_stream() {
    let mut buf = Vec::new();
    buf.push(PROTOCOL_VERSION);
    buf.push(RESP_CONNECTION_ERROR);
    buf.extend_from_slice(&0x99u64.to_be_bytes());
    buf.extend_from_slice(b"host unreachable");
    let r = decode_response(&buf).unwrap();
    assert_eq!(r.conn_id, 0x99);
    assert!(r.closed);
    assert_eq!(r.data, b"", "an error message is not stream data");
}

#[test]
fn a_malformed_or_wrong_version_response_is_refused() {
    // Wrong version.
    assert!(decode_response(&[1, RESP_NETWORK_DATA, 0, 0, 0, 0, 0, 0, 0, 0]).is_none());
    // Unknown flag.
    assert!(decode_response(&[PROTOCOL_VERSION, 9]).is_none());
    // Network data too short for its fixed fields.
    assert!(decode_response(&[PROTOCOL_VERSION, RESP_NETWORK_DATA, 0, 0, 0]).is_none());
}

#[test]
fn an_undersized_buffer_is_refused() {
    let mut small = [0u8; 8];
    assert!(encode_connect(1, &Dest::V4([1, 2, 3, 4], 80), &mut small).is_none());
    assert!(encode_send(1, 0, false, b"too big for this buffer here", &mut small).is_none());
}
