#[path = "../../src/paint/resample.rs"]
mod resample;

use resample::{cover, source_span};

const ONE: u64 = 1 << 16;

fn main() {
    let mut fail = 0usize;

    for dst in [1u32, 3, 16, 18, 30, 96, 192] {
        for src in [1u32, 8, 20, 64, 96, 192] {
            let mut total = 0u64;
            for i in 0..dst {
                let (lo, hi) = source_span(i, src, dst);
                if hi <= lo {
                    println!("empty span src={} dst={} i={}", src, dst, i);
                    fail += 1;
                }
                if hi > (src as u64) << 16 {
                    println!("overrun src={} dst={} i={} hi={}", src, dst, i, hi);
                    fail += 1;
                }
                for cell in 0..src as u64 {
                    total += cover(lo, hi, cell);
                }
            }
            let want = (src as u64) * ONE;
            let slack = dst as u64;
            if total + slack < want || total > want + slack {
                println!("coverage src={} dst={} total={} want={}", src, dst, total, want);
                fail += 1;
            }
        }
    }

    let (lo, hi) = source_span(0, 192, 18);
    if cover(lo, hi, 0) != ONE {
        println!("whole-cell overlap wrong: {}", cover(lo, hi, 0));
        fail += 1;
    }
    if cover(lo, hi, 191) != 0 {
        println!("disjoint overlap should be 0: {}", cover(lo, hi, 191));
        fail += 1;
    }

    if fail == 0 {
        println!("[RESAMPLE] PASS");
    } else {
        println!("[RESAMPLE] FAIL {}", fail);
        std::process::exit(1);
    }
}
