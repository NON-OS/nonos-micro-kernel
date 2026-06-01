use nonos_libc::mk_debug;

const PREFIX: &[u8] = b"[input_probe] ";
const MAX_LABEL: usize = 200;

pub fn marker(label: &[u8]) {
    let n = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let total = PREFIX.len() + n + 1;
    let mut buf = [0u8; PREFIX.len() + MAX_LABEL + 1];
    buf[..PREFIX.len()].copy_from_slice(PREFIX);
    buf[PREFIX.len()..PREFIX.len() + n].copy_from_slice(&label[..n]);
    buf[PREFIX.len() + n] = b'\n';
    let _ = mk_debug(buf.as_ptr(), total);
}

pub fn marker_u32(v: u32) {
    let mut hex = [0u8; 10];
    hex[0] = b'0';
    hex[1] = b'x';
    for i in 0..8 {
        let nibble = ((v >> ((7 - i) * 4)) & 0xf) as u8;
        hex[2 + i] = if nibble < 10 { b'0' + nibble } else { b'a' + nibble - 10 };
    }
    marker(&hex);
}
