// NONOS Operating System (AGPL-3.0-or-later)
//! What a descriptor must mean, checked on both architectures at once.
//!
//! Every property here is stated once and run twice through a macro, because
//! the whole value of the boundary is that the two backends answer the same
//! questions the same way from different bits. A property that only held on
//! one of them would be a boundary that does not exist.

use crate::descriptor::flags;

const PA: u64 = 0x0000_0001_2345_6000;

/// Instantiate the shared property set against one backend.
macro_rules! properties_for {
    ($name:ident, $backend:path) => {
        mod $name {
            use super::{flags, PA};
            use $backend as d;

            /// Presence follows what was asked for, both ways. A leaf built
            /// without `PRESENT` carries swap metadata and must fault; one
            /// that forced presence would make it a live mapping instead.
            #[test]
            fn presence_follows_the_request() {
                assert!(d::is_present(d::leaf(PA, flags::PRESENT)));
                assert!(d::is_present(d::leaf(PA, flags::PRESENT | flags::WRITABLE)));
                assert!(!d::is_present(d::leaf(PA, 0)));
                assert!(!d::is_present(d::leaf(PA, flags::WRITABLE | flags::USER)));
            }

            /// The output address survives encoding. This is the one thing
            /// every other property depends on.
            #[test]
            fn address_round_trips() {
                assert_eq!(d::address(d::leaf(PA, flags::PRESENT)), PA & d::ADDR_MASK);
            }

            /// Flag bits must never leak into the address field, whatever the
            /// caller passes.
            #[test]
            fn flags_do_not_disturb_the_address() {
                let noisy = flags::PRESENT
                    | flags::WRITABLE
                    | flags::USER
                    | flags::NO_CACHE
                    | flags::GLOBAL
                    | flags::NO_EXECUTE;
                assert_eq!(d::address(d::leaf(PA, noisy)), PA & d::ADDR_MASK);
            }

            /// Writability round-trips. aarch64 encodes this inverted, so a
            /// backend that copied the x86 rule fails here.
            #[test]
            fn writable_round_trips() {
                assert!(d::is_writable(d::leaf(PA, flags::PRESENT | flags::WRITABLE)));
                assert!(!d::is_writable(d::leaf(PA, flags::PRESENT)));
            }

            /// EL0 reachability round-trips, and is off unless asked for.
            #[test]
            fn user_round_trips() {
                assert!(d::is_user(d::leaf(PA, flags::PRESENT | flags::USER)));
                assert!(!d::is_user(d::leaf(PA, flags::PRESENT)));
            }

            /// Block and page are distinguishable. aarch64 marks a block by
            /// leaving a bit clear rather than setting one.
            #[test]
            fn block_round_trips() {
                assert!(d::is_block(d::leaf(PA, flags::PRESENT | flags::HUGE)));
                assert!(!d::is_block(d::leaf(PA, flags::PRESENT)));
                // An absent entry maps nothing, so it is not a block whatever
                // its other bits say.
                assert!(!d::is_block(d::leaf(PA, flags::HUGE)));
            }

            /// A table entry is present and is never mistaken for a block. The
            /// walk decides whether to descend on exactly this question, so a
            /// table that reads as a block ends the walk at a page table.
            #[test]
            fn table_is_present_and_not_a_block() {
                for user in [false, true] {
                    let entry = d::table(PA, user);
                    assert!(d::is_present(entry));
                    assert!(!d::is_block(entry));
                    assert_eq!(d::address(entry), PA & d::ADDR_MASK);
                }
            }

            /// A kernel mapping is never reachable from EL0. This is the bit
            /// whose polarity was inverted, and the failure mode is the whole
            /// kernel readable from userspace.
            #[test]
            fn kernel_leaf_is_never_user_reachable() {
                for extra in [0, flags::WRITABLE, flags::GLOBAL, flags::NO_EXECUTE] {
                    assert!(!d::is_user(d::leaf(PA, flags::PRESENT | extra)));
                }
            }

            /// A user mapping is always reachable from EL0, whatever else was
            /// asked for. The inverse failure locks userspace out of its own
            /// memory.
            #[test]
            fn user_leaf_is_always_user_reachable() {
                for extra in [0, flags::WRITABLE, flags::NO_CACHE, flags::NO_EXECUTE] {
                    assert!(d::is_user(d::leaf(PA, flags::PRESENT | flags::USER | extra)));
                }
            }

            /// A read-only request never yields a writable mapping. W^X for
            /// code pages rests on this.
            #[test]
            fn read_only_is_never_writable() {
                for extra in [0, flags::USER, flags::GLOBAL, flags::HUGE] {
                    assert!(!d::is_writable(d::leaf(PA, flags::PRESENT | extra)));
                }
            }
        }
    };
}

properties_for!(x86_64, crate::descriptor::x86_64);
properties_for!(aarch64, crate::descriptor::aarch64);
