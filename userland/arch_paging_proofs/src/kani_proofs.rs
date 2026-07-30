// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the descriptor encoding is faithful for every input, on
//! both architectures.
//!
//! The host proofs beside this file check named cases. These check the same
//! properties over every physical address and every combination of flags, so
//! the privilege ones are statements about the encoding rather than about the
//! samples chosen.

use crate::descriptor::flags;

/// Instantiate the shared harness set against one backend.
macro_rules! harnesses_for {
    ($name:ident, $backend:path) => {
        mod $name {
            use super::flags;
            use $backend as d;

            /// Encoding then decoding returns what was asked for, for every
            /// address and every flag word.
            #[kani::proof]
            fn leaf_round_trips() {
                let pa: u64 = kani::any();
                let f: u64 = kani::any();
                let entry = d::leaf(pa, f);

                // Presence is the caller's to decide, not the encoder's. An
                // encoder that forced it would turn a deliberately absent
                // leaf, the kind carrying swap metadata, into a live mapping.
                assert_eq!(d::is_present(entry), f & flags::PRESENT != 0);
                assert_eq!(d::address(entry), pa & d::ADDR_MASK);
                assert_eq!(d::is_writable(entry), f & flags::WRITABLE != 0);
                assert_eq!(d::is_user(entry), f & flags::USER != 0);
                // An absent entry maps nothing, so it is not a block whatever
                // its other bits say.
                assert_eq!(
                    d::is_block(entry),
                    f & flags::PRESENT != 0 && f & flags::HUGE != 0
                );
            }

            /// No flag word can push a bit into the output address, and no
            /// address can push a bit into the attributes.
            #[kani::proof]
            fn address_and_attributes_never_overlap() {
                let pa: u64 = kani::any();
                let f: u64 = kani::any();
                assert_eq!(d::address(d::leaf(pa, f)), pa & d::ADDR_MASK);
            }

            /// EL0 can never reach a mapping that did not ask to be user
            /// accessible. There is no flag combination that opens it.
            #[kani::proof]
            fn kernel_mappings_are_closed_to_el0() {
                let pa: u64 = kani::any();
                let f: u64 = kani::any();
                kani::assume(f & flags::USER == 0);
                assert!(!d::is_user(d::leaf(pa, f)));
            }

            /// A mapping that did not ask to be writable never is. W^X for
            /// code pages rests on this holding for every other flag.
            #[kani::proof]
            fn read_only_mappings_are_never_writable() {
                let pa: u64 = kani::any();
                let f: u64 = kani::any();
                kani::assume(f & flags::WRITABLE == 0);
                assert!(!d::is_writable(d::leaf(pa, f)));
            }

            /// A table entry is present and never reads as a block, for every
            /// address. The walk decides whether to descend on exactly this,
            /// so a table that reads as a block ends the walk at a page table
            /// and hands its bytes back as if they were data.
            #[kani::proof]
            fn tables_are_never_blocks() {
                let pa: u64 = kani::any();
                let user: bool = kani::any();
                let entry = d::table(pa, user);
                assert!(d::is_present(entry));
                assert!(!d::is_block(entry));
                assert_eq!(d::address(entry), pa & d::ADDR_MASK);
            }
        }
    };
}

harnesses_for!(x86_64, crate::descriptor::x86_64);
harnesses_for!(aarch64, crate::descriptor::aarch64);

/// Splitting seconds into a date and putting them back is the identity.
///
/// Bounded to the first year on purpose. The conversion loops once per year
/// and once per month, so the whole 32-bit range unrolls into something a SAT
/// solver cannot close in useful time; the unbounded statement is proved by
/// induction in `verification/lean/Nonos/CivilTime.lean` instead, and the
/// host proofs beside this file walk four decades of real instants. What this
/// harness adds is every second of a year, including the carries.
#[kani::proof]
#[kani::unwind(14)]
fn civil_time_round_trips_within_the_first_year() {
    let seconds: u32 = kani::any();
    kani::assume(seconds < 365 * 24 * 60 * 60);
    let t = crate::civil::time::from_unix(seconds as u64);
    assert_eq!(crate::civil::time::to_unix(&t), seconds as u64);
}
