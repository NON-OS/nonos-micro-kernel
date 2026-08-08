// NONOS Operating System (AGPL-3.0-or-later)
//! What the staking screen is allowed to believe about the balance. A wallet
//! that mistakes "not read yet" for "holds nothing", or the other way round,
//! either blocks a real stake or signs one that can only revert.

use crate::nox::stakeable::held_wei;

const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;

fn word(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..32].copy_from_slice(&v.to_be_bytes());
    w
}

// An unread balance is not a balance of zero, and staking must be able to
// tell the two apart before it signs.
#[test]
fn an_unread_balance_is_not_zero() {
    assert_eq!(held_wei(false, &word(3000 * WEI_PER_NOX)), None);
    assert_eq!(held_wei(true, &word(0)), Some(0));
}

// Reported in wei, so staking everything can mean everything. Truncating to
// whole tokens would strand the fraction, and on a balance that grows by
// rewards the fraction is most of what is new.
#[test]
fn it_keeps_the_balance_at_chain_precision() {
    assert_eq!(held_wei(true, &word(3000 * WEI_PER_NOX)), Some(3000 * WEI_PER_NOX));
    assert_eq!(held_wei(true, &word(3000 * WEI_PER_NOX - 1)), Some(3000 * WEI_PER_NOX - 1));
    assert_eq!(held_wei(true, &word(1)), Some(1));
}

// A word too large to be a real balance is a decode fault, not a fortune.
#[test]
fn an_undecodable_word_reports_nothing() {
    let mut huge = [0u8; 32];
    huge[0] = 1;
    assert_eq!(held_wei(true, &huge), None);
}
