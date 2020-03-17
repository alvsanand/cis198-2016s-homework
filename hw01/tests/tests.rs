#![cfg(test)]

extern crate hw01;

use hw01::problem4::{hanoi, Peg};

//
// Problem 4
//

#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3, Peg::A, Peg::B, Peg::C);
    let expected = vec![
        (Peg::A, Peg::C),
        (Peg::A, Peg::B),
        (Peg::C, Peg::B),
        (Peg::A, Peg::C),
        (Peg::B, Peg::A),
        (Peg::B, Peg::C),
        (Peg::A, Peg::C),
    ];
    assert_eq!(expected, result);
    assert_eq!(expected.len(), result.len());
}
