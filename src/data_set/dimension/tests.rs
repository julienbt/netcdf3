#![cfg(test)]

use std::rc::Rc;
use crate::{Dimension, DimensionType};

#[test]
fn test_dim_new_fixed_size() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let dim = Dimension::new_fixed_size(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(DIM_NAME, dim.name());
    assert_eq!(DIM_SIZE, dim.size());
    assert_eq!(DimensionType::FixedSize, dim.dim_type());
    assert_eq!(true, dim.is_fixed());
    assert_eq!(false, dim.is_unlimited())
}

#[test]
fn test_dim_new_unlimited_size() {
    const DIM_NAME: &str = "dim_1";
    const DIM_SIZE: usize = 10;

    let dim = Dimension::new_unlimited_size(DIM_NAME, DIM_SIZE).unwrap();

    assert_eq!(DIM_NAME, dim.name());
    assert_eq!(DIM_SIZE, dim.size());
    assert_eq!(DimensionType::UnlimitedSize, dim.dim_type());
    assert_eq!(false, dim.is_fixed());
    assert_eq!(true, dim.is_unlimited());
}

#[test]
fn test_dim_equality() {

    // test equality between 2 fixed-size dimension
    {
        let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
        let dim_b: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
        assert_eq!(dim_a, dim_b);
    }

    // test equality between 2 fixed-size dimension with different sizes
    {
        let dim_a: Dimension = Dimension::new_fixed_size("name_1", 90).unwrap();
        let dim_b: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
        assert_ne!(dim_a, dim_b);
    }

    // test equality between 2 fixed-size dimension with different names
    {
        let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
        let dim_b: Dimension = Dimension::new_fixed_size("name_2", 180).unwrap();
        assert_ne!(dim_a, dim_b);
    }

    // test equality between 2 unlimited-size dimension
    {
        let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
        let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
        assert_eq!(dim_a, dim_b);
    }

    // test equality between 2 unlimited-size dimension with different sizes
    {
        let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 90).unwrap();
        let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
        assert_ne!(dim_a, dim_b);
    }

    // test equality between 2 unlimited-size dimension with different names
    {
        let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
        let dim_b: Dimension = Dimension::new_unlimited_size("name_2", 180).unwrap();
        assert_ne!(dim_a, dim_b);
    }

    // test equality between 1 unlimited-size dimension and 1 fixed-size dimension
    {
        let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
        let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
        assert_ne!(dim_a, dim_b);
    }
}

#[test]
fn test_rc_dim_equality() {
    // test equality between 2 fixed-size dimensions
    {
        let dim_a: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("name_1", 180).unwrap());
        let dim_b: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("name_1", 180).unwrap());

        assert_eq!(dim_a, dim_b);
        assert!(!Rc::ptr_eq(&dim_a, &dim_b));

        let dim_c: Rc<Dimension> = Rc::clone(&dim_a);
        assert_eq!(dim_a, dim_c);
        assert_eq!(dim_b, dim_c);
        assert!(Rc::ptr_eq(&dim_a, &dim_c));
        assert!(!Rc::ptr_eq(&dim_b, &dim_c));
    }
}