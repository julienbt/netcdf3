mod tests;

/// Name of the `DataType::I8` (a.k.a. `NC_BYTE`) used in the NetCDF C-API.
const I8_TYPE_C_API_NAME: &'static str = "NC_BYTE";
/// Name of the `DataType::U8` (a.k.a. `NC_CHAR`) used in the NetCDF C-API.
const U8_TYPE_C_API_NAME: &'static str = "NC_CHAR";
/// Name of the `DataType::I16` (a.k.a. `NC_SHORT`) used in the NetCDF C-API.
const I16_TYPE_C_API_NAME: &'static str = "NC_SHORT";
/// Name of the `DataType::I32` (a.k.a. `NC_INT`) used in the NetCDF C-API.
const I32_TYPE_C_API_NAME: &'static str = "NC_INT";
/// Name of the `DataType::F32` (a.k.a. `NC_FLOAT`) used in the NetCDF C-API.
const F32_TYPE_C_API_NAME: &'static str = "NC_FLOAT";
/// Name of the `DataType::F64` (a.k.a. `NC_DOUBLE`) used in the NetCDF C-API.
const F64_TYPE_C_API_NAME: &'static str = "NC_DOUBLE";

/// All the data types supported by the NetCDF-3 format
///
/// # Example
///
/// ```
/// use netcdf3::{DataType};
///
/// assert_eq!("NC_BYTE",   DataType::I8.c_api_name());
/// assert_eq!("NC_CHAR",   DataType::U8.c_api_name());
/// assert_eq!("NC_SHORT",  DataType::I16.c_api_name());
/// assert_eq!("NC_INT",    DataType::I32.c_api_name());
/// assert_eq!("NC_FLOAT",  DataType::F32.c_api_name());
/// assert_eq!("NC_DOUBLE", DataType::F64.c_api_name());
///
/// assert_eq!(1, DataType::I8.size_of());
/// assert_eq!(1, DataType::U8.size_of());
/// assert_eq!(2, DataType::I16.size_of());
/// assert_eq!(4, DataType::I32.size_of());
/// assert_eq!(4, DataType::F32.size_of());
/// assert_eq!(8, DataType::F64.size_of());
/// ```
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    /// 8-bit signed integer, a.k.a. `NC_BYTE`
    I8 = 1,
    /// 8-bit character, a.k.a. `NC_CHAR`
    U8 = 2,
    /// 16-bit signed integer, a.k.a. `NC_SHORT`
    I16 = 3,
    /// 32-bit signed integer, a.k.a. `NC_INT`
    I32 = 4,
    /// 32-bit floating-point number, a.k.a. `NC_FLOAT`
    F32 = 5,
    /// 64-bit floating-point number, a.k.a. `NC_DOUBLE`
    F64 = 6,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "DataType::{}",
            match self {
                DataType::I8 => "I8",
                DataType::U8 => "U8",
                DataType::I16 => "I16",
                DataType::I32 => "I32",
                DataType::F32 => "F32",
                DataType::F64 => "F64",
            }
        )
    }
}

impl std::convert::TryFrom<u32> for DataType {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<DataType, &'static str> {
        match value {
            1_u32 => Ok(DataType::I8),
            2_u32 => Ok(DataType::U8),
            3_u32 => Ok(DataType::I16),
            4_u32 => Ok(DataType::I32),
            5_u32 => Ok(DataType::F32),
            6_u32 => Ok(DataType::F64),
            _ => Err("Invalid value for a NetCDF-3 data type."),
        }
    }
}

impl DataType {
    /// Returns the size (in bytes) of one element of `DataType`.
    ///
    /// # Example
    ///
    /// ```
    /// # use netcdf3::DataType;
    /// assert_eq!(1, DataType::I8.size_of());
    /// assert_eq!(1, DataType::U8.size_of());
    /// assert_eq!(2, DataType::I16.size_of());
    /// assert_eq!(4, DataType::I32.size_of());
    /// assert_eq!(4, DataType::F32.size_of());
    /// assert_eq!(8, DataType::F64.size_of());
    /// ```
    pub fn size_of(&self) -> usize {
        match self {
            DataType::I8 => std::mem::size_of::<i8>(),
            DataType::U8 => std::mem::size_of::<u8>(),
            DataType::I16 => std::mem::size_of::<i16>(),
            DataType::I32 => std::mem::size_of::<i32>(),
            DataType::F32 => std::mem::size_of::<f32>(),
            DataType::F64 => std::mem::size_of::<f64>(),
        }
    }

    /// Returns the name of the `DataType` commoly used in the NedCDF C API.
    ///
    /// # Example
    ///
    /// ```
    /// # use netcdf3::DataType;
    /// assert_eq!("NC_BYTE", DataType::I8.c_api_name());
    /// assert_eq!("NC_CHAR", DataType::U8.c_api_name());
    /// assert_eq!("NC_SHORT", DataType::I16.c_api_name());
    /// assert_eq!("NC_INT", DataType::I32.c_api_name());
    /// assert_eq!("NC_FLOAT", DataType::F32.c_api_name());
    /// assert_eq!("NC_DOUBLE", DataType::F64.c_api_name());
    /// ```
    ///
    /// # See also
    ///
    /// The [NetCDF C-API](https://github.com/Unidata/netcdf-c/blob/master/include/netcdf.h)
    pub fn c_api_name(&self) -> &'static str {
        match self {
            DataType::I8 => I8_TYPE_C_API_NAME,
            DataType::U8 => U8_TYPE_C_API_NAME,
            DataType::I16 => I16_TYPE_C_API_NAME,
            DataType::I32 => I32_TYPE_C_API_NAME,
            DataType::F32 => F32_TYPE_C_API_NAME,
            DataType::F64 => F64_TYPE_C_API_NAME,
        }
    }
}
