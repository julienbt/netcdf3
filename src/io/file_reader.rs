use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use byteorder::{BigEndian, ReadBytesExt};

use nom::Parser;
use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    combinator::{map_res, verify},
    multi::many_m_n,
    number::streaming::{be_f32, be_f64, be_i16, be_i32, be_i64, be_i8, be_u32, be_u8},
};

use crate::{
    data_set::DimensionSize,
    error::parse_header_error::{NomError, ParseHeaderError, ParseHeaderErrorKind},
    error::ReadError,
    io::{compute_padding_size, Offset, ABSENT_TAG, ATTRIBUTE_TAG, DIMENSION_TAG, VARIABLE_TAG},
    DataSet, DataType, DataVector, Dimension, Variable, Version,
};

pub trait SeekRead: Seek + Read {}
impl<T: Seek + Read> SeekRead for T {}

impl Debug for dyn SeekRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:p}", self)
    }
}

/// Allows to read NetCDF-3 files (the *classic* and the *64-bit offset* versions).
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use netcdf3::{FileReader, DataSet, DataVector, DataType, Version, DimensionType};
///
/// const LATITUDE_DIM_NAME: &str = "latitude";
/// const LATITUDE_VAR_NAME: &str = LATITUDE_DIM_NAME;
/// const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 0.5, 1.0];
/// const LATITUDE_VAR_LEN: usize = LATITUDE_VAR_DATA.len();
///
/// const LONGITUDE_DIM_NAME: &str = "longitude";
/// const LONGITUDE_VAR_NAME: &str = LONGITUDE_DIM_NAME;
/// const LONGITUDE_VAR_DATA: [f32; 5] = [0.0, 0.5, 1.0, 1.5, 2.0];
/// const LONGITUDE_VAR_LEN: usize = LONGITUDE_VAR_DATA.len();
///
/// const TIME_DIM_NAME: &str = "time";
/// const TIME_VAR_NAME: &str = TIME_DIM_NAME;
/// const TIME_VAR_DATA: [f32; 2] = [438_300.0, 438_324.0];
/// const TIME_VAR_LEN: usize = TIME_VAR_DATA.len();
///
/// const TEMP_I8_VAR_NAME: &str = "temperature_i8";
/// const TEMP_I8_VAR_DATA: [i8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
/// const TEMP_I8_VAR_LEN: usize = TEMP_I8_VAR_DATA.len();
///
/// const TEMP_U8_VAR_NAME: &str = "temperature_u8";
/// const TEMP_U8_VAR_DATA: [u8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
/// const TEMP_U8_VAR_LEN: usize = TEMP_U8_VAR_DATA.len();
///
/// const TEMP_I16_VAR_NAME: &str = "temperature_i16";
/// const TEMP_I16_VAR_DATA: [i16; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
/// const TEMP_I16_VAR_LEN: usize = TEMP_I16_VAR_DATA.len();
///
/// const TEMP_I32_VAR_NAME: &str = "temperature_i32";
/// const TEMP_I32_VAR_DATA: [i32; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
/// const TEMP_I32_VAR_LEN: usize = TEMP_I32_VAR_DATA.len();
///
/// const TEMP_F32_VAR_NAME: &str = "temperature_f32";
/// const TEMP_F32_VAR_DATA: [f32; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];
/// const TEMP_F32_VAR_LEN: usize = TEMP_F32_VAR_DATA.len();
///
/// const TEMP_F64_VAR_NAME: &str = "temperature_f64";
/// const TEMP_F64_VAR_DATA: [f64; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];
/// const TEMP_F64_VAR_LEN: usize = TEMP_F64_VAR_DATA.len();
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
/// // Open the file and read the header
/// // ---------------------------------
/// let mut file_reader: FileReader = FileReader::open(input_file_path).unwrap();
///
/// let data_set: &DataSet = file_reader.data_set();
///
/// // Get the NetCDf-3 version
/// // ------------------------
/// assert_eq!(Version::Classic,                    file_reader.version());
///
/// // Get the global attributes
/// // --------------------------
/// assert_eq!(2,                                   data_set.num_global_attrs());
/// assert_eq!("Example of NETCDF3_CLASSIC file",   data_set.get_global_attr_as_string("title").unwrap());
/// assert_eq!("CF-1.8",                            data_set.get_global_attr_as_string("Conventions").unwrap());
///
/// // Get the dimensions
/// // ------------------
/// assert_eq!(3,                                   data_set.num_dims());
///
/// assert_eq!(true,                                data_set.has_dim(LATITUDE_DIM_NAME));
/// assert_eq!(Some(LATITUDE_VAR_LEN),              data_set.dim_size(LATITUDE_DIM_NAME));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(LATITUDE_DIM_NAME));
///
/// assert_eq!(true,                                data_set.has_dim(LONGITUDE_DIM_NAME));
/// assert_eq!(Some(LONGITUDE_VAR_LEN),             data_set.dim_size(LONGITUDE_DIM_NAME));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(LONGITUDE_DIM_NAME));
///
/// assert_eq!(true,                                data_set.has_dim(TIME_DIM_NAME));
/// assert_eq!(Some(TIME_VAR_LEN),                  data_set.dim_size(TIME_DIM_NAME));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.dim_type(TIME_DIM_NAME));
///
/// // Get the variable definitions
/// // ----------------------------
/// assert_eq!(9,                                   data_set.num_vars());
///
/// assert_eq!(true,                                data_set.has_var(LATITUDE_VAR_NAME));
/// assert_eq!(Some(DataType::F32),                 data_set.var_data_type(LATITUDE_VAR_NAME));
/// assert_eq!(Some(false),                         data_set.is_record_var(LATITUDE_VAR_NAME));
/// assert_eq!(Some(LATITUDE_VAR_LEN),              data_set.var_len(LATITUDE_VAR_NAME));
///
/// // ..
///
/// // Get the variable attributes
/// // ---------------------------
/// assert_eq!(Some(4),                             data_set.num_var_attrs(LATITUDE_VAR_NAME));
/// assert_eq!("latitude",                          data_set.get_var_attr_as_string(LATITUDE_VAR_NAME, "standard_name").unwrap());
/// assert_eq!("LATITUDE",                          data_set.get_var_attr_as_string(LATITUDE_VAR_NAME, "long_name").unwrap());
/// assert_eq!("degrees_north",                     data_set.get_var_attr_as_string(LATITUDE_VAR_NAME, "units").unwrap());
/// assert_eq!("Y",                                 data_set.get_var_attr_as_string(LATITUDE_VAR_NAME, "axis").unwrap());
///
/// assert_eq!(Some(3),                             data_set.num_var_attrs(TEMP_F32_VAR_NAME));
/// assert_eq!("air_temperature",                   data_set.get_var_attr_as_string(TEMP_F32_VAR_NAME, "standard_name").unwrap());
/// assert_eq!("TEMPERATURE",                       data_set.get_var_attr_as_string(TEMP_F32_VAR_NAME, "long_name").unwrap());
/// assert_eq!("Celsius",                           data_set.get_var_attr_as_string(TEMP_F32_VAR_NAME, "units").unwrap());
///
/// // ...
///
/// // Read all the variables
/// // ----------------------
/// let variables: HashMap<String, DataVector> = file_reader.read_all_vars().unwrap();
/// let data_set: &DataSet = file_reader.data_set();
/// assert_eq!(9,                                   variables.len());
///
///
/// assert_eq!(true,                                variables.contains_key(LATITUDE_VAR_NAME));
/// assert_eq!(DataType::F32,                       variables[LATITUDE_VAR_NAME].data_type());
/// assert_eq!(Some(&LATITUDE_VAR_DATA[..]),        variables[LATITUDE_VAR_NAME].get_f32());
///
/// assert_eq!(true,                                variables.contains_key(LONGITUDE_VAR_NAME));
/// assert_eq!(DataType::F32,                       variables[LONGITUDE_VAR_NAME].data_type());
/// assert_eq!(Some(&LONGITUDE_VAR_DATA[..]),       variables[LONGITUDE_VAR_NAME].get_f32());
///
/// assert_eq!(true,                                variables.contains_key(TIME_VAR_NAME));
/// assert_eq!(DataType::F32,                       variables[TIME_VAR_NAME].data_type());
/// assert_eq!(Some(&TIME_VAR_DATA[..]),            variables[TIME_VAR_NAME].get_f32());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_I8_VAR_NAME));
/// assert_eq!(DataType::I8,                        variables[TEMP_I8_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_I8_VAR_DATA[..]),         variables[TEMP_I8_VAR_NAME].get_i8());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_U8_VAR_NAME));
/// assert_eq!(DataType::U8,                        variables[TEMP_U8_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_U8_VAR_DATA[..]),         variables[TEMP_U8_VAR_NAME].get_u8());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_I16_VAR_NAME));
/// assert_eq!(DataType::I16,                       variables[TEMP_I16_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_I16_VAR_DATA[..]),        variables[TEMP_I16_VAR_NAME].get_i16());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_I32_VAR_NAME));
/// assert_eq!(DataType::I32,                       variables[TEMP_I32_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_I32_VAR_DATA[..]),        variables[TEMP_I32_VAR_NAME].get_i32());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_F32_VAR_NAME));
/// assert_eq!(DataType::F32,                       variables[TEMP_F32_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_F32_VAR_DATA[..]),        variables[TEMP_F32_VAR_NAME].get_f32());
///
/// assert_eq!(true,                                variables.contains_key(TEMP_F64_VAR_NAME));
/// assert_eq!(DataType::F64,                       variables[TEMP_F64_VAR_NAME].data_type());
/// assert_eq!(Some(&TEMP_F64_VAR_DATA[..]),        variables[TEMP_F64_VAR_NAME].get_f64());
/// // ...
/// # tmp_dir.close();
/// ```
#[derive(Debug)]
pub struct FileReader {
    data_set: DataSet,
    version: Version,
    input_file_path: PathBuf,
    input_file: Box<dyn SeekRead>,
    vars_info: Vec<VariableParsedMetadata>,
}

macro_rules! impl_read_typed_var {
    ($func_name:ident, $prim_type:ty, $data_type:path, $data_vector:path) => {
        /// Reads the typed variable and returns its values into a typed `Vec`.
        pub fn $func_name(&mut self, var_name: &str) -> Result<Vec<$prim_type>, ReadError> {
            let (_var_index, var): (usize, &Variable) = self
                .data_set
                .find_var_from_name(var_name)
                .map_err(|_err| ReadError::VariableNotDefined(String::from(var_name)))?;
            if var.data_type != $data_type {
                return Err(ReadError::VariableMismatchDataType {
                    var_name: String::from(var_name),
                    req: var.data_type.clone(),
                    get: $data_type,
                });
            }
            let data_vec: DataVector = self.read_var(var_name)?;
            match data_vec {
                $data_vector(data) => return Ok(data),
                _ => return Err(ReadError::Unexpected), // previously checked
            }
        }
    };
}

macro_rules! impl_read_typed_record {
    ($func_name:ident, $prim_type:ty, $data_type:path, $data_vector:path) => {
        /// Reads the typed records and returns its values into a typed`Vec`.
        pub fn $func_name(
            &mut self,
            var_name: &str,
            record_index: usize,
        ) -> Result<Vec<$prim_type>, ReadError> {
            let (_var_index, var): (usize, &Variable) = self
                .data_set
                .find_var_from_name(var_name)
                .map_err(|_err| ReadError::VariableNotDefined(String::from(var_name)))?;
            if var.data_type != $data_type {
                return Err(ReadError::VariableMismatchDataType {
                    var_name: String::from(var_name),
                    req: var.data_type.clone(),
                    get: $data_type,
                });
            }
            let data_vec: DataVector = self.read_record(var_name, record_index)?;
            match data_vec {
                $data_vector(data) => return Ok(data),
                _ => return Err(ReadError::Unexpected), // previously checked
            };
        }
    };
}

impl FileReader {
    /// Returns the data set managed by the reader.
    pub fn data_set(&self) -> &DataSet {
        &self.data_set
    }

    pub fn version(&self) -> Version {
        self.version.clone()
    }

    /// Returns the data set managed by the reader.
    pub fn file_path(&self) -> &std::path::Path {
        &self.input_file_path
    }

    pub fn open_seek_read(
        input_file_name: &str,
        mut input_file: Box<dyn SeekRead>,
    ) -> Result<Self, ReadError> {
        let input_file_path: PathBuf = PathBuf::from(input_file_name);

        // determine length as in use https://doc.rust-lang.org/stable/src/std/io/mod.rs.html#1871-1882
        let pos = input_file.stream_position()?;
        let len = input_file.seek(SeekFrom::End(0))?;
        if pos != len {
            input_file.seek(SeekFrom::Start(pos))?;
        }

        Self::read_header(input_file_path, input_file, len)
    }

    /// Opens the file and parses the header of the NetCDF-3.
    pub fn open<P: AsRef<Path>>(input_file_path: P) -> Result<Self, ReadError> {
        let input_file_path: PathBuf = {
            let mut path = PathBuf::new();
            path.push(input_file_path);
            path
        };
        let input_file: Box<dyn SeekRead> = Box::new(std::fs::File::open(input_file_path.clone())?);
        let file_size = std::fs::metadata(&input_file_path)?.len();

        Self::read_header(input_file_path, input_file, file_size)
    }

    /// Opens the file and parses the header of the NetCDF-3.
    fn read_header(
        input_file_path: PathBuf,
        mut input_file: Box<dyn SeekRead>,
        file_size: u64,
    ) -> Result<Self, ReadError> {
        const BUFFER_SIZE: usize = 1024;

        // Parse the header
        let (data_set, version, vars_info): (DataSet, Version, Vec<VariableParsedMetadata>) = {
            let mut buffer: Vec<u8> = vec![];
            let (data_set, version, vars_info): (DataSet, Version, Vec<VariableParsedMetadata>);
            loop {
                // Load bytes
                let old_buf_start: usize = buffer.len();
                let new_buf_size: usize =
                    std::cmp::min((buffer.len() + BUFFER_SIZE) as u64, file_size) as usize;
                let start: &usize = &old_buf_start;
                let end: &usize = &new_buf_size;
                buffer.resize(new_buf_size, 0_u8);
                let _num_of_bytes = input_file.read(&mut buffer[*start..*end])?;

                // TODO: do not cast file_size to usize, instead make parse_header() work with u64
                let parsing_result: Result<
                    (DataSet, Version, Vec<VariableParsedMetadata>),
                    ReadError,
                > = FileReader::parse_header(&buffer, file_size as usize);
                match parsing_result {
                    Ok((data_set_2, version_2, vars_info_2)) => {
                        data_set = data_set_2;
                        version = version_2;
                        vars_info = vars_info_2;
                        break;
                    }
                    Err(read_err) => {
                        if read_err.header_is_incomplete() {
                            let buf_size: u64 = buffer.len() as u64;
                            if buf_size < file_size {
                                // nothing to do
                            } else {
                                return Err(read_err);
                            }
                        } else {
                            return Err(read_err);
                        }
                    }
                }
            }
            (data_set, version, vars_info)
        };

        // Return the result
        Ok(FileReader {
            data_set,
            version,
            input_file_path,
            input_file,
            vars_info, // convert the list of tuples to a map
        })
    }

    /// Closes the file and releases the data set and the file version.
    pub fn close(self) -> (DataSet, Version) {
        (self.data_set, self.version)
    }

    /// Allows to read all variable data easily.
    ///
    /// Also see an example [here](struct.FileReader.html#example).
    pub fn read_all_vars(&mut self) -> Result<HashMap<String, DataVector>, ReadError> {
        let var_names: Vec<String> = self.data_set.get_var_names();
        var_names
            .into_iter()
            .map(|var_name: String| {
                let var_data: DataVector = self.read_var(&var_name)?;
                Ok((var_name, var_data))
            })
            .collect()
    }

    /// Reads the typed variable and returns its values into `Vec`.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{FileReader, DataSet, DataVector, DataType};
    ///
    /// const LATITUDE_VAR_NAME: &str = "latitude";
    /// const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 0.5, 1.0];
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
    /// let mut file_reader: FileReader = FileReader::open(input_file_path).unwrap();
    ///
    /// // Open the file
    /// // -------------
    /// assert_eq!(true,                    file_reader.data_set().has_var(LATITUDE_VAR_NAME));
    /// assert_eq!(Some(DataType::F32),     file_reader.data_set().var_data_type(LATITUDE_VAR_NAME));
    ///
    /// // Read the variable
    /// // -----------------
    /// // using the method `FileReader::read_var`
    /// {
    ///     let latitudes: DataVector = file_reader.read_var(LATITUDE_VAR_NAME).unwrap();
    ///     assert_eq!(DataType::F32,                           latitudes.data_type());
    ///
    ///     assert_eq!(None,                                    latitudes.get_i8());
    ///     assert_eq!(None,                                    latitudes.get_u8());
    ///     assert_eq!(None,                                    latitudes.get_i16());
    ///     assert_eq!(None,                                    latitudes.get_i32());
    ///     assert_eq!(Some(&LATITUDE_VAR_DATA[..]),            latitudes.get_f32());
    ///     assert_eq!(None,                                    latitudes.get_f64());
    /// }
    ///
    /// // using the method `FileReader::read_var_f32`
    /// {
    ///     let latitudes: Vec<f32> = file_reader.read_var_f32(LATITUDE_VAR_NAME).unwrap();
    ///     assert_eq!(&LATITUDE_VAR_DATA[..],                  &latitudes[..]);
    /// }
    /// ```
    pub fn read_var(&mut self, var_name: &str) -> Result<DataVector, ReadError> {
        let (_, var): (usize, &Variable) = self
            .data_set
            .find_var_from_name(var_name)
            .map_err(|_err| ReadError::VariableNotDefined(String::from(var_name)))?;
        let record_size: usize = self.data_set.record_size().unwrap_or(0);
        let num_records: usize = self.data_set.num_records().unwrap_or(0);
        let begin_offset: u64 = {
            let var_info: &VariableParsedMetadata =
                self.find_var_info(var_name).ok_or(ReadError::Unexpected)?;
            i64::from(var_info.begin_offset.clone()) as u64
        };
        let data_type: DataType = var.data_type();
        let chunk_len: usize = var.chunk_len();
        let padding_size: usize = {
            let num_bytes: usize = chunk_len * data_type.size_of();
            compute_padding_size(num_bytes)
        };
        let input = &mut self.input_file;
        input.seek(SeekFrom::Start(begin_offset))?;
        // memory allocation
        let mut data_vec = DataVector::new(data_type, var.len());
        if !var.is_record_var() {
            match data_vec {
                DataVector::I8(ref mut data) => input.read_i8_into(&mut data[..]),
                DataVector::U8(ref mut data) => input.read_exact(&mut data[..]),
                DataVector::I16(ref mut data) => input.read_i16_into::<BigEndian>(&mut data[..]),
                DataVector::I32(ref mut data) => input.read_i32_into::<BigEndian>(&mut data[..]),
                DataVector::F32(ref mut data) => input.read_f32_into::<BigEndian>(&mut data[..]),
                DataVector::F64(ref mut data) => input.read_f64_into::<BigEndian>(&mut data[..]),
            }?;
            if padding_size > 0 {
                input.seek(SeekFrom::Current(padding_size as i64))?;
            }
        } else {
            let chunk_size: usize = var.chunk_size();

            let offset_size: i64 = (record_size + padding_size - chunk_size) as i64;
            for i in 0_usize..num_records {
                // reader.seek(SeekFrom::)
                let start: usize = i * chunk_len;
                let end: usize = (i + 1) * chunk_len;
                match data_vec {
                    DataVector::I8(ref mut data) => input.read_i8_into(&mut data[start..end]),
                    DataVector::U8(ref mut data) => input.read_exact(&mut data[start..end]),
                    DataVector::I16(ref mut data) => {
                        input.read_i16_into::<BigEndian>(&mut data[start..end])
                    }
                    DataVector::I32(ref mut data) => {
                        input.read_i32_into::<BigEndian>(&mut data[start..end])
                    }
                    DataVector::F32(ref mut data) => {
                        input.read_f32_into::<BigEndian>(&mut data[start..end])
                    }
                    DataVector::F64(ref mut data) => {
                        input.read_f64_into::<BigEndian>(&mut data[start..end])
                    }
                }?;
                input.seek(SeekFrom::Current(offset_size))?;
            }
        }
        Ok(data_vec)
    }

    impl_read_typed_var!(read_var_i8, i8, DataType::I8, DataVector::I8);
    impl_read_typed_var!(read_var_u8, u8, DataType::U8, DataVector::U8);
    impl_read_typed_var!(read_var_i16, i16, DataType::I16, DataVector::I16);
    impl_read_typed_var!(read_var_i32, i32, DataType::I32, DataVector::I32);
    impl_read_typed_var!(read_var_f32, f32, DataType::F32, DataVector::F32);
    impl_read_typed_var!(read_var_f64, f64, DataType::F64, DataVector::F64);

    /// Reads the typed records and returns its values into a typed`Vec`.
    pub fn read_record(
        &mut self,
        var_name: &str,
        record_index: usize,
    ) -> Result<DataVector, ReadError> {
        let (_var_index, var): (usize, &Variable) = self
            .data_set
            .find_var_from_name(var_name)
            .map_err(|_err| ReadError::VariableNotDefined(String::from(var_name)))?;
        let num_records: usize = self.data_set.num_records().unwrap_or(1); // fixed-size variables haves exaclty one record
        if record_index >= num_records {
            return Err(ReadError::RecordIndexExceeded {
                index: record_index,
                num_records,
            });
        }

        // Compute the record offset from the start of the NetCDF3 file
        let var_info: &VariableParsedMetadata =
            self.find_var_info(var_name).ok_or(ReadError::Unexpected)?;
        let record_offset: u64 = (i64::from(var_info.begin_offset.clone()) as u64)
            + ((record_index * self.data_set.record_size().unwrap_or(0)) as u64);
        self.input_file.seek(SeekFrom::Start(record_offset))?;

        // Read the data
        let data_type: DataType = var.data_type();
        let mut data_vec: DataVector = DataVector::new(data_type, var.chunk_len());
        match data_vec {
            DataVector::I8(ref mut data) => self.input_file.read_i8_into(&mut data[..]),
            DataVector::U8(ref mut data) => self.input_file.read_exact(&mut data[..]),
            DataVector::I16(ref mut data) => {
                self.input_file.read_i16_into::<BigEndian>(&mut data[..])
            }
            DataVector::I32(ref mut data) => {
                self.input_file.read_i32_into::<BigEndian>(&mut data[..])
            }
            DataVector::F32(ref mut data) => {
                self.input_file.read_f32_into::<BigEndian>(&mut data[..])
            }
            DataVector::F64(ref mut data) => {
                self.input_file.read_f64_into::<BigEndian>(&mut data[..])
            }
        }?;
        Ok(data_vec)
    }

    impl_read_typed_record!(read_record_i8, i8, DataType::I8, DataVector::I8);
    impl_read_typed_record!(read_record_u8, u8, DataType::U8, DataVector::U8);
    impl_read_typed_record!(read_record_i16, i16, DataType::I16, DataVector::I16);
    impl_read_typed_record!(read_record_i32, i32, DataType::I32, DataVector::I32);
    impl_read_typed_record!(read_record_f32, f32, DataType::F32, DataVector::F32);
    impl_read_typed_record!(read_record_f64, f64, DataType::F64, DataVector::F64);

    /// Parses the NetCDF-3 header
    fn parse_header(
        input: &[u8],
        total_file_size: usize,
    ) -> Result<(DataSet, Version, Vec<VariableParsedMetadata>), ReadError> {
        // the magic word
        let (input, _): (&[u8], &[u8]) = FileReader::parse_magic_word(input)?;
        // the version number
        let (input, version): (&[u8], Version) = FileReader::parse_version(input)?;

        // the number of records
        let (input, num_records): (&[u8], Option<usize>) =
            FileReader::parse_as_usize_optional(input)?;
        let (input, dims_list): (&[u8], Vec<(String, usize)>) = FileReader::parse_dims_list(input)?;
        let (input, global_attrs_list): (&[u8], Vec<_>) = FileReader::parse_attrs_list(input)?;
        let (_input, var_info_list): (&[u8], Vec<VariableParsedMetadata>) =
            FileReader::parse_vars_list(input, version.clone())?;

        // Create a new dataset
        let mut data_set = DataSet::new();
        let (num_records, num_records_is_determinated): (usize, bool) = match num_records {
            Some(num_records) => (num_records, true),
            None => (0, false),
        };

        // Append it the dimensions
        for (dim_name, dim_size) in dims_list.into_iter() {
            if dim_size == 0 {
                data_set.set_unlimited_dim(dim_name, num_records)?;
            } else {
                data_set.add_fixed_dim(dim_name, dim_size)?;
            }
        }

        // Append ot the global attributes
        for (attr_name, attr_data) in global_attrs_list.into_iter() {
            use DataVector::*;
            match attr_data {
                I8(data) => {
                    data_set.add_global_attr_i8(&attr_name, data)?;
                }
                U8(data) => {
                    data_set.add_global_attr_u8(&attr_name, data)?;
                }
                I16(data) => {
                    data_set.add_global_attr_i16(&attr_name, data)?;
                }
                I32(data) => {
                    data_set.add_global_attr_i32(&attr_name, data)?;
                }
                F32(data) => data_set.add_global_attr_f32(&attr_name, data)?,
                F64(data) => {
                    data_set.add_global_attr_f64(&attr_name, data)?;
                }
            }
        }

        // Append the variables
        let mut record_var_begin_offsets: Vec<Offset> = vec![]; // used to computed the number of records if necessaray
        for var_info in var_info_list.iter() {
            let dim_refs: Vec<Rc<Dimension>> = data_set.get_dims_from_dim_ids(&var_info.dim_ids)?;
            // Create the variable the variable
            let var: &Variable = data_set.add_var_using_dim_refs(
                &var_info.name,
                dim_refs,
                var_info.data_type.clone(),
            )?;
            // Keep the `begin_offset` of the variable
            if var.is_record_var() {
                record_var_begin_offsets.push(var_info.begin_offset.clone());
            }
            // Append variable attributes
            let var_name: String = var_info.name.clone();
            for (attr_name, attr_data) in var_info.attrs_list.iter() {
                use DataVector::*;
                match attr_data {
                    I8(data) => {
                        data_set.add_var_attr_i8(&var_name, attr_name, data.clone())?;
                    }
                    U8(data) => {
                        data_set.add_var_attr_u8(&var_name, attr_name, data.clone())?;
                    }
                    I16(data) => {
                        data_set.add_var_attr_i16(&var_name, attr_name, data.clone())?;
                    }
                    I32(data) => {
                        data_set.add_var_attr_i32(&var_name, attr_name, data.clone())?;
                    }
                    F32(data) => {
                        data_set.add_var_attr_f32(&var_name, attr_name, data.clone())?;
                    }
                    F64(data) => {
                        data_set.add_var_attr_f64(&var_name, attr_name, data.clone())?;
                    }
                }
            }
        }

        if !num_records_is_determinated {
            // Case an *unlimited-size* dim s defined
            if let Some(dim) = data_set.get_unlimited_dim() {
                let num_records: usize;
                // Case: the unlimited dim  is defined but no record variable is defined
                if record_var_begin_offsets.is_empty() {
                    num_records = 0;
                } else {
                    // Computation of the number of records
                    let first_begin_offset: usize = record_var_begin_offsets
                        .into_iter()
                        .map(|begin_offset: Offset| i64::from(begin_offset) as usize)
                        .min()
                        .unwrap();
                    let all_records_size: usize = total_file_size - first_begin_offset; // the size allocated for all record data
                    let record_size: usize = data_set.record_size().ok_or(ReadError::Unexpected)?;
                    if record_size == 0 {
                        // cannot be zero
                        return Err(ReadError::Unexpected);
                    }
                    num_records = all_records_size
                        .checked_div_euclid(record_size)
                        .ok_or(ReadError::Unexpected)?;
                    let num_rem_bytes: usize = all_records_size
                        .checked_rem_euclid(record_size)
                        .ok_or(ReadError::Unexpected)?; // the number of remaining bytes
                    if num_rem_bytes != 0 {
                        return Err(ReadError::ComputationNumberOfRecords);
                    }
                }
                if let DimensionSize::Unlimited(dim_size) = &dim.size {
                    dim_size.replace(num_records);
                }
                if let DimensionSize::Unlimited(dim_size) = &dim.size {
                    dim_size.replace(num_records);
                }
            }
        }
        Ok((data_set, version, var_info_list))
    }

    fn parse_magic_word(input: &[u8]) -> Result<(&[u8], &[u8]), ParseHeaderError> {
        let (input, tag_value): (&[u8], &[u8]) = tag(&b"CDF"[..])(input)
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::MagicWord))?;
        Ok((input, tag_value))
    }

    fn parse_version(input: &[u8]) -> Result<(&[u8], Version), ParseHeaderError> {
        let (input, version_number): (&[u8], u8) = verify(be_u8, |ver_num: &u8| {
            ver_num == &(Version::Classic as u8) || ver_num == &(Version::Offset64Bit as u8)
        })
        .parse(input)
        .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::VersionNumber))?;
        let version = Version::try_from(version_number).unwrap(); // previously checked
        Ok((input, version))
    }

    /// Parses a `i32` word and checks that it is non-negative.
    fn parse_non_neg_i32(input: &[u8]) -> Result<(&[u8], i32), ParseHeaderError> {
        verify(be_i32, |number: &i32| *number >= 0_i32)
            .parse(input)
            .map_err(|err: NomError| {
                ParseHeaderError::new(err, ParseHeaderErrorKind::NonNegativeI32)
            })
    }

    /// Parses a non-negative `i32` word and converts it to a `usize`.
    fn parse_as_usize(input: &[u8]) -> Result<(&[u8], usize), ParseHeaderError> {
        let (input, number): (&[u8], i32) = FileReader::parse_non_neg_i32(input)?;
        Ok((input, number as usize))
    }

    /// Parses the number of records
    ///
    /// Returns :
    /// - The numbers of records if it is a valid integer.
    /// - `None` if the number of records is indeterminated
    fn parse_as_usize_optional(input: &[u8]) -> Result<(&[u8], Option<usize>), ParseHeaderError> {
        const INDETERMINATE_VALUE: u32 = u32::MAX;
        let (input, value): (&[u8], u32) = verify(be_u32, |number: &u32| {
            *number <= (i32::MAX as u32) || *number == INDETERMINATE_VALUE
        })
        .parse(input)
        .map_err(|err: NomError| {
            ParseHeaderError::new(err, ParseHeaderErrorKind::NonNegativeI32)
        })?;
        let value: Option<usize> = match value {
            INDETERMINATE_VALUE => None,
            _ => Some(value as usize),
        };
        Ok((input, value))
    }

    /// Parses a non-negative `i32` word and converts it to a `u32`.
    fn parse_as_u32(input: &[u8]) -> Result<(&[u8], u32), ParseHeaderError> {
        let (input, number): (&[u8], i32) = FileReader::parse_non_neg_i32(input)?;
        Ok((input, number as u32))
    }
    /// Parses a string
    fn parse_name_string(input: &[u8]) -> Result<(&[u8], String), ParseHeaderError> {
        let (input, num_of_bytes): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let (input, name): (&[u8], String) = map_res(take(num_of_bytes), |bytes: &[u8]| {
            String::from_utf8(bytes.to_vec())
        })
        .parse(input)
        .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::Utf8))?;
        // Take the zero padding bytes if necessary
        let (input, _zero_padding_bytes): (&[u8], &[u8]) =
            FileReader::parse_zero_padding(input, compute_padding_size(num_of_bytes))?;
        Ok((input, name))
    }

    // Parses a NetCDF-3 data type.
    fn parse_data_type(input: &[u8]) -> Result<(&[u8], DataType), ParseHeaderError> {
        let start: &[u8] = input;
        let (input, data_type_number): (&[u8], u32) = FileReader::parse_as_u32(input)?;
        let data_type: DataType = DataType::try_from(data_type_number)
            .map_err(|_err| nom::Err::Error((&start[0..4], nom::error::ErrorKind::Verify)))
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::DataType))?;
        Ok((input, data_type))
    }

    fn parse_typed_data_elements(
        input: &[u8],
        num_of_elements: usize,
        data_type: DataType,
    ) -> Result<(&[u8], DataVector), ParseHeaderError> {
        // Parsed the useful data
        let (input, data_vector): (&[u8], DataVector) = match data_type {
            DataType::I8 => many_m_n(num_of_elements, num_of_elements, be_i8)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<i8>)| (input, DataVector::I8(data))),
            DataType::U8 => many_m_n(num_of_elements, num_of_elements, be_u8)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<u8>)| (input, DataVector::U8(data))),
            DataType::I16 => many_m_n(num_of_elements, num_of_elements, be_i16)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<i16>)| (input, DataVector::I16(data))),
            DataType::I32 => many_m_n(num_of_elements, num_of_elements, be_i32)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<i32>)| (input, DataVector::I32(data))),
            DataType::F32 => many_m_n(num_of_elements, num_of_elements, be_f32)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<f32>)| (input, DataVector::F32(data))),
            DataType::F64 => many_m_n(num_of_elements, num_of_elements, be_f64)
                .parse(input)
                .map(|(input, data): (&[u8], Vec<f64>)| (input, DataVector::F64(data))),
        }
        .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::DataElements))?;

        // Parse the zero padding bytes if necessary
        let num_of_bytes: usize = data_type.size_of() * num_of_elements;
        let (input, _zero_padding_bytes): (&[u8], &[u8]) =
            FileReader::parse_zero_padding(input, compute_padding_size(num_of_bytes))?;
        Ok((input, data_vector))
    }

    fn parse_zero_padding(
        input: &[u8],
        num_bytes: usize,
    ) -> Result<(&[u8], &[u8]), ParseHeaderError> {
        verify(take(num_bytes), |padding_bytes: &[u8]| {
            padding_bytes.iter().all(|byte: &u8| *byte == 0_u8)
        })
        .parse(input)
        .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::ZeroPadding))
    }

    // Parses the list of the dimensions from the header.
    #[allow(clippy::type_complexity)]
    fn parse_dims_list(input: &[u8]) -> Result<(&[u8], Vec<(String, usize)>), ParseHeaderError> {
        fn parse_dim(input: &[u8]) -> Result<(&[u8], (String, usize)), ParseHeaderError> {
            let (input, dim_name): (&[u8], String) = FileReader::parse_name_string(input)?;
            let (input, dim_size): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            Ok((input, (dim_name, dim_size)))
        }
        let (input, dim_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(DIMENSION_TAG)))
            .parse(input)
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::DimTag))?;
        if dim_tag == ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_dims): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut dims_list: Vec<(String, usize)> = Vec::with_capacity(num_of_dims);
        for _ in 0..num_of_dims {
            let (rem_input, dim): (&[u8], (String, usize)) = parse_dim(input)?;
            input = rem_input;
            dims_list.push(dim);
        }

        Ok((input, dims_list))
    }

    // Parses a list of attributes (global of from any variables) from the header.
    #[allow(clippy::type_complexity)]
    fn parse_attrs_list(
        input: &[u8],
    ) -> Result<(&[u8], Vec<(String, DataVector)>), ParseHeaderError> {
        fn parse_attr(input: &[u8]) -> Result<(&[u8], (String, DataVector)), ParseHeaderError> {
            let (input, attr_name): (&[u8], String) = FileReader::parse_name_string(input)?;
            let (input, attr_data_type): (&[u8], DataType) = FileReader::parse_data_type(input)?;
            let (input, num_of_elements): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            let (input, attr_data): (&[u8], DataVector) =
                FileReader::parse_typed_data_elements(input, num_of_elements, attr_data_type)?;
            Ok((input, (attr_name, attr_data)))
        }
        let (input, attr_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(ATTRIBUTE_TAG)))
            .parse(input)
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::AttrTag))?;
        if attr_tag == ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_attrs): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut attrs_list: Vec<(String, DataVector)> = Vec::with_capacity(num_of_attrs);
        for _ in 0..num_of_attrs {
            let (rem_input, attr): (&[u8], (String, DataVector)) = parse_attr(input)?;
            input = rem_input;
            attrs_list.push(attr);
        }
        Ok((input, attrs_list))
    }

    // Parses a list of variables from the header.
    fn parse_vars_list(
        input: &[u8],
        version: Version,
    ) -> Result<(&[u8], Vec<VariableParsedMetadata>), ParseHeaderError> {
        fn parse_dim_ids_list(input: &[u8]) -> Result<(&[u8], Vec<usize>), ParseHeaderError> {
            // number of dimensions
            let (mut input, num_of_dims): (&[u8], usize) = FileReader::parse_as_usize(input)?;
            // list of the dimension ids
            let mut dim_ids_list: Vec<usize> = Vec::with_capacity(num_of_dims);
            for _ in 0..num_of_dims {
                let (rem_input, dim_id): (&[u8], usize) = FileReader::parse_as_usize(input)?;
                input = rem_input;
                dim_ids_list.push(dim_id);
            }
            Ok((input, dim_ids_list))
        }

        fn parse_offset(
            input: &[u8],
            version: Version,
        ) -> Result<(&[u8], Offset), ParseHeaderError> {
            match version {
                Version::Classic => be_i32(input)
                    .map(|(input, num_of_bytes): (&[u8], i32)| (input, Offset::I32(num_of_bytes))),
                Version::Offset64Bit => be_i64(input)
                    .map(|(input, num_of_bytes): (&[u8], i64)| (input, Offset::I64(num_of_bytes))),
            }
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::Offset))
        }

        fn parse_var(
            input: &[u8],
            version: Version,
        ) -> Result<(&[u8], VariableParsedMetadata), ParseHeaderError> {
            // Variable name
            let (input, var_name): (&[u8], String) = FileReader::parse_name_string(input)?;

            // list of the dimensions
            let (input, dim_ids): (&[u8], Vec<usize>) = parse_dim_ids_list(input)?;
            // list of the variable attributes
            let (input, attrs_list): (&[u8], Vec<(String, DataVector)>) =
                FileReader::parse_attrs_list(input)?;
            // data type of the variable
            let (input, data_type): (&[u8], DataType) = FileReader::parse_data_type(input)?;
            // size occupied in each record by the variable (number of bytes)
            let (input, chunk_size): (&[u8], Option<usize>) =
                FileReader::parse_as_usize_optional(input)?;
            // begin offset (number of bytes)
            let (input, begin_offset): (&[u8], Offset) = parse_offset(input, version)?;
            let var_def = VariableParsedMetadata {
                name: var_name,
                dim_ids,
                attrs_list,
                data_type,
                _chunk_size: chunk_size,
                begin_offset,
            };
            Ok((input, var_def))
        }
        let (input, var_tag): (&[u8], &[u8]) = alt((tag(ABSENT_TAG), tag(VARIABLE_TAG)))
            .parse(input)
            .map_err(|err: NomError| ParseHeaderError::new(err, ParseHeaderErrorKind::VarTag))?;
        if var_tag == ABSENT_TAG {
            return Ok((input, vec![]));
        }
        let (mut input, num_of_vars): (&[u8], usize) = FileReader::parse_as_usize(input)?;
        let mut vars_list: Vec<VariableParsedMetadata> = vec![];
        for _ in 0..num_of_vars {
            let (temp_input, var) = parse_var(input, version.clone())?;
            input = temp_input;
            vars_list.push(var);
        }
        Ok((input, vars_list))
    }

    fn find_var_info(&self, var_name: &str) -> Option<&VariableParsedMetadata> {
        self.vars_info
            .iter()
            .find(|var_info| var_info.name == var_name)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct VariableParsedMetadata {
    name: String,
    dim_ids: Vec<usize>,
    attrs_list: Vec<(String, DataVector)>,
    data_type: DataType,
    _chunk_size: Option<usize>,
    begin_offset: Offset,
}
