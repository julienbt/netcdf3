#![cfg(test)]

use super::{Attribute, DataType};
#[test]

fn test_new_i8() {
    let attr = Attribute::new_i8("attr1", vec![0, 1, 2, 3]).unwrap();

    assert_eq!(DataType::I8, attr.data_type());
    assert!(attr.get_i8().is_some());
    assert_eq!(&[0_i8, 1, 2, 3], attr.get_i8().unwrap());

    assert_eq!(None, attr.get_u8());
    assert_eq!(None, attr.get_i16());
    assert_eq!(None, attr.get_i32());
    assert_eq!(None, attr.get_f32());
    assert_eq!(None, attr.get_f64());
}
