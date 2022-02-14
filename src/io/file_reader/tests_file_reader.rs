#![cfg(test)]
use std::num::NonZeroUsize;
use byteorder::{WriteBytesExt, BigEndian};

use crate::{
    FileReader, Variable, DataSet, Attribute, Dimension, DataType, DimensionType, Version,
    error::ReadError,
    error::parse_header_error::{ParseHeaderError, ParseHeaderErrorKind, InvalidBytes},
    io::compute_padding_size,
};

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
    SCALAR_VARIABLES_FILE_NAME, SCALAR_VARIABLES_FILE_BYTES,
    EMPTY_DATA_SET_FILE_NAME, EMPTY_DATA_SET_FILE_BYTES,
    NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME, NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_BYTES,
};

const TEMP_I8_VAR_NAME: &str = "temperature_i8";
const TEMP_I8_VAR_DATA: [i8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
const TEMP_I8_VAR_LEN: usize = TEMP_I8_VAR_DATA.len();

const TEMP_U8_VAR_NAME: &str = "temperature_u8";
const TEMP_U8_VAR_DATA: [u8; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
const TEMP_U8_VAR_LEN: usize = TEMP_U8_VAR_DATA.len();

const TEMP_I16_VAR_NAME: &str = "temperature_i16";
const TEMP_I16_VAR_DATA: [i16; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
const TEMP_I16_VAR_LEN: usize = TEMP_I16_VAR_DATA.len();

const TEMP_I32_VAR_NAME: &str = "temperature_i32";
const TEMP_I32_VAR_DATA: [i32; 30] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
const TEMP_I32_VAR_LEN: usize = TEMP_I32_VAR_DATA.len();

const TEMP_F32_VAR_NAME: &str = "temperature_f32";
const TEMP_F32_VAR_DATA: [f32; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];
const TEMP_F32_VAR_LEN: usize = TEMP_F32_VAR_DATA.len();

const TEMP_F64_VAR_NAME: &str = "temperature_f64";
const TEMP_F64_VAR_DATA: [f64; 30] = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29.];
const TEMP_F64_VAR_LEN: usize = TEMP_F64_VAR_DATA.len();

#[test]
fn test_read_var_i8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I8_VAR_NAME));
        assert_eq!(Some(DataType::I8),              data_set.var_data_type(TEMP_I8_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I8_VAR_DATA.to_vec()), file_reader.read_var_i8(TEMP_I8_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I8},
        file_reader.read_var_i8(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I8},
        file_reader.read_var_i8(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I8},
        file_reader.read_var_i8(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I8},
        file_reader.read_var_i8(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I8},
        file_reader.read_var_i8(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_u8("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();

    assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
    assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
}

#[test]
fn test_read_var_u8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
        assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_U8_VAR_DATA.to_vec()), file_reader.read_var_u8(TEMP_U8_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::U8},
        file_reader.read_var_u8(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::U8},
        file_reader.read_var_u8(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::U8},
        file_reader.read_var_u8(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::U8},
        file_reader.read_var_u8(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::U8},
        file_reader.read_var_u8(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_u8("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();

    assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
    assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
}

#[test]
fn test_read_var_i16() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I16_VAR_NAME));
        assert_eq!(Some(DataType::I16),             data_set.var_data_type(TEMP_I16_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I16_VAR_DATA.to_vec()),      file_reader.read_var_i16(TEMP_I16_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I16},
        file_reader.read_var_i16(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I16},
        file_reader.read_var_i16(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I16},
        file_reader.read_var_i16(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I16},
        file_reader.read_var_i16(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I16},
        file_reader.read_var_i16(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_i16("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_I16_VAR_NAME));
    assert_eq!(Some(DataType::I16),              data_set.var_data_type(TEMP_I16_VAR_NAME));
}

#[test]
fn test_read_var_i32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I32_VAR_NAME));
        assert_eq!(Some(DataType::I32),             data_set.var_data_type(TEMP_I32_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_I32_VAR_DATA.to_vec()),      file_reader.read_var_i32(TEMP_I32_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I32},
        file_reader.read_var_i32(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I32},
        file_reader.read_var_i32(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I32},
        file_reader.read_var_i32(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I32},
        file_reader.read_var_i32(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I32},
        file_reader.read_var_i32(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_i32("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_I32_VAR_NAME));
    assert_eq!(Some(DataType::I32),             data_set.var_data_type(TEMP_I32_VAR_NAME));
}

#[test]
fn test_read_var_f32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F32_VAR_NAME));
        assert_eq!(Some(DataType::F32),             data_set.var_data_type(TEMP_F32_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_F32_VAR_DATA.to_vec()),      file_reader.read_var_f32(TEMP_F32_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F32},
        file_reader.read_var_f32(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F32},
        file_reader.read_var_f32(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F32},
        file_reader.read_var_f32(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F32},
        file_reader.read_var_f32(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::F32},
        file_reader.read_var_f32(TEMP_F64_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_f32("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_F32_VAR_NAME));
    assert_eq!(Some(DataType::F32),             data_set.var_data_type(TEMP_F32_VAR_NAME));
}

#[test]
fn test_read_var_f64() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F64_VAR_NAME));
        assert_eq!(Some(DataType::F64),             data_set.var_data_type(TEMP_F64_VAR_NAME));
    }

    assert_eq!(Ok(TEMP_F64_VAR_DATA.to_vec()),      file_reader.read_var_f64(TEMP_F64_VAR_NAME));

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F64},
        file_reader.read_var_f64(TEMP_I8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F64},
        file_reader.read_var_f64(TEMP_U8_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F64},
        file_reader.read_var_f64(TEMP_I16_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F64},
        file_reader.read_var_f64(TEMP_I32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::F64},
        file_reader.read_var_f64(TEMP_F32_VAR_NAME).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_var_f64("undef_var").unwrap_err()
    );

    let data_set: DataSet = file_reader.close().0;
    tmp_dir.close().unwrap();
    assert_eq!(true,                            data_set.has_var(TEMP_F64_VAR_NAME));
    assert_eq!(Some(DataType::F64),             data_set.var_data_type(TEMP_F64_VAR_NAME));
}

#[test]
fn test_parse_header() {
    use std::rc::Rc;
    use super::VariableParsedMetadata;

    const LATITUDE_DIM_NAME: &str = "latitude";
    const LATITUDE_VAR_NAME: &str = LATITUDE_DIM_NAME;
    const LATITUDE_VAR_LEN: usize = 3;

    const LONGITUDE_DIM_NAME: &str = "longitude";
    const LONGITUDE_VAR_NAME: &str = LONGITUDE_DIM_NAME;
    const LONGITUDE_VAR_LEN: usize = 5;

    const TIME_DIM_NAME: &str = "time";
    const TIME_VAR_NAME: &str = TIME_DIM_NAME;
    const TIME_VAR_LEN: usize = 2;

    let num_of_bytes: usize = NC3_CLASSIC_FILE_BYTES.len();
    let parsing_result: Result<(DataSet, Version, Vec<VariableParsedMetadata>), ReadError>;
    parsing_result = FileReader::parse_header(NC3_CLASSIC_FILE_BYTES, num_of_bytes);
    assert_eq!(true,                        parsing_result.is_ok());
    let (data_set, version, _vars_info) = parsing_result.unwrap();

    // Check the version
    assert_eq!(Version::Classic,         version);

    // Check the global-attributes
    {
        assert_eq!(2,                        data_set.num_global_attrs());

        let global_attrs: Vec<&Attribute> = data_set.get_global_attrs();
        assert_eq!(2,                        global_attrs.len());

        assert_eq!("title",                                                 global_attrs[0].name());
        assert_eq!(DataType::U8,                                            global_attrs[0].data_type());
        assert_eq!(Some(String::from("Example of NETCDF3_CLASSIC file")),   global_attrs[0].get_as_string());

        assert_eq!("Conventions",                                           global_attrs[1].name());
        assert_eq!(DataType::U8,                                            global_attrs[1].data_type());
        assert_eq!(Some(String::from("CF-1.8")),                            global_attrs[1].get_as_string());
    }

    // Check the dimensions
    {
        assert_eq!(3,                           data_set.num_dims());

        let dims: Vec<Rc<Dimension>> = data_set.get_dims();
        assert_eq!(3,                           dims.len());

        assert_eq!(LATITUDE_DIM_NAME,           dims[0].name());
        assert_eq!(LATITUDE_VAR_LEN,            dims[0].size());
        assert_eq!(DimensionType::FixedSize,    dims[0].dim_type());

        assert_eq!(LONGITUDE_DIM_NAME,          dims[1].name());
        assert_eq!(LONGITUDE_VAR_LEN,           dims[1].size());
        assert_eq!(DimensionType::FixedSize,    dims[1].dim_type());

        assert_eq!(TIME_DIM_NAME,                   dims[2].name());
        assert_eq!(TIME_VAR_LEN,                    dims[2].size());
        assert_eq!(DimensionType::UnlimitedSize,    dims[2].dim_type());
    }

    // Check the variables
    {
        assert_eq!(9,                        data_set.num_vars());

        let vars: Vec<&Variable> = data_set.get_vars();
        assert_eq!(9,                       vars.len());

        assert_eq!(LATITUDE_DIM_NAME,       vars[0].name());
        assert_eq!(DataType::F32,           vars[0].data_type());

        // latitude
        {
            assert_eq!(true,                                                    data_set.has_var(LATITUDE_VAR_NAME));
            let var: &Variable = data_set.get_var(LATITUDE_VAR_NAME).unwrap();

            assert_eq!(DataType::F32,                                                   var.data_type());
            assert_eq!(false,                                                           var.is_record_var());
            assert_eq!(vec![LATITUDE_DIM_NAME],                                         var.dim_names());
            assert_eq!(1,                                                               var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN,                                                var.chunk_len());
            assert_eq!(LATITUDE_VAR_LEN,                                                var.len());

            // Check the variable attributes
            assert_eq!(4,                                                               var.num_attrs());
            assert_eq!(Some(String::from("latitude")),                                  var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("LATITUDE")),                                  var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("degrees_north")),                             var.get_attr_as_string("units"));
            assert_eq!(Some(String::from("Y")),                                         var.get_attr_as_string("axis"));
        }
        // longitude
        {
            assert_eq!(true,                                                            data_set.has_var(LONGITUDE_VAR_NAME));
            let var: &Variable = data_set.get_var(LONGITUDE_VAR_NAME).unwrap();

            assert_eq!(DataType::F32,                                                   var.data_type());
            assert_eq!(false,                                                           var.is_record_var());
            assert_eq!(vec![LONGITUDE_DIM_NAME],                                        var.dim_names());
            assert_eq!(1,                                                               var.num_chunks());
            assert_eq!(LONGITUDE_VAR_LEN,                                               var.chunk_len());
            assert_eq!(LONGITUDE_VAR_LEN,                                               var.len());

            // Check the variable attributes
            assert_eq!(4,                                                               var.num_attrs());
            assert_eq!(Some(String::from("longitude")),                                 var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("LONGITUDE")),                                 var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("degrees_east")),                              var.get_attr_as_string("units"));
            assert_eq!(Some(String::from("X")),                                         var.get_attr_as_string("axis"));
        }
        // time
        {
            assert_eq!(true,                                                            data_set.has_var(TIME_VAR_NAME));
            let var: &Variable = data_set.get_var(TIME_VAR_NAME).unwrap();

            assert_eq!(DataType::F32,                                                   var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME],                                             var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(1,                                                               var.chunk_len());
            assert_eq!(TIME_VAR_LEN,                                                    var.len());

            // Check the variable attributes
            assert_eq!(5,                                                               var.num_attrs());
            assert_eq!(Some(String::from("time")),                                      var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TIME")),                                      var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("hours since 1970-01-01 00:00:00")),           var.get_attr_as_string("units"));
            assert_eq!(Some(String::from("T")),                                         var.get_attr_as_string("axis"));
            assert_eq!(Some(String::from("gregorian")),                                 var.get_attr_as_string("calendar"));
        }
        // temperature_i8
        {
            assert_eq!(true,                                                            data_set.has_var(TEMP_I8_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_I8_VAR_NAME).unwrap();

            assert_eq!(DataType::I8,                                                    var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_I8_VAR_LEN,                                                 var.len());
            assert_eq!(TEMP_I8_VAR_LEN,                                                 TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));

        }
        // temperature_u8
        {
            assert_eq!(true,                                                            data_set.has_var(TEMP_U8_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_U8_VAR_NAME).unwrap();

            assert_eq!(DataType::U8,                                                    var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_U8_VAR_LEN,                                                 var.len());
            assert_eq!(TEMP_U8_VAR_LEN,                                                 TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));
        }
        // temperature_i16
        {
            assert_eq!(true,                                                            data_set.has_var(TEMP_I16_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_I16_VAR_NAME).unwrap();

            assert_eq!(DataType::I16,                                                   var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_I16_VAR_LEN,                                                var.len());
            assert_eq!(TEMP_I16_VAR_LEN,                                                TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));
        }
        // temperature_i32
        {
            assert_eq!(true,                                                            data_set.has_var(TEMP_I32_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_I32_VAR_NAME).unwrap();

            assert_eq!(DataType::I32,                                                   var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_I32_VAR_LEN,                                                var.len());
            assert_eq!(TEMP_I32_VAR_LEN,                                                TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));
        }
        // temperature_f32
        {
            assert_eq!(true,                                                            data_set.has_var(TEMP_F32_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_F32_VAR_NAME).unwrap();

            assert_eq!(DataType::F32,                                                   var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_F32_VAR_LEN,                                                var.len());
            assert_eq!(TEMP_F32_VAR_LEN,                                                TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));
        }
        // temperature_f64
        {
            assert_eq!(true,                                            data_set.has_var(TEMP_F64_VAR_NAME));
            let var: &Variable = data_set.get_var(TEMP_F64_VAR_NAME).unwrap();

            assert_eq!(DataType::F64,                                                   var.data_type());
            assert_eq!(true,                                                            var.is_record_var());
            assert_eq!(vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],      var.dim_names());
            assert_eq!(TIME_VAR_LEN,                                                    var.num_chunks());
            assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN,                            var.chunk_len());
            assert_eq!(TEMP_F64_VAR_LEN,                                                var.len());
            assert_eq!(TEMP_F64_VAR_LEN,                                                TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN);

            // Check the variable attributes
            assert_eq!(3,                                                               var.num_attrs());
            assert_eq!(Some(String::from("air_temperature")),                           var.get_attr_as_string("standard_name"));
            assert_eq!(Some(String::from("TEMPERATURE")),                               var.get_attr_as_string("long_name"));
            assert_eq!(Some(String::from("Celsius")),                                   var.get_attr_as_string("units"));
        }
    }
}

#[test]
fn test_parse_truncated_header()
{
    const HEADER_NUM_OF_BYTES: usize = 1_684;
    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &b""[..];
        let file_size: usize = truncated_file_bytes.len();
        // Open the NetCDF-3 file
        let parsing_res: Result<(DataSet, Version, Vec<_>), ReadError> = FileReader::parse_header(truncated_file_bytes, file_size);
        assert_eq!(true,                parsing_res.is_err());
        let parsing_err: ReadError = parsing_res.unwrap_err();
        assert_eq!(true,                parsing_err.header_is_incomplete());
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..1];
        let file_size: usize = truncated_file_bytes.len();
        // Open the NetCDF-3 file
        let parsing_res: Result<(DataSet, Version, Vec<_>), ReadError> = FileReader::parse_header(truncated_file_bytes, file_size);
        assert_eq!(true,                parsing_res.is_err());
        let parsing_err: ReadError = parsing_res.unwrap_err();
        assert_eq!(true,                parsing_err.header_is_incomplete());
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..(HEADER_NUM_OF_BYTES - 1)];
        let file_size: usize = truncated_file_bytes.len();
        // Open the NetCDF-3 file
        let parsing_res: Result<(DataSet, Version, Vec<_>), ReadError> = FileReader::parse_header(truncated_file_bytes, file_size);
        assert_eq!(true,                parsing_res.is_err());
        let parsing_err: ReadError = parsing_res.unwrap_err();
        assert_eq!(true,                parsing_err.header_is_incomplete());
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..(HEADER_NUM_OF_BYTES)];
        let file_size: usize = truncated_file_bytes.len();
        // Open the NetCDF-3 file
        let parsing_res: Result<(DataSet, Version, Vec<_>), ReadError> = FileReader::parse_header(truncated_file_bytes, file_size);
        assert_eq!(true,                parsing_res.is_ok());
    }
}

#[test]
fn test_parse_non_neg_i32() {
    // Test `0_i32`
    {
        let a: i32 = 0_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(0_i32, b);
    }

    // Test `1_i32`
    {
        let a: i32 = 1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(1_i32, b);
    }

    // Test `std::i32::MAX`
    {
        let a: i32 = std::i32::MAX;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8], rem_bytes);
        assert_eq!(std::i32::MAX, b);
    }

    // Test `-1_i32`
    {
        let a: i32 = -1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        // check the returned error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(false,  parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes,
        );
    }

    // Test `std::i32::MIN`
    {
        let a: i32 = std::i32::MIN;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        // check the returned error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(false,  parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes,
        );
    }

    // Test with a larger input
    {
        let a: i32 = 1_i32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // Add some bytes
        let mut bytes: Vec<u8> = Vec::from(&bytes[..]);
        bytes.push(42);
        bytes.push(43);
        bytes.push(44);
        // parse the integer
        let (rem_bytes, b): (&[u8], i32) = FileReader::parse_non_neg_i32(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[42, 43, 44], rem_bytes);
        assert_eq!(1_i32, b);
    }

    // Missing input bytes
    {
        let a: i32 = 1_i32;
        let bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..2]);
        assert_eq!(2, bytes.len());
        // check the returned error
        let parsing_result = FileReader::parse_non_neg_i32(&bytes[..]);
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32 ,parsing_err.kind);
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(NonZeroUsize::new(2).unwrap())),
            parsing_err.invalid_bytes
        );
    }
}

#[test]
fn test_parse_num_records() {
    // Test the indeterminated valud `std::u32::MAX`
    {
        let a: u32 = std::u32::MAX;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], Option<usize>) = FileReader::parse_as_usize_optional(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8],                    rem_bytes);
        assert_eq!(None,                            b);
    }

    // Test `0`
    {
        let a: u32 = 0_u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8],Option<usize>) = FileReader::parse_as_usize_optional(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8],                rem_bytes);
        assert_eq!(Some(0),                     b);
    }

    // Test `1_u32`
    {
        let a: u32 = 1_u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8],Option<usize>) = FileReader::parse_as_usize_optional(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8],                rem_bytes);
        assert_eq!(Some(1),                     b);
    }

    // Test `std::i32::MAX`
    {
        let a: u32 = std::i32::MAX as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let (rem_bytes, b): (&[u8], Option<usize>) = FileReader::parse_as_usize_optional(&bytes[..]).unwrap();
        // test remaining bytes and the parsed value
        assert_eq!(&[] as &[u8],                    rem_bytes);
        assert_eq!(Some(std::i32::MAX as usize),    b);
    }

    // Test `std::i32::MIN`
    {
        let a: i32 = std::i32::MIN;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_as_usize_optional(&bytes[..]);
        // check the returned error
        assert_eq!(true,                                        parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(false,                                   parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32,   parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes,
        );
    }

    // Test with a larger input
    {
        let a: u32 = (std::i32::MIN as u32) + 1;
        let bytes: [u8; 4] = a.to_be_bytes();
        // parse the integer
        let parsing_result = FileReader::parse_as_usize_optional(&bytes[..]);
        // check the returned error
        assert_eq!(true,                                        parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(false,                                       parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32,        parsing_err.kind);
        assert_eq!(InvalidBytes::Bytes(bytes.to_vec()),         parsing_err.invalid_bytes);
    }

    // Missing input bytes
    {
        let a: u32 = 0_u32;
        let bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[0..3]);
        // parse the integer
        // parse the integer
        let parsing_result = FileReader::parse_as_usize_optional(&bytes[..]);
        // check the returned error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(true,                                                                            parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32,                                            parsing_err.kind);
        assert_eq!(InvalidBytes::Incomplete(nom::Needed::Size(NonZeroUsize::new(1).unwrap())),      parsing_err.invalid_bytes);
    }
}

#[test]
fn test_parse_name_string() {
    {
        // Test a ASCII word
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }

                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("foo", name);
            // And test the remaining bytes
            assert_eq!(0, rem_bytes.len());
        }

        // Test a ASCII word extended by other bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
                // Append other bytes
                bytes.extend(&[1, 2, 3]);

                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("foo", name);
            // And test the remaining bytes
            assert_eq!(&[1, 2, 3], rem_bytes);
        }

        // Test with a wrong zero-padding bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foooo");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                assert!(zero_padding_size > 0);
                for i in 0..zero_padding_size
                {
                    if i == 0 {
                        // Append a wrong bytes here
                        bytes.push(1_u8);
                    }
                    else {
                        bytes.push(0_u8);
                    }
                }
                bytes
            };
            // check the returned error
            let parsing_result = FileReader::parse_name_string(&bytes[..]);
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert_eq!(false,                               parsing_err.header_is_incomplete());
            assert_eq!(ParseHeaderErrorKind::ZeroPadding,   parsing_err.kind);
            assert_eq!(InvalidBytes::Bytes(vec![1, 0, 0]),  parsing_err.invalid_bytes);
        }

        // Test a valid UTF-8 word
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("café");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }

                bytes
            };
            // Parse the bytes into a string
            let (rem_bytes, name): (&[u8], String)= FileReader::parse_name_string(&bytes).unwrap();
            // Test the parsed string
            assert_eq!("café", name);
            // And test the remaining bytes
            assert_eq!(0, rem_bytes.len());
        }


        // Test a latin-1 word (not valid UTF-8)
        {
            let bytes: Vec<u8> = {
                let word : Vec<u8> = vec![b'c', b'a', b'f', b'\xe9'];  // latin-1 encoding

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(&word);
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }

                bytes
            };
            // Parse the bytes into a string
            let parsing_result: Result<_, _> = FileReader::parse_name_string(&bytes);
            // Test the parsed string
            assert!(parsing_result.is_err());
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert_eq!(false,  parsing_err.header_is_incomplete());
            assert_eq!(ParseHeaderErrorKind::Utf8 ,parsing_err.kind);
            assert_eq!(
                InvalidBytes::Bytes(vec![b'c', b'a', b'f', b'\xe9']),
                parsing_err.invalid_bytes,
            );
        }

        // Test missing zero padding bytes
        {
            let bytes: Vec<u8> = {
                let word : String = String::from("foobar");

                // Write the name
                let mut bytes: Vec<u8> = vec![];
                let num_of_bytes = word.len();
                bytes.write_i32::<BigEndian>(num_of_bytes as i32).unwrap();
                bytes.extend(word.as_bytes());
                // Append zero-padding bytes if necessary
                let zero_padding_size: usize = compute_padding_size(num_of_bytes);
                for _ in 0..zero_padding_size
                {
                    bytes.push(0_u8);
                }
                // remove the last byte
                assert!(bytes.len() >= 2);
                bytes.remove(bytes.len() - 1);

                bytes
            };
            // Parse the bytes into a string
            let parsing_result: Result<_, _> = FileReader::parse_name_string(&bytes);
            // Test the parsed string
            assert!(parsing_result.is_err());
            let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
            assert!(parsing_err.header_is_incomplete());
            assert_eq!(ParseHeaderErrorKind::ZeroPadding ,parsing_err.kind);
            assert_eq!(
                InvalidBytes::Incomplete(nom::Needed::Size(NonZeroUsize::new(1).unwrap())),
                parsing_err.invalid_bytes,
            );
        }
    }
}

#[test]
fn test_parse_data_type() {
    use std::convert::TryFrom;

    // test parse `DataType::I8`
    {
        let a: u32 = DataType::I8 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I8, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse `DataType::U8`
    {
        let a: u32 = DataType::U8 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::U8, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::I16`
    {
        let a: u32 = DataType::I16 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I16, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::I32`
    {
        let a: u32 = DataType::I32 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::I32, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // // test parse `DataType::F32`
    {
        let a: u32 = DataType::F32 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F32, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse `DataType::F64`
    {
        let a: u32 = DataType::F64 as u32;
        let bytes: [u8; 4] = a.to_be_bytes();
        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F64, data_type);
        assert_eq!(&[] as &[u8], rem_input);
    }

    // test parse a non-existant `DataType`
    {
        let a: u32 = 0_u32;
        assert!(DataType::try_from(a).is_err());

        let bytes: [u8; 4] = a.to_be_bytes();
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        assert!(parsing_result.is_err());
    }

    // test parse a negative `DataType` number
    {
        let a: i32 = -1_i32;

        let bytes: [u8; 4] = a.to_be_bytes();
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert_eq!(false,  parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32, parsing_err.kind);
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes
        );
    }

    // check the remaining bytes
    {
        let a: u32 = DataType::F64 as u32;

        let mut bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..]);
        bytes.push(42);
        bytes.push(43);
        bytes.push(44);

        let (rem_input, data_type): (&[u8], DataType) = FileReader::parse_data_type(&bytes[..]).unwrap();
        assert_eq!(DataType::F64, data_type);
        assert_eq!(
            &[42, 43, 44],
            rem_input
        );
    }

    // test missing input bytes
    {
        let a: u32 = DataType::F64 as u32;
        let bytes: Vec<u8> = Vec::from(&a.to_be_bytes()[..3]);
        assert_eq!(3, bytes.len());
        let parsing_result = FileReader::parse_data_type(&bytes[..]);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err: ParseHeaderError = parsing_result.unwrap_err();
        assert!(parsing_err.header_is_incomplete());
        assert_eq!(ParseHeaderErrorKind::NonNegativeI32, parsing_err.kind);
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(NonZeroUsize::new(1).unwrap())),
            parsing_err.invalid_bytes
        );
    }
}

#[test]
fn test_parse_zero_padding() {
    // Test valid zero padding
    {
        let bytes: [u8; 3] = [0_u8; 3];
        let (rem_input, zero_padding): (&[u8], &[u8]) = FileReader::parse_zero_padding(&bytes, 3).unwrap();
        assert_eq!(0, rem_input.len());
        assert_eq!(&[0, 0, 0], zero_padding);

    }
    // Test not valid zero padding
    {
        let bytes: [u8; 3] = [0, 1, 0];
        let parsing_result = FileReader::parse_zero_padding(&bytes, 3);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err = parsing_result.unwrap_err();
        assert_eq!(false,  parsing_err.header_is_incomplete());
        assert_eq!(
            ParseHeaderErrorKind::ZeroPadding,
            parsing_err.kind,
        );
        assert_eq!(
            InvalidBytes::Bytes(bytes.to_vec()),
            parsing_err.invalid_bytes
        );
    }
    // Test missing bytes
    {
        let bytes: [u8; 3] = [0_u8; 3];
        let parsing_result = FileReader::parse_zero_padding(&bytes[0..2], 3);
        // Check the return error
        assert!(parsing_result.is_err());
        let parsing_err = parsing_result.unwrap_err();
        assert!(parsing_err.header_is_incomplete());
        assert_eq!(
            ParseHeaderErrorKind::ZeroPadding,
            parsing_err.kind,
        );
        assert_eq!(
            InvalidBytes::Incomplete(nom::Needed::Size(NonZeroUsize::new(1).unwrap())),
            parsing_err.invalid_bytes
        );
    }
}

#[test]
fn test_read_indeterminated_num_records() {
    // Test a NetCDF-3 file which has an unlimited-size
    // ------------------------------------------------
    {
        const UNLIM_DIM_NAME: &str = "time";
        const UNLIM_DIM_SIZE: usize = 2;

        // 1: Read a data set where in which the `num_records` is defined
        let original_dataset: DataSet = {
            let (tmp_dir, input_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 2. Change the input file bytes (set indeterminated the `num_records`) and read it as a NetCDF-3 data set
        let modified_dataset: DataSet = {
            let modified_bytes: Vec<u8> = {
                // the indeterminate value is (2^32 - 1), see the file format specifications (https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
                let indeterminated_value_as_bytes: [u8; 4] = std::u32::MAX.to_be_bytes();
                let mut bytes: Vec<u8> = NC3_CLASSIC_FILE_BYTES.to_vec();
                bytes[4..8].copy_from_slice(&indeterminated_value_as_bytes);
                bytes
            };
            let (tmp_dir, input_file_path) =  copy_bytes_to_tmp_file(&modified_bytes[..], NC3_CLASSIC_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 3. Compare the unlimited-size dimensions of the 2 datasets
        assert_eq!(true,                                                original_dataset.has_unlimited_dim());
        {
            let unlim_dim = original_dataset.get_unlimited_dim().unwrap();
            assert_eq!(UNLIM_DIM_NAME,                                  unlim_dim.name());
            assert_eq!(UNLIM_DIM_SIZE,                                  unlim_dim.size());
            assert_eq!(false,                                           unlim_dim.is_fixed());
            assert_eq!(true,                                            unlim_dim.is_unlimited());
            assert_eq!(DimensionType::UnlimitedSize,                    unlim_dim.dim_type());
        }

        assert_eq!(true,                                                modified_dataset.has_unlimited_dim());
        {
            let unlim_dim = modified_dataset.get_unlimited_dim().unwrap();
            assert_eq!(UNLIM_DIM_NAME,                                  unlim_dim.name());
            assert_eq!(UNLIM_DIM_SIZE,                                  unlim_dim.size());
            assert_eq!(false,                                           unlim_dim.is_fixed());
            assert_eq!(true,                                            unlim_dim.is_unlimited());
            assert_eq!(DimensionType::UnlimitedSize,                    unlim_dim.dim_type());
        }

        assert_eq!(original_dataset.record_size(),                      modified_dataset.record_size());
        assert_eq!(original_dataset.num_records(),                      modified_dataset.num_records());
    }

    // Test a NetCDF-3 file which has not a unlimited-size dim but in which the number of records is indeterminated
    {
        // 1: Read a data set where in which the `num_records` is defined
        let original_dataset: DataSet = {
            let (tmp_dir, input_file_path) = copy_bytes_to_tmp_file(SCALAR_VARIABLES_FILE_BYTES, SCALAR_VARIABLES_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 2. Change the input file bytes (set indeterminated the `num_records`) and read it as a NetCDF-3 data set
        let modified_dataset: DataSet = {
            let modified_bytes: Vec<u8> = {
                // the indeterminate value is (2^32 - 1), see the file format specifications (https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
                let indeterminated_value_as_bytes: [u8; 4] = std::u32::MAX.to_be_bytes();
                let mut bytes: Vec<u8> = SCALAR_VARIABLES_FILE_BYTES.to_vec();
                bytes[4..8].copy_from_slice(&indeterminated_value_as_bytes);
                bytes
            };
            let (tmp_dir, input_file_path) =  copy_bytes_to_tmp_file(&modified_bytes[..], SCALAR_VARIABLES_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 3. Compare the unlimited-size dimensions of the 2 datasets
        assert_eq!(false,                                               original_dataset.has_unlimited_dim());
        assert_eq!(false,                                               modified_dataset.has_unlimited_dim());

        assert_eq!(original_dataset.record_size(),                      modified_dataset.record_size());
        assert_eq!(original_dataset.num_records(),                      modified_dataset.num_records())
    }

    // Test an empty data set NetCDF-3 file which has not a unlimited-size dim but in which the number of records is indeterminated
    {
        // 1: Read a data set where in which the `num_records` is defined
        let original_dataset: DataSet = {
            let (tmp_dir, input_file_path) = copy_bytes_to_tmp_file(EMPTY_DATA_SET_FILE_BYTES, EMPTY_DATA_SET_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 2. Change the input file bytes (set indeterminated the `num_records`) and read it as a NetCDF-3 data set
        let modified_dataset: DataSet = {
            let modified_bytes: Vec<u8> = {
                // the indeterminate value is (2^32 - 1), see the file format specifications (https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
                let indeterminated_value_as_bytes: [u8; 4] = std::u32::MAX.to_be_bytes();
                let mut bytes: Vec<u8> = EMPTY_DATA_SET_FILE_BYTES.to_vec();
                bytes[4..8].copy_from_slice(&indeterminated_value_as_bytes);
                bytes
            };
            let (tmp_dir, input_file_path) =  copy_bytes_to_tmp_file(&modified_bytes[..], EMPTY_DATA_SET_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 3. Compare the unlimited-size dimensions of the 2 datasets
        assert_eq!(false,                                               original_dataset.has_unlimited_dim());
        assert_eq!(false,                                               modified_dataset.has_unlimited_dim());

        assert_eq!(original_dataset.record_size(),                      modified_dataset.record_size());
        assert_eq!(original_dataset.num_records(),                      modified_dataset.num_records())
    }

    // Test a NetCDF-3 file conatining a zero-sized unlimited dimension but in which the number of records is indeterminated
    {
        const UNLIM_DIM_NAME: &str = "unlim_dim";
        const UNLIM_DIM_SIZE: usize = 0;

        // 1: Read a data set where in which the `num_records` is defined
        let original_dataset: DataSet = {
            let (tmp_dir, input_file_path) = copy_bytes_to_tmp_file(NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_BYTES, NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 2. Change the input file bytes (set indeterminated the `num_records`) and read it as a NetCDF-3 data set
        let modified_dataset: DataSet = {
            let modified_bytes: Vec<u8> = {
                // the indeterminate value is (2^32 - 1), see the file format specifications (https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
                let indeterminated_value_as_bytes: [u8; 4] = std::u32::MAX.to_be_bytes();
                let mut bytes: Vec<u8> = NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_BYTES.to_vec();
                bytes[4..8].copy_from_slice(&indeterminated_value_as_bytes);
                bytes
            };
            let (tmp_dir, input_file_path) =  copy_bytes_to_tmp_file(&modified_bytes[..], NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME);
            let file_reader: FileReader = FileReader::open(input_file_path).unwrap();
            let data_set: DataSet = file_reader.close().0;
            tmp_dir.close().unwrap();
            data_set
        };

        // 3. Compare the unlimited-size dimensions of the 2 datasets
        assert_eq!(true,                                               original_dataset.has_unlimited_dim());
        {
            let unlim_dim = original_dataset.get_unlimited_dim().unwrap();
            assert_eq!(UNLIM_DIM_NAME,                                  unlim_dim.name());
            assert_eq!(UNLIM_DIM_SIZE,                                  unlim_dim.size());
            assert_eq!(false,                                           unlim_dim.is_fixed());
            assert_eq!(true,                                            unlim_dim.is_unlimited());
            assert_eq!(DimensionType::UnlimitedSize,                    unlim_dim.dim_type());
        }

        assert_eq!(true,                                                modified_dataset.has_unlimited_dim());
        {
            let unlim_dim = modified_dataset.get_unlimited_dim().unwrap();
            assert_eq!(UNLIM_DIM_NAME,                                  unlim_dim.name());
            assert_eq!(UNLIM_DIM_SIZE,                                  unlim_dim.size());
            assert_eq!(false,                                           unlim_dim.is_fixed());
            assert_eq!(true,                                            unlim_dim.is_unlimited());
            assert_eq!(DimensionType::UnlimitedSize,                    unlim_dim.dim_type());
        }

        assert_eq!(original_dataset.record_size(),                      modified_dataset.record_size());
        assert_eq!(original_dataset.num_records(),                      modified_dataset.num_records())
    }
}

#[test]
fn test_read_record_i8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I8_VAR_NAME));
        assert_eq!(Some(DataType::I8),              data_set.var_data_type(TEMP_I8_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I8_VAR_NAME).unwrap();
        assert_eq!(DataType::I8,                    var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_I8_VAR_DATA.len(),          var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_I8_VAR_DATA[0..15].to_vec()),    file_reader.read_record_i8(TEMP_I8_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_I8_VAR_DATA[15..30].to_vec()),   file_reader.read_record_i8(TEMP_I8_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_i8(TEMP_I8_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I8},
        file_reader.read_record_i8(TEMP_U8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I8},
        file_reader.read_record_i8(TEMP_I16_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I8},
        file_reader.read_record_i8(TEMP_I32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I8},
        file_reader.read_record_i8(TEMP_F32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I8},
        file_reader.read_record_i8(TEMP_F64_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_i8("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}

#[test]
fn test_read_record_u8() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_U8_VAR_NAME));
        assert_eq!(Some(DataType::U8),              data_set.var_data_type(TEMP_U8_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_U8_VAR_NAME).unwrap();
        assert_eq!(DataType::U8,                    var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_U8_VAR_DATA.len(),          var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_U8_VAR_DATA[0..15].to_vec()),    file_reader.read_record_u8(TEMP_U8_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_U8_VAR_DATA[15..30].to_vec()),   file_reader.read_record_u8(TEMP_U8_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_u8(TEMP_U8_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::U8},
        file_reader.read_record_u8(TEMP_I8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::U8},
        file_reader.read_record_u8(TEMP_I16_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::U8},
        file_reader.read_record_u8(TEMP_I32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::U8},
        file_reader.read_record_u8(TEMP_F32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::U8},
        file_reader.read_record_u8(TEMP_F64_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_u8("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}

#[test]
fn test_read_record_i16() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I16_VAR_NAME));
        assert_eq!(Some(DataType::I16),             data_set.var_data_type(TEMP_I16_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I16_VAR_NAME).unwrap();
        assert_eq!(DataType::I16,                   var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_I16_VAR_DATA.len(),         var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_I16_VAR_DATA[0..15].to_vec()),   file_reader.read_record_i16(TEMP_I16_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_I16_VAR_DATA[15..30].to_vec()),  file_reader.read_record_i16(TEMP_I16_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_i16(TEMP_I16_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I16},
        file_reader.read_record_i16(TEMP_I8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I16},
        file_reader.read_record_i16(TEMP_U8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::I16},
        file_reader.read_record_i16(TEMP_I32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I16},
        file_reader.read_record_i16(TEMP_F32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I16},
        file_reader.read_record_i16(TEMP_F64_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_i16("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}

#[test]
fn test_read_record_i32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_I32_VAR_NAME));
        assert_eq!(Some(DataType::I32),             data_set.var_data_type(TEMP_I32_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I32_VAR_NAME).unwrap();
        assert_eq!(DataType::I32,                   var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_I32_VAR_DATA.len(),         var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_I32_VAR_DATA[0..15].to_vec()),   file_reader.read_record_i32(TEMP_I32_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_I32_VAR_DATA[15..30].to_vec()),  file_reader.read_record_i32(TEMP_I32_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_i32(TEMP_I32_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::I32},
        file_reader.read_record_i32(TEMP_I8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::I32},
        file_reader.read_record_i32(TEMP_U8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::I32},
        file_reader.read_record_i32(TEMP_I16_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::I32},
        file_reader.read_record_i32(TEMP_F32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::I32},
        file_reader.read_record_i32(TEMP_F64_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_i32("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}

#[test]
fn test_read_record_f32() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F32_VAR_NAME));
        assert_eq!(Some(DataType::F32),             data_set.var_data_type(TEMP_F32_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_F32_VAR_NAME).unwrap();
        assert_eq!(DataType::F32,                   var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_F32_VAR_DATA.len(),         var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_F32_VAR_DATA[0..15].to_vec()),   file_reader.read_record_f32(TEMP_F32_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_F32_VAR_DATA[15..30].to_vec()),  file_reader.read_record_f32(TEMP_F32_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_f32(TEMP_F32_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F32},
        file_reader.read_record_f32(TEMP_I8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F32},
        file_reader.read_record_f32(TEMP_U8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F32},
        file_reader.read_record_f32(TEMP_I16_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F32},
        file_reader.read_record_f32(TEMP_I32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F64_VAR_NAME), req: DataType::F64, get: DataType::F32},
        file_reader.read_record_f32(TEMP_F64_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_f32("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}

#[test]
fn test_read_record_f64() {
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    {
        let data_set: &DataSet = file_reader.data_set();
        assert_eq!(true,                            data_set.has_var(TEMP_F64_VAR_NAME));
        assert_eq!(Some(DataType::F64),             data_set.var_data_type(TEMP_F64_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_F64_VAR_NAME).unwrap();
        assert_eq!(DataType::F64,                   var.data_type());
        assert_eq!(true,                            var.is_record_var());
        assert_eq!(2,                               var.num_chunks());
        assert_eq!(15,                              var.chunk_len());
        assert_eq!(30,                              var.len());
        assert_eq!(TEMP_F64_VAR_DATA.len(),         var.len());
    }
    let num_records = file_reader.data_set().num_records().unwrap();
    assert_eq!(2,                                       num_records);

    // Read the 1st record
    assert_eq!(Ok(TEMP_F64_VAR_DATA[0..15].to_vec()),   file_reader.read_record_f64(TEMP_F64_VAR_NAME, 0));
    // Read the 2nd records
    assert_eq!(Ok(TEMP_F64_VAR_DATA[15..30].to_vec()),  file_reader.read_record_f64(TEMP_F64_VAR_NAME, 1));
    assert_eq!(
        ReadError::RecordIndexExceeded{index: 2, num_records: num_records},
        file_reader.read_record_f64(TEMP_F64_VAR_NAME, num_records).unwrap_err(),
    );

    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I8_VAR_NAME), req: DataType::I8, get: DataType::F64},
        file_reader.read_record_f64(TEMP_I8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_U8_VAR_NAME), req: DataType::U8, get: DataType::F64},
        file_reader.read_record_f64(TEMP_U8_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I16_VAR_NAME), req: DataType::I16, get: DataType::F64},
        file_reader.read_record_f64(TEMP_I16_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_I32_VAR_NAME), req: DataType::I32, get: DataType::F64},
        file_reader.read_record_f64(TEMP_I32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableMismatchDataType{var_name: String::from(TEMP_F32_VAR_NAME), req: DataType::F32, get: DataType::F64},
        file_reader.read_record_f64(TEMP_F32_VAR_NAME, 0).unwrap_err()
    );
    assert_eq!(
        ReadError::VariableNotDefined(String::from("undef_var")),
        file_reader.read_record_f64("undef_var", 0).unwrap_err()
    );

    let _ = file_reader.close();
    tmp_dir.close().unwrap();
}
