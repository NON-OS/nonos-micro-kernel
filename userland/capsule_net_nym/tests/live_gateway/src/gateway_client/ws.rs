//! Stand-in for the capsule's websocket stack. `WsWire` is compiled against it
//! so the shipping type is type-checked here, but the live run drives a real
//! socket through `Wire` directly; these are never called.

pub fn send_text(_tcp_port: u32, _stream: u32, _payload: &[u8]) -> Result<(), u16> {
    Err(0)
}

pub fn recv_binary(_tcp_port: u32, _stream: u32, _out: &mut [u8]) -> Result<usize, u16> {
    Err(0)
}
