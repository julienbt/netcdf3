#![cfg(test)]
use crate::{DataSet, DataType, InvalidDataSet};

#[test]
fn test_add_var_attr_i8() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_i8";
    const VAR_ATTR_DATA: [i8; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `u8` variable attribute
    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),             data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),          data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN),  data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::I8),  data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `u8` stored values
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_u8() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_u8";
    const VAR_ATTR_DATA: [u8; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `i8` variable attribute
    data_set.add_var_attr_u8(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),                 data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),              data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN), data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::U8),      data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `i8` stored values
    assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_i16() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_i16";
    const VAR_ATTR_DATA: [i16; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `i16` variable attribute
    data_set.add_var_attr_i16(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),                 data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),              data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN), data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::I16),     data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `i16` stored values
    assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_i32() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_i32";
    const VAR_ATTR_DATA: [i32; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `i32` variable attribute
    data_set.add_var_attr_i32(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),                 data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),              data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN), data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::I32),     data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `i32` stored values
    assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_f32() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_f32";
    const VAR_ATTR_DATA: [f32; 3] = [1.0, 2.0, 3.0];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `f32` variable attribute
    data_set.add_var_attr_f32(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),                 data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),              data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN), data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::F32),     data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `f32` stored values
    assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_f64() {
    const VAR_NAME: &str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_f64";
    const VAR_ATTR_DATA: [f64; 3] = [1.0, 2.0, 3.0];
    const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();

    // First create the data set and a variable named `var_1`
    let mut data_set = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));
    assert_eq!(None, data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None, data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a variable
    data_set.add_var::<&str>(VAR_NAME, &vec![], DataType::F32).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Create a `f64` variable attribute
    data_set.add_var_attr_f64(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),                 data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),              data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(VAR_ATTR_DATA_LEN), data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(DataType::F64),     data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));

    // Then retreive the `f32` stored values
    assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_rename_var_attr()
{
    const VAR_NAME: &'static  str = "var_1";
    const VAR_ATTR_NAME_1: &str = "attr_1";
    const VAR_ATTR_NAME_2: &str = "attr_2";
    const VAR_ATTR_DATA: [i8; 3] = [1, 2, 3];

    let mut data_set: DataSet = DataSet::new();
    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_1, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    data_set.rename_var_attr(VAR_NAME, VAR_ATTR_NAME_1, VAR_ATTR_NAME_2).unwrap();

    // The attributes and its data have not been replaced or deleted
    assert_eq!(Some(1),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(true),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    assert_eq!(
        Some(&VAR_ATTR_DATA[..]),
        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_2)
    );
}


#[test]
fn test_remove_var_attr()
{
    const VAR_NAME: &'static  str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_1";
    const VAR_ATTR_DATA: [i8; 3] = [1, 2, 3];

    let mut data_set: DataSet = DataSet::new();
    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));

    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(Some(1),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));

    data_set.remove_var_attr(VAR_NAME, VAR_ATTR_NAME).unwrap();

    // The attributes and its data have not been replaced or deleted
    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));

}


#[test]
fn test_add_var_attr_error_attr_name_not_valid() {
    const VAR_NAME: &str = "var_1";
    const INVALID_VAR_ATTR_NAME: &str = "!invalid_name";
    const VAR_ATTR_DATA: [i8; 3] = [1, 2, 3];

    let mut data_set: DataSet = DataSet::new();

    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, INVALID_VAR_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::VariableAttributeNameNotValid{
            var_name: VAR_NAME.to_string(),
            attr_name: INVALID_VAR_ATTR_NAME.to_string()
        },
        data_set.add_var_attr_i8(VAR_NAME, INVALID_VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap_err()
    );

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, INVALID_VAR_ATTR_NAME));
}

#[test]
fn test_add_var_attr_error_attr_already_exists() {
    const VAR_NAME: &'static  str = "var_1";
    const VAR_ATTR_NAME: &str = "attr_1";
    const VAR_ATTR_DATA_1: [i8; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_2: [i8; 4] = [4, 5, 6, 7];

    let mut data_set: DataSet = DataSet::new();
    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));

    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA_1.to_vec()).unwrap();

    assert_eq!(Some(1),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::VariableAttributeAlreadyExists{
            var_name: VAR_NAME.to_string(),
            attr_name: VAR_ATTR_NAME.to_string()
        },
        data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA_2.to_vec()).unwrap_err()
    );

    // The attribute and its data have not been replaced
    assert_eq!(Some(1),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
    assert_eq!(
        Some(&VAR_ATTR_DATA_1[..]),
        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME)
    );
}

#[test]
fn test_rename_var_attr_error_attr_already_exists() {
    const VAR_NAME: &'static  str = "var_1";
    const VAR_ATTR_NAME_1: &str = "attr_1";
    const VAR_ATTR_NAME_2: &str = "attr_2";
    const VAR_ATTR_DATA_1: [i8; 3] = [1, 2, 3];
    const VAR_ATTR_DATA_2: [i8; 4] = [4, 5, 6, 7];

    let mut data_set: DataSet = DataSet::new();
    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(false), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_1, VAR_ATTR_DATA_1.to_vec()).unwrap();
    data_set.add_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_2, VAR_ATTR_DATA_2.to_vec()).unwrap();

    assert_eq!(Some(2),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(true), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    assert_eq!(
        InvalidDataSet::VariableAttributeAlreadyExists{
            var_name: VAR_NAME.to_string(),
            attr_name: VAR_ATTR_NAME_1.to_string()
        },
        data_set.rename_var_attr(VAR_NAME, VAR_ATTR_NAME_2, VAR_ATTR_NAME_1).unwrap_err()
    );

    // The attributes and its data have not been replaced or deleted
    assert_eq!(Some(2),     data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(true), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
    assert_eq!(Some(true), data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));

    assert_eq!(
        Some(&VAR_ATTR_DATA_1[..]),
        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_1)
    );
    assert_eq!(
        Some(&VAR_ATTR_DATA_2[..]),
        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME_2)
    );
}

#[test]
fn test_rename_var_attr_error_attr_not_defined () {
    const VAR_NAME: &str = "var_1";
    const UNDEF_VAR_ATTR_NAME: &str = "undef_name";
    const VAR_ATTR_NAME: &str = "attr_1";

    let mut data_set: DataSet = DataSet::new();

    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0), data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false),  data_set.has_var_attr(VAR_NAME, UNDEF_VAR_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::VariableAttributeNotDefined{
            var_name: VAR_NAME.to_string(),
            attr_name: UNDEF_VAR_ATTR_NAME.to_string(),
        },
        data_set.rename_var_attr(VAR_NAME, UNDEF_VAR_ATTR_NAME, VAR_ATTR_NAME).unwrap_err()
    );

    assert_eq!(Some(0), data_set.num_var_attrs(VAR_NAME));
    assert_eq!(Some(false),  data_set.has_var_attr(VAR_NAME, UNDEF_VAR_ATTR_NAME));
    assert_eq!(Some(false),  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
}

#[test]
fn test_remove_var_attr_error_attr_not_defined () {
    const VAR_NAME: &str = "var_1";
    const UNDEF_VAR_ATTR_NAME: &str = "undef_name";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(None, data_set.num_var_attrs(VAR_NAME));

    data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(Some(0), data_set.num_var_attrs(VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableAttributeNotDefined{
            var_name: VAR_NAME.to_string(),
            attr_name: UNDEF_VAR_ATTR_NAME.to_string(),
        },
        data_set.remove_var_attr(VAR_NAME, UNDEF_VAR_ATTR_NAME).unwrap_err()
    );

    assert_eq!(Some(0), data_set.num_var_attrs(VAR_NAME));
}