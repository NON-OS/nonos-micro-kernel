#[path = "../../src/controller/bdl.rs"]
mod bdl;
use bdl::*;

fn main() {
    let e = build_bdl(0x10_0000);
    assert_eq!(e.len(), 4);
    assert_eq!(N_PERIODS, 4);
    assert_eq!(RING_BYTES, 0x8000);
    assert_eq!(e[0].addr, 0x10_0000);
    assert_eq!(e[1].addr, 0x10_0000 + 0x2000);
    assert_eq!(e[3].addr, 0x10_0000 + 3 * 0x2000);
    for x in &e {
        assert_eq!(x.len, 0x2000);
        assert_eq!(x.flags & 1, 1);
    }
    println!("HOSTTEST-PASS bdl");
}
