// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the Nym `Socks5Request`/`Socks5Response` framing (protocol 3)
//! inside the service provider envelope: every request opens with the
//! interface version and the provider-data tag; a connect request renders
//! IPv4, IPv6 and domain destinations as the ASCII host:port a Nym exit
//! parses, with the big-endian id and no return address; a send request
//! carries the id, close flag, sequence and data; a response decodes its id,
//! sequence, closed flag and stream bytes; a connection error decodes as a
//! closed, empty connection; a response sent without the envelope, or in one
//! we do not speak, is refused; and an undersized output buffer is refused.

use crate::conn::Dest;
use crate::tunnel::{
    decode_response, encode_connect, encode_send, ENVELOPE_BYTES, INTERFACE_VERSION,
    PROTOCOL_VERSION, REQ_CONNECT, REQ_SEND, RESP_CONNECTION_ERROR, RESP_NETWORK_DATA,
    TAG_PROVIDER_DATA,
};

// The envelope an exit reads before it reads any SOCKS5, followed by the
// request's own bytes.
fn body(out: &[u8]) -> &[u8] {
    assert_eq!(out[0], INTERFACE_VERSION, "interface version leads");
    assert_eq!(out[1], TAG_PROVIDER_DATA, "tagged as data for the provider");
    &out[ENVELOPE_BYTES..]
}

// Wrap a response the way an exit answers, so decoding sees what arrives.
fn wrap(inner: &[u8]) -> Vec<u8> {
    let mut buf = vec![INTERFACE_VERSION, TAG_PROVIDER_DATA];
    buf.extend_from_slice(inner);
    buf
}

// Parse the fixed connect prefix: version, flag, big-endian id, and the u16
// big-endian address length. Returns the rendered address bytes.
fn connect_addr(out: &[u8], n: usize) -> (u64, &[u8]) {
    let req = body(out);
    assert_eq!(req[0], PROTOCOL_VERSION);
    assert_eq!(req[1], REQ_CONNECT);
    let conn_id = u64::from_be_bytes(req[2..10].try_into().unwrap());
    let addr_len = u16::from_be_bytes(req[10..12].try_into().unwrap()) as usize;
    assert_eq!(ENVELOPE_BYTES + 12 + addr_len, n, "length names exactly the rendered address");
    (conn_id, &req[12..12 + addr_len])
}

#[test]
fn connect_renders_an_ipv4_host_port() {
    let mut out = [0u8; 64];
    let n =
        encode_connect(0x0102_0304_0506_0708, &Dest::V4([192, 168, 1, 4], 443), &mut out).unwrap();
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
    let req = body(&out);
    assert_eq!(req[0], PROTOCOL_VERSION);
    assert_eq!(req[1], REQ_SEND);
    assert_eq!(u64::from_be_bytes(req[2..10].try_into().unwrap()), 0xAABB);
    assert_eq!(req[10], 0, "not closed");
    assert_eq!(u64::from_be_bytes(req[11..19].try_into().unwrap()), 42);
    assert_eq!(&req[19..n - ENVELOPE_BYTES], b"hello");

    // A closing send is empty with the flag set.
    let n = encode_send(0xAABB, 43, true, &[], &mut out).unwrap();
    assert_eq!(body(&out)[10], 1, "closed");
    assert_eq!(n, ENVELOPE_BYTES + 19);
}

#[test]
fn network_data_decodes_to_id_sequence_closed_and_data() {
    // A response leads with the close flag, then the id, then the position.
    // A request writes the same fields id first, and reading one as the
    // other shifts the id by a byte.
    let mut inner = vec![PROTOCOL_VERSION, RESP_NETWORK_DATA];
    inner.push(1); // closed
    inner.extend_from_slice(&0x1234u64.to_be_bytes());
    inner.extend_from_slice(&7u64.to_be_bytes()); // seq
    inner.extend_from_slice(b"payload");
    let framed = wrap(&inner);
    let r = decode_response(&framed).unwrap();
    assert_eq!(r.conn_id, 0x1234);
    assert_eq!(r.seq, 7);
    assert!(r.closed);
    assert_eq!(r.data, b"payload");
}

#[test]
fn a_connection_error_decodes_as_a_closed_empty_stream() {
    let mut inner = vec![PROTOCOL_VERSION, RESP_CONNECTION_ERROR];
    inner.extend_from_slice(&0x99u64.to_be_bytes());
    inner.extend_from_slice(b"host unreachable");
    let framed = wrap(&inner);
    let r = decode_response(&framed).unwrap();
    assert_eq!(r.conn_id, 0x99);
    assert!(r.closed);
    assert_eq!(r.data, b"", "an error message is not stream data");
}

#[test]
fn a_malformed_or_wrong_version_response_is_refused() {
    // Wrong SOCKS5 version inside a well-formed envelope.
    assert!(decode_response(&wrap(&[1, RESP_NETWORK_DATA, 0, 0, 0, 0, 0, 0, 0, 0])).is_none());
    // Unknown flag.
    assert!(decode_response(&wrap(&[PROTOCOL_VERSION, 9])).is_none());
    // Network data too short for its fixed fields.
    assert!(decode_response(&wrap(&[PROTOCOL_VERSION, RESP_NETWORK_DATA, 0, 0, 0])).is_none());
}

#[test]
fn a_response_outside_the_envelope_is_refused() {
    let mut inner = vec![PROTOCOL_VERSION, RESP_NETWORK_DATA];
    inner.extend_from_slice(&1u64.to_be_bytes());
    inner.push(0);
    inner.extend_from_slice(&0u64.to_be_bytes());
    // Exactly the bytes that used to be sent and expected bare. Read as an
    // envelope the leading 3 is the interface version and the flag becomes
    // the tag, which is how a request went out looking like a control
    // message no exit could answer.
    assert!(decode_response(&inner).is_none(), "bare response is not one we speak");
    // A tag that is not provider data belongs to the control channel.
    let mut control = vec![INTERFACE_VERSION, 0];
    control.extend_from_slice(&inner);
    assert!(decode_response(&control).is_none());
}

#[test]
fn an_undersized_buffer_is_refused() {
    let mut small = [0u8; 8];
    assert!(encode_connect(1, &Dest::V4([1, 2, 3, 4], 80), &mut small).is_none());
    assert!(encode_send(1, 0, false, b"too big for this buffer here", &mut small).is_none());
    // Too small even for the envelope.
    let mut tiny = [0u8; 1];
    assert!(encode_connect(1, &Dest::V4([1, 2, 3, 4], 80), &mut tiny).is_none());
}
