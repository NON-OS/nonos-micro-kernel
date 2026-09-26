// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The shape of the block at rsp.

use alloc::vec::Vec;

use crate::image::stack_words::words;

#[test]
fn argc_leads_and_both_lists_are_null_terminated() {
    let at = [0x1000, 0x1010, 0x2000];
    let got = words(2, &at, alloc_aux()).expect("two of three is a valid split");
    assert_eq!(got[0], 2);
    assert_eq!(&got[1..3], &[0x1000, 0x1010]);
    assert_eq!(got[3], 0, "argv is terminated before envp starts");
    assert_eq!(got[4], 0x2000);
    assert_eq!(got[5], 0, "envp is terminated before the auxv");
    assert_eq!(&got[6..], &[7, 7]);
}

#[test]
fn no_arguments_still_produces_both_terminators() {
    let got = words(0, &[], Vec::new()).expect("an empty vector is a valid one");
    assert_eq!(got, alloc::vec![0, 0, 0]);
}

#[test]
fn an_environment_with_no_arguments_keeps_its_place() {
    let got = words(0, &[0x3000], Vec::new()).expect("zero argv, one envp");
    // argc, the empty argv's null, the one envp entry, envp's null.
    assert_eq!(got, alloc::vec![0, 0, 0x3000, 0]);
}

#[test]
fn a_count_past_the_end_is_refused_rather_than_truncated() {
    assert!(words(3, &[0x1000], Vec::new()).is_none());
}

fn alloc_aux() -> Vec<u64> {
    alloc::vec![7, 7]
}
