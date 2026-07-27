// NONOS Operating System (AGPL-3.0-or-later)
//! The bit-exact gate for the parallel periodic commit: on real AIRs with real
//! periodic columns, the all-cores root must equal the serial baked root
//! byte-for-byte. This is the only check that catches a self-consistent but
//! wrong root, so a settlement emit uses the parallel path only after it is
//! green. Covers the wired join-split (a real multi-column periodic AIR), and
//! the zero-periodic-column edge (RangeCheck) where the leaf count must still
//! agree.

use crate::crypto::stark::air::{periodic_root, Accumulator, Air, AirExt, RangeCheck, WiredExt};
use crate::crypto::stark::field::Fp;
use crate::production_vector_gen::parallel::parallel_periodic_root;
use alloc::boxed::Box;
use alloc::vec::Vec;

fn join_split() -> WiredExt {
    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7))
}

#[test]
fn the_join_split_has_periodic_columns_and_the_roots_match() {
    let air = join_split();
    assert!(
        !air.periodic_columns().is_empty(),
        "the test AIR must carry periodic columns for the gate to mean anything"
    );
    for extra in [0u32, 2, 4] {
        assert_eq!(
            periodic_root(&air, extra),
            parallel_periodic_root(&air, extra),
            "parallel root diverged from serial at extra_blowup {}",
            extra
        );
    }
}

#[test]
fn the_zero_periodic_column_edge_still_agrees() {
    let air = RangeCheck { log_t: 6 };
    assert!(air.periodic_columns().is_empty(), "RangeCheck is the zero-column edge case");
    for extra in [0u32, 3] {
        assert_eq!(
            periodic_root(&air, extra),
            parallel_periodic_root(&air, extra),
            "parallel root diverged from serial on the zero-column edge at extra_blowup {}",
            extra
        );
    }
}
