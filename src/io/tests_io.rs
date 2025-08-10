#![cfg(test)]

use super::compute_padding_size;

#[test]
fn test_compute_padding_size() {
    assert_eq!(0, compute_padding_size(0));
    assert_eq!(3, compute_padding_size(1));
    assert_eq!(2, compute_padding_size(2));
    assert_eq!(1, compute_padding_size(3));
    assert_eq!(0, compute_padding_size(4));
    assert_eq!(3, compute_padding_size(5));
}
