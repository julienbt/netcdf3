#![cfg(test)]

#![cfg(test)]
use crate::{DataSet, DataType, InvalidDataSet};

#[test]
fn test_add_global_attr_i8() {
    const GLOBAL_ATTR_NAME: &str = "attr_i8";
    const GLOBAL_ATTR_DATA: [i8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `i8` stored values
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}


#[test]
fn test_add_global_attr_u8() {
    const GLOBAL_ATTR_NAME: &str = "attr_u8";
    const GLOBAL_ATTR_DATA: [u8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_u8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::U8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `u8` stored values
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_global_attr_i16() {
    const GLOBAL_ATTR_NAME: &str = "attr_i16";
    const GLOBAL_ATTR_DATA: [i16; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_i16(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I16),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `i16` stored values
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_global_attr_i32() {
    const GLOBAL_ATTR_NAME: &str = "attr_i32";
    const GLOBAL_ATTR_DATA: [i32; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_i32(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `i32` stored values
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_global_attr_f32() {
    const GLOBAL_ATTR_NAME: &str = "attr_f32";
    const GLOBAL_ATTR_DATA: [f32; 3] = [1.0, 2.0, 3.0];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_f32(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::F32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `f32` stored values
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_global_attr_f64() {
    const GLOBAL_ATTR_NAME: &str = "attr_f64";
    const GLOBAL_ATTR_DATA: [f64; 3] = [1.0, 2.0, 3.0];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_f64(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::F64),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));

    // Then retreive the `f32` stored values
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
}

#[test]
fn test_rename_global_attr()
{
    const GLOBAL_ATTR_NAME_1: &str = "attr_1";
    const GLOBAL_ATTR_NAME_2: &str = "attr_2";
    const GLOBAL_ATTR_DATA: [i8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    assert_eq!(None, data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(None, data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(DataType::I8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(false,                       data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,                        data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,                        data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));

    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    data_set.rename_global_attr(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_NAME_2).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(false,                       data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,                        data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,                        data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(DataType::I8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));

    assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
}

#[test]
fn test_remove_global_attr()
{
    const GLOBAL_ATTR_NAME: &str = "attr_1";
    const GLOBAL_ATTR_DATA: [i8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();

    assert_eq!(1,                           data_set.num_global_attrs());
    assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));

    data_set.remove_global_attr(GLOBAL_ATTR_NAME).unwrap();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_global_attr_error_attr_name_not_valid() {
    const INVALID_GLOBAL_ATTR_NAME: &str = "!invalid_name";
    const GLOBAL_ATTR_DATA: [i8; 3] = [1, 2, 3];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(INVALID_GLOBAL_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::GlobalAttributeNameNotValid(INVALID_GLOBAL_ATTR_NAME.to_string()),
        data_set.add_global_attr_i8(INVALID_GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap_err()
    );

    assert_eq!(0,       data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(INVALID_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(INVALID_GLOBAL_ATTR_NAME));
}

#[test]
fn test_add_var_attr_error_attr_already_exists() {
    const GLOBAL_ATTR_NAME: &str = "attr_1";
    const GLOBAL_ATTR_DATA_1: [i8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN_1: usize = GLOBAL_ATTR_DATA_1.len();
    const GLOBAL_ATTR_DATA_2: [i8; 4] = [4, 5, 6, 7];

    let mut data_set = DataSet::new();

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));

    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA_1.to_vec()).unwrap();

    assert_eq!(1,                               data_set.num_global_attrs());
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_1),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_1[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::GlobalAttributeAlreadyExists(GLOBAL_ATTR_NAME.to_string()),
        data_set.add_global_attr_i8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA_2.to_vec()).unwrap_err()
    );

    assert_eq!(1,                               data_set.num_global_attrs());
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_1),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_1[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
}

#[test]
fn test_rename_var_attr_error_attr_already_exists() {
    const GLOBAL_ATTR_NAME_1: &str = "attr_1";
    const GLOBAL_ATTR_DATA_1: [i8; 3] = [1, 2, 3];
    const GLOBAL_ATTR_DATA_LEN_1: usize = GLOBAL_ATTR_DATA_1.len();
    const GLOBAL_ATTR_NAME_2: &str = "attr_2";
    const GLOBAL_ATTR_DATA_2: [i8; 4] = [4, 5, 6, 7];
    const GLOBAL_ATTR_DATA_LEN_2: usize = GLOBAL_ATTR_DATA_2.len();

    let mut data_set = DataSet::new();

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_DATA_1.to_vec()).unwrap();
    data_set.add_global_attr_i8(GLOBAL_ATTR_NAME_2, GLOBAL_ATTR_DATA_2.to_vec()).unwrap();

    assert_eq!(2,                               data_set.num_global_attrs());
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_1),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_1[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_2),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_2[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    assert_eq!(
        InvalidDataSet::GlobalAttributeAlreadyExists(GLOBAL_ATTR_NAME_1.to_string()),
        data_set.add_global_attr_i8(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_DATA_2.to_vec()).unwrap_err()
    );

    assert_eq!(2,                               data_set.num_global_attrs());
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_1),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_1[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_1));
    assert_eq!(true,                            data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(GLOBAL_ATTR_DATA_LEN_2),    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(DataType::I8),              data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(Some(&GLOBAL_ATTR_DATA_2[..]),   data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
}

#[test]
fn test_rename_var_attr_error_attr_not_defined () {
    const UNDEF_GLOBAL_ATTR_NAME: &str = "undef_attr";
    const GLOBAL_ATTR_NAME_2: &str = "attr_2";

    let mut data_set = DataSet::new();

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));

    assert_eq!(
        InvalidDataSet::GlobalAttributeNotDefined(UNDEF_GLOBAL_ATTR_NAME.to_string()),
        data_set.rename_global_attr(UNDEF_GLOBAL_ATTR_NAME, GLOBAL_ATTR_NAME_2).unwrap_err()
    );

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(false,   data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
    assert_eq!(None,    data_set.get_global_attr_i8(GLOBAL_ATTR_NAME_2));
}

#[test]
fn test_remove_var_attr_error_attr_not_defined () {
    const UNDEF_GLOBAL_ATTR_NAME: &str = "undef_attr";

    let mut data_set = DataSet::new();

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));

    assert_eq!(
        InvalidDataSet::GlobalAttributeNotDefined(UNDEF_GLOBAL_ATTR_NAME.to_string()),
        data_set.remove_global_attr(UNDEF_GLOBAL_ATTR_NAME).unwrap_err()
    );

    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(false,   data_set.has_global_attr(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_len(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_data_type(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
    assert_eq!(None,    data_set.get_global_attr_i8(UNDEF_GLOBAL_ATTR_NAME));
}