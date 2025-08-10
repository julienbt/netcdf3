#![cfg(test)]
use std::io::Cursor;
use std::rc::Rc;

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file, EMPTY_DATA_SET_FILE_BYTES, EMPTY_DATA_SET_FILE_NAME,
    NC3_64BIT_OFFSET_FILE_BYTES, NC3_64BIT_OFFSET_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
    NC3_CLASSIC_FILE_NAME, NC3_FILL_VALUES_FILE_BYTES, NC3_FILL_VALUES_FILE_NAME,
    NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_BYTES, NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME,
    SCALAR_VARIABLES_FILE_BYTES, SCALAR_VARIABLES_FILE_NAME,
};

use netcdf3::NC_FILL_F32;
use netcdf3::NC_FILL_F64;
use netcdf3::NC_FILL_I16;
use netcdf3::NC_FILL_I32;
use netcdf3::NC_FILL_I8;
use netcdf3::NC_FILL_U8;
use netcdf3::{
    error::ReadError, DataSet, DataType, Dimension, DimensionType, FileReader, Variable, Version,
};

const LATITUDE_DIM_NAME: &str = "latitude";
const LATITUDE_VAR_NAME: &str = LATITUDE_DIM_NAME;
const LATITUDE_VAR_DATA: [f32; 3] = [0.0, 0.5, 1.0];
const LATITUDE_VAR_LEN: usize = LATITUDE_VAR_DATA.len();

const LONGITUDE_DIM_NAME: &str = "longitude";
const LONGITUDE_VAR_NAME: &str = LONGITUDE_DIM_NAME;
const LONGITUDE_VAR_DATA: [f32; 5] = [0.0, 0.5, 1.0, 1.5, 2.0];
const LONGITUDE_VAR_LEN: usize = LONGITUDE_VAR_DATA.len();

const TIME_DIM_NAME: &str = "time";
const TIME_VAR_NAME: &str = TIME_DIM_NAME;
const TIME_VAR_DATA: [f32; 2] = [438_300.0, 438_324.0];
const TIME_VAR_LEN: usize = TIME_VAR_DATA.len();

const TEMP_I8_VAR_NAME: &str = "temperature_i8";
const TEMP_I8_VAR_DATA: [i8; 30] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29,
];
const TEMP_I8_VAR_LEN: usize = TEMP_I8_VAR_DATA.len();

const TEMP_U8_VAR_NAME: &str = "temperature_u8";
const TEMP_U8_VAR_DATA: [u8; 30] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29,
];
const TEMP_U8_VAR_LEN: usize = TEMP_U8_VAR_DATA.len();

const TEMP_I16_VAR_NAME: &str = "temperature_i16";
const TEMP_I16_VAR_DATA: [i16; 30] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29,
];
const TEMP_I16_VAR_LEN: usize = TEMP_I16_VAR_DATA.len();

const TEMP_I32_VAR_NAME: &str = "temperature_i32";
const TEMP_I32_VAR_DATA: [i32; 30] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29,
];
const TEMP_I32_VAR_LEN: usize = TEMP_I32_VAR_DATA.len();

const TEMP_F32_VAR_NAME: &str = "temperature_f32";
const TEMP_F32_VAR_DATA: [f32; 30] = [
    0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    21., 22., 23., 24., 25., 26., 27., 28., 29.,
];
const TEMP_F32_VAR_LEN: usize = TEMP_F32_VAR_DATA.len();

const TEMP_F64_VAR_NAME: &str = "temperature_f64";
const TEMP_F64_VAR_DATA: [f64; 30] = [
    0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    21., 22., 23., 24., 25., 26., 27., 28., 29.,
];
const TEMP_F64_VAR_LEN: usize = TEMP_F64_VAR_DATA.len();


fn check_nc3_classic_data(mut file_reader: &mut FileReader) {
    // Check the NetCDF-3 definition
    // -----------------------------
    assert_eq!(Version::Classic, file_reader.version());
    // Check the global attributes
    let data_set: &DataSet = file_reader.data_set();
    assert_eq!(2, data_set.num_global_attrs());
    assert_eq!(
        Some(String::from("Example of NETCDF3_CLASSIC file")),
        data_set.get_global_attr_as_string("title")
    );
    assert_eq!(
        Some(String::from("CF-1.8")),
        data_set.get_global_attr_as_string("Conventions")
    );
    check_temperatures_definition(file_reader.data_set());

    // Check the NetCDF-3 data
    // -----------------------
    check_temperatures_data(&mut file_reader);
}

#[test]
fn test_read_file_nc3_classic() {
    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) =
        copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    check_nc3_classic_data(&mut file_reader);

    tmp_dir.close().unwrap();
}

#[test]
/// Test reading a file through an input that implement Seek and Read traits.
fn test_read_file_nc3_classic_with_seek_read() {

    // Create a cursor (implements Seek and Read traits) for reading they NetCDF-3 bytes.
    // ------------------------------
    let cursor = Cursor::new(NC3_CLASSIC_FILE_BYTES);
    let boxed_cursor = Box::new(cursor);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open_seek_read(NC3_CLASSIC_FILE_NAME, boxed_cursor).unwrap();

    check_nc3_classic_data(&mut file_reader);
}

#[test]
fn test_read_file_nc3_classic_open_with_truncated_file() {
    const HEADER_NUM_OF_BYTES: usize = 1_684;

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &b""[..];
        let (tmp_dir, input_data_file_path) =
            copy_bytes_to_tmp_file(truncated_file_bytes, NC3_CLASSIC_FILE_NAME);
        // Open the NetCDF-3 file
        let reading_res: Result<FileReader, ReadError> = FileReader::open(input_data_file_path);
        assert_eq!(true, reading_res.is_err());
        let reading_err: ReadError = reading_res.unwrap_err();
        assert_eq!(true, reading_err.header_is_incomplete());
        // Check the parsing result
        tmp_dir.close().unwrap();
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..1];
        let (tmp_dir, input_data_file_path) =
            copy_bytes_to_tmp_file(truncated_file_bytes, NC3_CLASSIC_FILE_NAME);
        // Open the NetCDF-3 file
        let reading_res: Result<FileReader, ReadError> = FileReader::open(input_data_file_path);
        // Check the parsing result
        assert_eq!(true, reading_res.is_err());
        let reading_err: ReadError = reading_res.unwrap_err();
        assert_eq!(true, reading_err.header_is_incomplete());
        tmp_dir.close().unwrap();
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..(HEADER_NUM_OF_BYTES - 1)];
        let (tmp_dir, input_data_file_path) =
            copy_bytes_to_tmp_file(truncated_file_bytes, NC3_CLASSIC_FILE_NAME);
        // Open the NetCDF-3 file
        let reading_res: Result<FileReader, ReadError> = FileReader::open(input_data_file_path);
        // Check the parsing result
        assert_eq!(true, reading_res.is_err());
        let reading_err: ReadError = reading_res.unwrap_err();
        assert_eq!(true, reading_err.header_is_incomplete());
        tmp_dir.close().unwrap();
    }

    {
        // Copy truncated bytes to a temporary file
        let truncated_file_bytes: &[u8] = &NC3_CLASSIC_FILE_BYTES[..(HEADER_NUM_OF_BYTES)];
        let (tmp_dir, input_data_file_path) =
            copy_bytes_to_tmp_file(truncated_file_bytes, NC3_CLASSIC_FILE_NAME);
        // Open the NetCDF-3 file
        let reading_res: Result<FileReader, ReadError> = FileReader::open(input_data_file_path);
        // Check the parsing result
        assert_eq!(true, reading_res.is_ok());
        tmp_dir.close().unwrap();
    }
}

#[test]
fn test_read_file_nc3_64bit_offset() {
    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) =
        copy_bytes_to_tmp_file(NC3_64BIT_OFFSET_FILE_BYTES, NC3_64BIT_OFFSET_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    // Check the NetCDF-3 definition
    // -----------------------------
    assert_eq!(Version::Offset64Bit, file_reader.version());
    // Check the global attributes
    let data_set: &DataSet = file_reader.data_set();
    assert_eq!(2, data_set.num_global_attrs());
    assert_eq!(
        Some(String::from("Example of NETCDF3_64BIT_OFFSET file")),
        data_set.get_global_attr_as_string("title")
    );
    assert_eq!(
        Some(String::from("CF-1.8")),
        data_set.get_global_attr_as_string("Conventions")
    );
    check_temperatures_definition(file_reader.data_set());

    // Check the NetCDF-3 data
    // -----------------------
    check_temperatures_data(&mut file_reader);

    tmp_dir.close().unwrap();
}

fn check_temperatures_definition(data_set: &DataSet) {
    // Check the parsing of the header
    assert!(data_set.has_unlimited_dim());
    // Check the dimension definitions
    // -------------------------------
    assert_eq!(3, data_set.num_dims());
    assert_eq!(true, data_set.has_unlimited_dim());
    // `latitude`
    {
        assert_eq!(true, data_set.has_dim(LATITUDE_DIM_NAME));
        assert_eq!(Some(LATITUDE_VAR_LEN), data_set.dim_size(LATITUDE_DIM_NAME));
        assert_eq!(
            Some(DimensionType::FixedSize),
            data_set.dim_type(LATITUDE_DIM_NAME)
        );
    }
    // `longitude`
    {
        assert_eq!(true, data_set.has_dim(LONGITUDE_VAR_NAME));
        assert_eq!(
            Some(LONGITUDE_VAR_LEN),
            data_set.dim_size(LONGITUDE_VAR_NAME)
        );
        assert_eq!(
            Some(DimensionType::FixedSize),
            data_set.dim_type(LONGITUDE_VAR_NAME)
        );
    }
    // `time`
    {
        assert_eq!(true, data_set.has_dim(TIME_VAR_NAME));
        assert_eq!(Some(TIME_VAR_LEN), data_set.dim_size(TIME_VAR_NAME));
        assert_eq!(
            Some(DimensionType::UnlimitedSize),
            data_set.dim_type(TIME_VAR_NAME)
        );
    }

    // Check the variable definitions
    // ------------------------------
    assert_eq!(9, data_set.num_vars());
    // latitude
    {
        assert_eq!(true, data_set.has_var(LATITUDE_VAR_NAME));
        let var: &Variable = data_set.get_var(LATITUDE_VAR_NAME).unwrap();

        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![LATITUDE_DIM_NAME], var.dim_names());
        assert_eq!(1, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(LATITUDE_VAR_LEN, var.len());

        // Check the variable attributes
        assert_eq!(4, var.num_attrs());
        assert_eq!(
            Some(String::from("latitude")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("LATITUDE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("degrees_north")),
            var.get_attr_as_string("units")
        );
        assert_eq!(Some(String::from("Y")), var.get_attr_as_string("axis"));
    }
    // longitude
    {
        assert_eq!(true, data_set.has_var(LONGITUDE_VAR_NAME));
        let var: &Variable = data_set.get_var(LONGITUDE_VAR_NAME).unwrap();

        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![LONGITUDE_DIM_NAME], var.dim_names());
        assert_eq!(1, var.num_chunks());
        assert_eq!(LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(LONGITUDE_VAR_LEN, var.len());

        // Check the variable attributes
        assert_eq!(4, var.num_attrs());
        assert_eq!(
            Some(String::from("longitude")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("LONGITUDE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("degrees_east")),
            var.get_attr_as_string("units")
        );
        assert_eq!(Some(String::from("X")), var.get_attr_as_string("axis"));
    }
    // time
    {
        assert_eq!(true, data_set.has_var(TIME_VAR_NAME));
        let var: &Variable = data_set.get_var(TIME_VAR_NAME).unwrap();

        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(vec![TIME_DIM_NAME], var.dim_names());
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(1, var.chunk_len());
        assert_eq!(TIME_VAR_LEN, var.len());

        // Check the variable attributes
        assert_eq!(5, var.num_attrs());
        assert_eq!(
            Some(String::from("time")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TIME")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("hours since 1970-01-01 00:00:00")),
            var.get_attr_as_string("units")
        );
        assert_eq!(Some(String::from("T")), var.get_attr_as_string("axis"));
        assert_eq!(
            Some(String::from("gregorian")),
            var.get_attr_as_string("calendar")
        );
    }
    // temperature_i8
    {
        assert_eq!(true, data_set.has_var(TEMP_I8_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I8_VAR_NAME).unwrap();

        assert_eq!(DataType::I8, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_I8_VAR_LEN, var.len());
        assert_eq!(
            TEMP_I8_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
    // temperature_u8
    {
        assert_eq!(true, data_set.has_var(TEMP_U8_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_U8_VAR_NAME).unwrap();

        assert_eq!(DataType::U8, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_U8_VAR_LEN, var.len());
        assert_eq!(
            TEMP_U8_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
    // temperature_i16
    {
        assert_eq!(true, data_set.has_var(TEMP_I16_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I16_VAR_NAME).unwrap();

        assert_eq!(DataType::I16, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_I16_VAR_LEN, var.len());
        assert_eq!(
            TEMP_I16_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
    // temperature_i32
    {
        assert_eq!(true, data_set.has_var(TEMP_I32_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_I32_VAR_NAME).unwrap();

        assert_eq!(DataType::I32, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_I32_VAR_LEN, var.len());
        assert_eq!(
            TEMP_I32_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
    // temperature_f32
    {
        assert_eq!(true, data_set.has_var(TEMP_F32_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_F32_VAR_NAME).unwrap();

        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_F32_VAR_LEN, var.len());
        assert_eq!(
            TEMP_F32_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
    // temperature_f64
    {
        assert_eq!(true, data_set.has_var(TEMP_F64_VAR_NAME));
        let var: &Variable = data_set.get_var(TEMP_F64_VAR_NAME).unwrap();

        assert_eq!(DataType::F64, var.data_type());
        assert_eq!(true, var.is_record_var());
        assert_eq!(
            vec![TIME_DIM_NAME, LATITUDE_DIM_NAME, LONGITUDE_DIM_NAME],
            var.dim_names()
        );
        assert_eq!(TIME_VAR_LEN, var.num_chunks());
        assert_eq!(LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN, var.chunk_len());
        assert_eq!(TEMP_F64_VAR_LEN, var.len());
        assert_eq!(
            TEMP_F64_VAR_LEN,
            TIME_VAR_LEN * LATITUDE_VAR_LEN * LONGITUDE_VAR_LEN
        );

        // Check the variable attributes
        assert_eq!(3, var.num_attrs());
        assert_eq!(
            Some(String::from("air_temperature")),
            var.get_attr_as_string("standard_name")
        );
        assert_eq!(
            Some(String::from("TEMPERATURE")),
            var.get_attr_as_string("long_name")
        );
        assert_eq!(
            Some(String::from("Celsius")),
            var.get_attr_as_string("units")
        );
    }
}

fn check_temperatures_data(file_reader: &mut FileReader) {
    assert_eq!(9, file_reader.data_set().num_vars());

    // `latitude` data
    assert_eq!(true, file_reader.data_set().has_var(LATITUDE_VAR_NAME));
    assert_eq!(
        Some(DataType::F32),
        file_reader.data_set().var_data_type(LATITUDE_VAR_NAME)
    );
    assert_eq!(
        Some(LATITUDE_VAR_LEN),
        file_reader.data_set().var_len(LATITUDE_VAR_NAME)
    );
    assert_eq!(
        Ok(LATITUDE_VAR_DATA.to_vec()),
        file_reader.read_var_f32(LATITUDE_VAR_NAME)
    );

    // `longitude` data
    assert_eq!(true, file_reader.data_set().has_var(LONGITUDE_VAR_NAME));
    assert_eq!(
        Some(DataType::F32),
        file_reader.data_set().var_data_type(LONGITUDE_VAR_NAME)
    );
    assert_eq!(
        Some(LONGITUDE_VAR_LEN),
        file_reader.data_set().var_len(LONGITUDE_VAR_NAME)
    );
    assert_eq!(
        Ok(LONGITUDE_VAR_DATA.to_vec()),
        file_reader.read_var_f32(LONGITUDE_VAR_NAME)
    );

    // `time` data
    assert_eq!(true, file_reader.data_set().has_var(TIME_VAR_NAME));
    assert_eq!(
        Some(DataType::F32),
        file_reader.data_set().var_data_type(TIME_VAR_NAME)
    );
    assert_eq!(
        Some(TIME_VAR_LEN),
        file_reader.data_set().var_len(TIME_VAR_NAME)
    );
    assert_eq!(
        Ok(TIME_VAR_DATA.to_vec()),
        file_reader.read_var_f32(TIME_VAR_NAME)
    );

    // `temperature_i8` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_I8_VAR_NAME));
    assert_eq!(
        Some(DataType::I8),
        file_reader.data_set().var_data_type(TEMP_I8_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_I8_VAR_LEN),
        file_reader.data_set().var_len(TEMP_I8_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_I8_VAR_DATA.to_vec()),
        file_reader.read_var_i8(TEMP_I8_VAR_NAME)
    );

    // `temperature_u8` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_U8_VAR_NAME));
    assert_eq!(
        Some(DataType::U8),
        file_reader.data_set().var_data_type(TEMP_U8_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_U8_VAR_LEN),
        file_reader.data_set().var_len(TEMP_U8_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_U8_VAR_DATA.to_vec()),
        file_reader.read_var_u8(TEMP_U8_VAR_NAME)
    );

    // `temperature_i16` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_I16_VAR_NAME));
    assert_eq!(
        Some(DataType::I16),
        file_reader.data_set().var_data_type(TEMP_I16_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_I16_VAR_LEN),
        file_reader.data_set().var_len(TEMP_I16_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_I16_VAR_DATA.to_vec()),
        file_reader.read_var_i16(TEMP_I16_VAR_NAME)
    );

    // `temperature_i32` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_I32_VAR_NAME));
    assert_eq!(
        Some(DataType::I32),
        file_reader.data_set().var_data_type(TEMP_I32_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_I32_VAR_LEN),
        file_reader.data_set().var_len(TEMP_I32_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_I32_VAR_DATA.to_vec()),
        file_reader.read_var_i32(TEMP_I32_VAR_NAME)
    );

    // `temperature_f32` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_F32_VAR_NAME));
    assert_eq!(
        Some(DataType::F32),
        file_reader.data_set().var_data_type(TEMP_F32_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_F32_VAR_LEN),
        file_reader.data_set().var_len(TEMP_F32_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_F32_VAR_DATA.to_vec()),
        file_reader.read_var_f32(TEMP_F32_VAR_NAME)
    );

    // `temperature_f64` data
    assert_eq!(true, file_reader.data_set().has_var(TEMP_F64_VAR_NAME));
    assert_eq!(
        Some(DataType::F64),
        file_reader.data_set().var_data_type(TEMP_F64_VAR_NAME)
    );
    assert_eq!(
        Some(TEMP_F64_VAR_LEN),
        file_reader.data_set().var_len(TEMP_F64_VAR_NAME)
    );
    assert_eq!(
        Ok(TEMP_F64_VAR_DATA.to_vec()),
        file_reader.read_var_f64(TEMP_F64_VAR_NAME)
    );
}

#[test]
fn test_read_file_empty_data_set() {
    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) =
        copy_bytes_to_tmp_file(EMPTY_DATA_SET_FILE_BYTES, EMPTY_DATA_SET_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let file_reader = FileReader::open(input_data_file_path).unwrap();

    // Check the NetCDF-3 definition
    // -----------------------------
    assert_eq!(Version::Classic, file_reader.version());
    let data_set: &DataSet = file_reader.data_set();
    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(0, data_set.num_dims());
    assert_eq!(0, data_set.num_vars());

    tmp_dir.close().unwrap();
}

#[test]
fn test_read_file_scalar_vars() {
    const SCALAR_VAR_I8_NAME: &str = "scalar_value_i8";
    const SCALAR_VAR_U8_NAME: &str = "scalar_value_u8";
    const SCALAR_VAR_I16_NAME: &str = "scalar_value_i16";
    const SCALAR_VAR_I32_NAME: &str = "scalar_value_i32";
    const SCALAR_VAR_F32_NAME: &str = "scalar_value_f32";
    const SCALAR_VAR_F64_NAME: &str = "scalar_value_f64";

    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) =
        copy_bytes_to_tmp_file(SCALAR_VARIABLES_FILE_BYTES, SCALAR_VARIABLES_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    // Check the NetCDF-3 definition
    // -----------------------------
    assert_eq!(Version::Classic, file_reader.version());
    let data_set: &DataSet = file_reader.data_set();
    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(0, data_set.num_dims());
    assert_eq!(6, data_set.num_vars());

    // `scalar_value_i8`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_I8_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_I8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I8, var.data_type());
        assert_eq!(1, var.len());
    }

    // `scalar_value_i8`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_I8_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_I8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I8, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_u8`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_U8_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_U8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::U8, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_i16`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_I16_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_I16_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I16, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_i32`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_I32_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_I32_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I32, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_f32`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_F32_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_F32_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_f64`
    assert_eq!(true, data_set.has_var(SCALAR_VAR_F64_NAME));
    {
        let var: &Variable = data_set.get_var(SCALAR_VAR_F64_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::F64, var.data_type());
        assert_eq!(1, var.len());
    }

    // Check the NetCDF-3 data
    // -----------------------
    // `scalar_value_i8`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_I8_NAME));
    {
        assert_eq!(
            Some(DataType::I8),
            file_reader.data_set().var_data_type(SCALAR_VAR_I8_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_I8_NAME));
        assert_eq!(Ok(vec![42]), file_reader.read_var_i8(SCALAR_VAR_I8_NAME));
    }
    // `scalar_value_u8`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_U8_NAME));
    {
        assert_eq!(
            Some(DataType::U8),
            file_reader.data_set().var_data_type(SCALAR_VAR_U8_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_U8_NAME));
        assert_eq!(Ok(vec![42]), file_reader.read_var_u8(SCALAR_VAR_U8_NAME));
    }
    // `scalar_value_i16`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_I16_NAME));
    {
        assert_eq!(
            Some(DataType::I16),
            file_reader.data_set().var_data_type(SCALAR_VAR_I16_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_I16_NAME));
        assert_eq!(Ok(vec![42]), file_reader.read_var_i16(SCALAR_VAR_I16_NAME));
    }
    // `scalar_value_i32`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_I32_NAME));
    {
        assert_eq!(
            Some(DataType::I32),
            file_reader.data_set().var_data_type(SCALAR_VAR_I32_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_I32_NAME));
        assert_eq!(Ok(vec![42]), file_reader.read_var_i32(SCALAR_VAR_I32_NAME));
    }
    // `scalar_value_f32`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_F32_NAME));
    {
        assert_eq!(
            Some(DataType::F32),
            file_reader.data_set().var_data_type(SCALAR_VAR_F32_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_F32_NAME));
        assert_eq!(
            Ok(vec![42.0]),
            file_reader.read_var_f32(SCALAR_VAR_F32_NAME)
        );
    }
    // `scalar_value_f64`
    assert_eq!(true, file_reader.data_set().has_var(SCALAR_VAR_F64_NAME));
    {
        assert_eq!(
            Some(DataType::F64),
            file_reader.data_set().var_data_type(SCALAR_VAR_F64_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(SCALAR_VAR_F64_NAME));
        assert_eq!(
            Ok(vec![42.0]),
            file_reader.read_var_f64(SCALAR_VAR_F64_NAME)
        );
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_read_file_nc_fill_values() {
    const VAR_I8_NAME: &str = "nc_fill_value_i8";
    const VAR_U8_NAME: &str = "nc_fill_value_u8";
    const VAR_I16_NAME: &str = "nc_fill_value_i16";
    const VAR_I32_NAME: &str = "nc_fill_value_i32";
    const VAR_F32_NAME: &str = "nc_fill_value_f32";
    const VAR_F64_NAME: &str = "nc_fill_value_f64";
    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) =
        copy_bytes_to_tmp_file(NC3_FILL_VALUES_FILE_BYTES, NC3_FILL_VALUES_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open(input_data_file_path).unwrap();

    // Check the NetCDF-3 definition
    // -----------------------------
    assert_eq!(Version::Classic, file_reader.version());
    let data_set: &DataSet = file_reader.data_set();
    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(0, data_set.num_dims());
    assert_eq!(6, data_set.num_vars());

    // `scalar_value_i8`
    assert_eq!(true, data_set.has_var(VAR_I8_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_I8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I8, var.data_type());
        assert_eq!(1, var.len());
    }

    // `scalar_value_i8`
    assert_eq!(true, data_set.has_var(VAR_I8_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_I8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I8, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_u8`
    assert_eq!(true, data_set.has_var(VAR_U8_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_U8_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::U8, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_i16`
    assert_eq!(true, data_set.has_var(VAR_I16_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_I16_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I16, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_i32`
    assert_eq!(true, data_set.has_var(VAR_I32_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_I32_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::I32, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_f32`
    assert_eq!(true, data_set.has_var(VAR_F32_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_F32_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::F32, var.data_type());
        assert_eq!(1, var.len());
    }
    // `scalar_value_f64`
    assert_eq!(true, data_set.has_var(VAR_F64_NAME));
    {
        let var: &Variable = data_set.get_var(VAR_F64_NAME).unwrap();
        assert_eq!(false, var.is_record_var());
        assert_eq!(vec![] as Vec::<String>, var.dim_names());
        assert_eq!(DataType::F64, var.data_type());
        assert_eq!(1, var.len());
    }

    // Check the NetCDF-3 data
    // -----------------------
    // `scalar_value_i8`
    assert_eq!(true, file_reader.data_set().has_var(VAR_I8_NAME));
    {
        assert_eq!(
            Some(DataType::I8),
            file_reader.data_set().var_data_type(VAR_I8_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_I8_NAME));
        assert_eq!(Ok(vec![NC_FILL_I8]), file_reader.read_var_i8(VAR_I8_NAME));
    }
    // `scalar_value_u8`
    assert_eq!(true, file_reader.data_set().has_var(VAR_U8_NAME));
    {
        assert_eq!(
            Some(DataType::U8),
            file_reader.data_set().var_data_type(VAR_U8_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_U8_NAME));
        assert_eq!(Ok(vec![NC_FILL_U8]), file_reader.read_var_u8(VAR_U8_NAME));
    }
    // `scalar_value_i16`
    assert_eq!(true, file_reader.data_set().has_var(VAR_I16_NAME));
    {
        assert_eq!(
            Some(DataType::I16),
            file_reader.data_set().var_data_type(VAR_I16_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_I16_NAME));
        assert_eq!(
            Ok(vec![NC_FILL_I16]),
            file_reader.read_var_i16(VAR_I16_NAME)
        );
    }
    // `scalar_value_i32`
    assert_eq!(true, file_reader.data_set().has_var(VAR_I32_NAME));
    {
        assert_eq!(
            Some(DataType::I32),
            file_reader.data_set().var_data_type(VAR_I32_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_I32_NAME));
        assert_eq!(
            Ok(vec![NC_FILL_I32]),
            file_reader.read_var_i32(VAR_I32_NAME)
        );
    }
    // `scalar_value_f32`
    assert_eq!(true, file_reader.data_set().has_var(VAR_F32_NAME));
    {
        assert_eq!(
            Some(DataType::F32),
            file_reader.data_set().var_data_type(VAR_F32_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_F32_NAME));
        assert_eq!(
            Ok(vec![NC_FILL_F32]),
            file_reader.read_var_f32(VAR_F32_NAME)
        );
    }
    // `scalar_value_f64`
    assert_eq!(true, file_reader.data_set().has_var(VAR_F64_NAME));
    {
        assert_eq!(
            Some(DataType::F64),
            file_reader.data_set().var_data_type(VAR_F64_NAME)
        );
        assert_eq!(Some(1), file_reader.data_set().var_len(VAR_F64_NAME));
        assert_eq!(
            Ok(vec![NC_FILL_F64]),
            file_reader.read_var_f64(VAR_F64_NAME)
        );
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_read_file_zero_sized_unlimited_dim() {
    const UNLIM_DIM_NAME: &str = "unlim_dim";
    const UNLIM_DIM_SIZE: usize = 0;

    // Copy bytes to a temporary file
    // ------------------------------
    let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(
        NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_BYTES,
        NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME,
    );

    // Open the NetCDF-3 file
    // ----------------------
    let file_reader = FileReader::open(input_data_file_path).unwrap();
    // Check the NetCDF-3 definition
    // -----------------------------
    let (data_set, version): (DataSet, Version) = file_reader.close();

    assert_eq!(Version::Classic, version);
    assert_eq!(0, data_set.num_global_attrs());
    assert_eq!(1, data_set.num_dims());
    assert_eq!(0, data_set.num_vars());

    // Check the zero-sized unlimited dimension
    assert_eq!(true, data_set.has_unlimited_dim());
    let unlim_dim: Rc<Dimension> = data_set.get_unlimited_dim().unwrap();
    assert_eq!(UNLIM_DIM_NAME, unlim_dim.name());
    assert_eq!(UNLIM_DIM_SIZE, unlim_dim.size());
    assert_eq!(false, unlim_dim.is_fixed());
    assert_eq!(true, unlim_dim.is_unlimited());
    assert_eq!(DimensionType::UnlimitedSize, unlim_dim.dim_type());

    tmp_dir.close().unwrap()
}
