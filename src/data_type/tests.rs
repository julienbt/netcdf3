#![cfg(test)]

use super::DataType;
use std::convert::TryFrom;

#[test]
fn test_data_type_display() {
    assert_eq!("DataType::I8", format!("{}", DataType::I8));
    assert_eq!("DataType::U8", format!("{}", DataType::U8));
    assert_eq!("DataType::I16", format!("{}", DataType::I16));
    assert_eq!("DataType::I32", format!("{}", DataType::I32));
    assert_eq!("DataType::F32", format!("{}", DataType::F32));
    assert_eq!("DataType::F64", format!("{}", DataType::F64));
}

#[test]
fn test_data_type_size_of_element() {
    assert_eq!(1, DataType::I8.size_of());
    assert_eq!(1, DataType::U8.size_of());
    assert_eq!(2, DataType::I16.size_of());
    assert_eq!(4, DataType::I32.size_of());
    assert_eq!(4, DataType::F32.size_of());
    assert_eq!(8, DataType::F64.size_of());
}

#[test]
fn test_data_type_c_api_name() {
    assert_eq!("NC_BYTE", DataType::I8.c_api_name());
    assert_eq!("NC_CHAR", DataType::U8.c_api_name());
    assert_eq!("NC_SHORT", DataType::I16.c_api_name());
    assert_eq!("NC_INT", DataType::I32.c_api_name());
    assert_eq!("NC_FLOAT", DataType::F32.c_api_name());
    assert_eq!("NC_DOUBLE", DataType::F64.c_api_name());
}

#[test]
fn test_data_type_try_from_u32() -> Result<(), &'static str> {

    assert_eq!(Err("Invalid value for a NetCDF-3 data type."), DataType::try_from(0_u32));
    assert_eq!(Ok(DataType::I8),                            DataType::try_from(1_u32));
    assert_eq!(Ok(DataType::U8),                            DataType::try_from(2_u32));
    assert_eq!(Ok(DataType::I16),                           DataType::try_from(3_u32));
    assert_eq!(Ok(DataType::I32),                           DataType::try_from(4_u32));
    assert_eq!(Ok(DataType::F32),                           DataType::try_from(5_u32));
    assert_eq!(Ok(DataType::F64),                           DataType::try_from(6_u32));
    assert_eq!(Err("Invalid value for a NetCDF-3 data type."), DataType::try_from(7_u32));

    Ok(())
}
