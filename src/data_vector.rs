mod tests;

use crate::DataType;

/// Wraps the six NetCDF-3 data types.
///
/// It allows to load variable data from files easily through the methods:
/// - [FileReader::read_all_vars](struct.FileReader.html#method.read_all_vars).
/// - [FileReader::read_var](struct.FileReader.html#method.read_var).
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use netcdf3::{FileReader, DataVector, DataType};
///
/// const LATITUDE_VAR_NAME: &str = "latitude";
/// const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 0.5, 1.0];
/// const NUM_VARS: usize = 9;
///
/// // ...
/// # use copy_to_tmp_file::{
/// #     copy_bytes_to_tmp_file,
/// #     NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
/// # };
/// #
/// # // Copy bytes to an temporary file
/// # let (tmp_dir, input_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);
///
/// // Read variable data from the file
/// // ---------------------------------
/// let mut file_reader: FileReader = FileReader::open(input_file_path).unwrap();
/// assert_eq!(NUM_VARS,                        file_reader.data_set().num_vars());
/// assert_eq!(true,                            file_reader.data_set().has_var(LATITUDE_VAR_NAME));
/// assert_eq!(DataType::F32,                   file_reader.data_set().var_data_type(LATITUDE_VAR_NAME).unwrap());
///
/// let mut data: HashMap<String, DataVector> = file_reader.read_all_vars().unwrap();
/// file_reader.close();
///
/// assert_eq!(NUM_VARS,                        data.len());
/// assert_eq!(true,                            data.contains_key(LATITUDE_VAR_NAME));
/// 
/// let latitude: DataVector = data.remove(LATITUDE_VAR_NAME).unwrap();
/// assert_eq!(DataType::F32,                   latitude.data_type());
/// let latitude: Vec<f32> = latitude.get_f32_into().unwrap();
/// assert_eq!(LATITUDE_VAR_DATA.to_vec(),      latitude);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DataVector {
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

impl DataVector {

    pub(crate) fn new(data_type: DataType, length: usize) -> Self {
        match data_type {
            DataType::I8 => DataVector::I8(vec![0; length]),
            DataType::U8 => DataVector::U8(vec![0; length]),
            DataType::I16 => DataVector::I16(vec![0; length]),
            DataType::I32 => DataVector::I32(vec![0; length]),
            DataType::F32 => DataVector::F32(vec![0.0; length]),
            DataType::F64 => DataVector::F64(vec![0.0; length]),
        }
    }

    /// Return the NetCDF-3 data type.
    pub fn data_type(&self) -> DataType {
        match self {
            DataVector::I8(_) => DataType::I8,
            DataVector::U8(_) => DataType::U8,
            DataVector::I16(_) => DataType::I16,
            DataVector::I32(_) => DataType::I32,
            DataVector::F32(_) => DataType::F32,
            DataVector::F64(_) => DataType::F64,
        }
    }

    /// Return the length (the number of elements) of the vector.
    pub fn len(&self) -> usize {
        match self {
            DataVector::I8(data) => data.len(),
            DataVector::U8(data) => data.len(),
            DataVector::I16(data) => data.len(),
            DataVector::I32(data) => data.len(),
            DataVector::F32(data) => data.len(),
            DataVector::F64(data) => data.len(),
        }
    }

    /// Returns a slice to the internal `Vec<i8>`.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataVector, DataType};
    ///
    /// let data_vec = DataVector::I8(vec![1_i8, 2, 3]);
    ///
    /// assert_eq!(DataType::I8,                data_vec.data_type());
    ///
    /// assert_eq!(Some(&[1_i8, 2, 3][..]),     data_vec.get_i8());
    /// assert_eq!(None,                        data_vec.get_u8());
    /// assert_eq!(None,                        data_vec.get_i16());
    /// assert_eq!(None,                        data_vec.get_i32());
    /// assert_eq!(None,                        data_vec.get_f32());
    /// assert_eq!(None,                        data_vec.get_f64());
    /// ```
    pub fn get_i8(&self) -> Option<&[i8]> {
        return match self {
            DataVector::I8(data) => Some(data),
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    /// Returns a slice to the internal `Vec<u8>`.
    ///
    /// Also see the method [get_i8](enum.DataVector.html#method.get_i8).
    pub fn get_u8(&self) -> Option<&[u8]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(data) => Some(data),
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    pub(crate) fn get_as_string(&self) -> Option<String> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(data) => String::from_utf8(data.to_vec()).ok(),
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    /// Returns a slice to the internal `Vec<i16>`.
    ///
    /// Also see the method [get_i8](enum.DataVector.html#method.get_i8).
    pub fn get_i16(&self) -> Option<&[i16]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(data) => Some(data),
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    /// Returns a slice to the internal `Vec<i32>`.
    ///
    /// Also see the method [get_i8](enum.DataVector.html#method.get_i8).
    pub fn get_i32(&self) -> Option<&[i32]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(data) => Some(data),
            DataVector::F32(_) => None,
            DataVector::F64(_) => None,
        };
    }

    /// Returns a slice to the internal `Vec<f32>`.
    ///
    /// Also see the method [get_i8](enum.DataVector.html#method.get_i8).
    pub fn get_f32(&self) -> Option<&[f32]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(data) => Some(data),
            DataVector::F64(_) => None,
        };
    }

    /// Returns a slice to the internal `Vec<f64>`.
    ///
    /// Also see the method [get_i8](enum.DataVector.html#method.get_i8).
    pub fn get_f64(&self) -> Option<&[f64]> {
        return match self {
            DataVector::I8(_) => None,
            DataVector::U8(_) => None,
            DataVector::I16(_) => None,
            DataVector::I32(_) => None,
            DataVector::F32(_) => None,
            DataVector::F64(data) => Some(data),
        };
    }

    /// Returns the internal `Vec<i8>` if the `DataVector` contains one.
    ///
    /// Otherwise the instance of the `DataVector` is returned as an errror.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataVector, DataType};
    ///
    /// let data_1: Vec<i8> = vec![1, 2 ,3];
    /// let ptr_1 : *const i8 = data_1.as_ptr();
    ///
    /// // Frirst create a `DataVector::I8`
    /// let data_vec: DataVector = DataVector::I8(data_1);
    /// assert_eq!(DataType::I8,               data_vec.data_type());
    ///
    /// // Try to extract the internal vector with the wrong data types
    /// let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
    /// let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
    /// let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
    /// let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();
    /// let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();
    ///
    /// // Extract the internal vector with the good data type
    /// let data_2: Vec<i8> = data_vec.get_i8_into().unwrap();
    /// let ptr_2 : *const i8 = data_2.as_ptr();
    ///
    /// assert_eq!(vec![1, 2, 3],           data_2);
    ///
    /// // No copy of the buffer has been done
    /// assert_eq!(ptr_1,                   ptr_2);
    /// ```
    pub fn get_i8_into(self) -> Result<Vec<i8>, DataVector> {
        if let DataVector::I8(data) = self {
            return Ok(data);
        }
        return Err(self);
    }

    pub fn get_u8_into(self) -> Result<Vec<u8>, DataVector> {
        if let DataVector::U8(data) = self {
            return Ok(data);
        }
        return Err(self);
    }

    pub fn get_i16_into(self) -> Result<Vec<i16>, DataVector> {
        if let DataVector::I16(data) = self {
            return Ok(data);
        }
        return Err(self);
    }

    pub fn get_i32_into(self) -> Result<Vec<i32>, DataVector> {
        if let DataVector::I32(data) = self {
            return Ok(data);
        }
        return Err(self);
    }

    pub fn get_f32_into(self) -> Result<Vec<f32>, DataVector> {
        if let DataVector::F32(data) = self {
            return Ok(data);
        }
        return Err(self);
    }

    pub fn get_f64_into(self) -> Result<Vec<f64>, DataVector> {
        if let DataVector::F64(data) = self {
            return Ok(data);
        }
        return Err(self);
    }
}
