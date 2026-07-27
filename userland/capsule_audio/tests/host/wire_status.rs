#[path = "../../src/sink/wire.rs"]
mod wire;

fn frame(status: i32) -> [u8; wire::HDR_LEN + wire::STATUS_LEN] {
    let mut b = [0u8; wire::HDR_LEN + wire::STATUS_LEN];
    b[20..24].copy_from_slice(&status.to_le_bytes());
    b
}

fn main() {
    assert_eq!(wire::reply_status(&frame(0)), 0);
    assert_eq!(wire::reply_status(&frame(-11)), -11);
    assert_eq!(wire::reply_status(&[0u8; 8]), i32::MIN);
    println!("HOSTTEST-PASS wire_status");
}
