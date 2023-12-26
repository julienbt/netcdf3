#![cfg(test)]
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::rc::Rc;

use tempdir::TempDir;

use byteorder::{BigEndian, ReadBytesExt};

use crate::error::WriteError;
use crate::DataType;
use crate::Dimension;
use crate::FileReader;
use crate::Variable;
use crate::NC_FILL_F32;
use crate::NC_FILL_F64;
use crate::NC_FILL_I16;
use crate::NC_FILL_I32;
use crate::NC_FILL_I8;
use crate::NC_FILL_U8;

use super::{DataSet, FileWriter, Version, ABSENT_TAG, DIMENSION_TAG};

const TMP_DIR_PREFIX: &str = "netcdf3_tests_";

#[test]
fn test_open() {
    const TEST_FILE_NAME: &str = "test_open.nc";
    const GLOBAL_ATTR_NAME_1: &str = "comment_1";
    const GLOBAL_ATTR_NAME_2: &str = "comment_2";

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file
    let mut file_writer_1: FileWriter = FileWriter::open(&test_file_path).unwrap();
    let mut data_set_1 = DataSet::new();
    data_set_1.add_global_attr_string(GLOBAL_ATTR_NAME_1, "test_file_1").unwrap();
    file_writer_1.set_def(&data_set_1, Version::Classic, 0).unwrap();
    file_writer_1.close().unwrap();
    assert_eq!(true, test_file_path.exists());

    // Reopen the same NetCDF-3 file and set an other global attributes
    let mut file_writer_2: FileWriter = FileWriter::open(&test_file_path).unwrap();
    let mut data_set_2 = DataSet::new();
    data_set_2.add_global_attr_string(GLOBAL_ATTR_NAME_2, "test_file_2").unwrap();
    file_writer_2.set_def(&data_set_2, Version::Classic, 0).unwrap();
    file_writer_2.close().unwrap();
    assert_eq!(true, test_file_path.exists());

    // Read the outlet file
    let file_reader: FileReader = FileReader::open(&test_file_path).unwrap();
    let data_set_3: DataSet = file_reader.close().0;
    assert_eq!(1, data_set_3.num_global_attrs());
    assert_eq!(false, data_set_3.has_global_attr(GLOBAL_ATTR_NAME_1));
    assert_eq!(true, data_set_3.has_global_attr(GLOBAL_ATTR_NAME_2));
    assert_eq!("test_file_2", data_set_3.get_global_attr_as_string(GLOBAL_ATTR_NAME_2).unwrap());

    tmp_dir.close().unwrap();
}

#[test]
fn test_create_new() {
    const TEST_FILE_NAME: &str = "test_create_new.nc";
    const GLOBAL_ATTR_NAME: &str = "comment_1";

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file
    let mut file_writer_1: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
    let mut data_set_1 = DataSet::new();
    data_set_1.add_global_attr_string(GLOBAL_ATTR_NAME, "test_file_1").unwrap();
    file_writer_1.set_def(&data_set_1, Version::Classic, 0).unwrap(); // set an empty data set
    file_writer_1.close().unwrap();
    assert_eq!(true, test_file_path.exists());

    // Try to recreate the already existing file
    assert_eq!(
        WriteError::IOErrorKind(std::io::ErrorKind::AlreadyExists),
        FileWriter::create_new(&test_file_path).unwrap_err(),
    );
    assert_eq!(true, test_file_path.exists());

    // The first file has not been overwritten
    let file_reader: FileReader = FileReader::open(&test_file_path).unwrap();
    let data_set_2: DataSet = file_reader.close().0;
    assert_eq!(1, data_set_2.num_global_attrs());
    assert_eq!(true, data_set_2.has_global_attr(GLOBAL_ATTR_NAME));
    assert_eq!("test_file_1", data_set_2.get_global_attr_as_string(GLOBAL_ATTR_NAME).unwrap());

    tmp_dir.close().unwrap();
}

#[test]
fn test_fill_missing_data_at_closing() {
    const TEST_FILE_NAME: &str = "test_fill_missing_data_at_closing.nc";

    const VAR_I8_NAME: &str = "var_i8";
    const VAR_U8_NAME: &str = "var_u8";
    const VAR_I16_NAME: &str = "var_i16";
    const VAR_I32_NAME: &str = "var_i32";
    const VAR_F32_NAME: &str = "var_f32";
    const VAR_F64_NAME: &str = "var_f64";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();

        let data_i8: Vec<i8> = file_reader.read_var_i8(VAR_I8_NAME).unwrap();
        let data_u8: Vec<u8> = file_reader.read_var_u8(VAR_U8_NAME).unwrap();
        let data_i16: Vec<i16> = file_reader.read_var_i16(VAR_I16_NAME).unwrap();
        let data_i32: Vec<i32> = file_reader.read_var_i32(VAR_I32_NAME).unwrap();
        let data_f32: Vec<f32> = file_reader.read_var_f32(VAR_F32_NAME).unwrap();
        let data_f64: Vec<f64> = file_reader.read_var_f64(VAR_F64_NAME).unwrap();

        assert_eq!(vec![NC_FILL_I8; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_i8);
        assert_eq!(vec![NC_FILL_U8; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_u8);
        assert_eq!(vec![NC_FILL_I16; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_i16);
        assert_eq!(vec![NC_FILL_I32; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_i32);
        assert_eq!(vec![NC_FILL_F32; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_f32);
        assert_eq!(vec![NC_FILL_F64; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], data_f64);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i8() {
    const TEST_FILE_NAME: &str = "test_write_record_i8.nc";

    const VAR_I8_NAME: &str = "var_i8";
    const RECORD_1_I8_DATA: [i8; FIXED_DIM_SIZE] = [1, 2, 3, 4];
    const RECORD_3_I8_DATA: [i8; FIXED_DIM_SIZE] = [5, 6, 7, 8];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // Write record #1
        file_writer.write_record_i8(VAR_I8_NAME, 1, &RECORD_1_I8_DATA).unwrap();
        // Write record #3
        file_writer.write_record_i8(VAR_I8_NAME, 3, &RECORD_3_I8_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I8_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I8_NAME).unwrap();
            assert_eq!(DataType::I8, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i8> = file_reader.read_var_i8(VAR_I8_NAME).unwrap();
        file_reader.close();

        let record_0: &[i8] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[i8] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[i8] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[i8] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[i8] = &var_data[4 * chunk_len..5 * chunk_len];
        assert_eq!(vec![NC_FILL_I8; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_I8_DATA, record_1);
        assert_eq!(vec![NC_FILL_I8; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_I8_DATA, record_3);
        assert_eq!(vec![NC_FILL_I8; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i8_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i8_errors.nc";
    const VAR_I8_NAME: &str = "var_i8";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_I8_DATA: [i8; FIXED_DIM_SIZE] = [1, 2, 3, 4];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_i8(UNDEF_VAR_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I8
            },
            file_writer.write_record_i8(VAR_U8_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::I8
            },
            file_writer.write_record_i8(VAR_I16_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::I8
            },
            file_writer.write_record_i8(VAR_I32_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I8
            },
            file_writer.write_record_i8(VAR_F32_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I8
            },
            file_writer.write_record_i8(VAR_F64_NAME, 1, &RECORD_I8_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_i8(VAR_I8_NAME, UNLIM_DIM_SIZE, &RECORD_I8_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_I8_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_i8(VAR_I8_NAME, 1, &RECORD_I8_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I8_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I8_NAME).unwrap();
            assert_eq!(DataType::I8, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i8> = file_reader.read_var_i8(VAR_I8_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I8; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_u8() {
    const TEST_FILE_NAME: &str = "test_write_record_u8.nc";

    const VAR_U8_NAME: &str = "var_u8";
    const RECORD_1_U8_DATA: [u8; FIXED_DIM_SIZE] = [1, 2, 3, 4];
    const RECORD_3_U8_DATA: [u8; FIXED_DIM_SIZE] = [5, 6, 7, 8];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // Write record #1
        file_writer.write_record_u8(VAR_U8_NAME, 1, &RECORD_1_U8_DATA).unwrap();
        // Write record #3
        file_writer.write_record_u8(VAR_U8_NAME, 3, &RECORD_3_U8_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_U8_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_U8_NAME).unwrap();
            assert_eq!(DataType::U8, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<u8> = file_reader.read_var_u8(VAR_U8_NAME).unwrap();
        file_reader.close();

        let record_0: &[u8] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[u8] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[u8] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[u8] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[u8] = &var_data[4 * chunk_len..5 * chunk_len];

        assert_eq!(vec![NC_FILL_U8; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_U8_DATA, record_1);
        assert_eq!(vec![NC_FILL_U8; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_U8_DATA, record_3);
        assert_eq!(vec![NC_FILL_U8; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_u8_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_u8_errors.nc";
    const VAR_U8_NAME: &str = "var_u8";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_U8_DATA: [u8; FIXED_DIM_SIZE] = [1, 2, 3, 4];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_u8(UNDEF_VAR_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::U8
            },
            file_writer.write_record_u8(VAR_I8_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::U8
            },
            file_writer.write_record_u8(VAR_I16_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::U8
            },
            file_writer.write_record_u8(VAR_I32_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::U8
            },
            file_writer.write_record_u8(VAR_F32_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::U8
            },
            file_writer.write_record_u8(VAR_F64_NAME, 1, &RECORD_U8_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_u8(VAR_U8_NAME, UNLIM_DIM_SIZE, &RECORD_U8_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_U8_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_u8(VAR_U8_NAME, 1, &RECORD_U8_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_U8_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_U8_NAME).unwrap();
            assert_eq!(DataType::U8, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<u8> = file_reader.read_var_u8(VAR_U8_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_U8; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i16() {
    const TEST_FILE_NAME: &str = "test_write_record_i16.nc";

    const VAR_I16_NAME: &str = "var_i16";
    const RECORD_1_I16_DATA: [i16; FIXED_DIM_SIZE] = [1, 2, 3, 4];
    const RECORD_3_I16_DATA: [i16; FIXED_DIM_SIZE] = [5, 6, 7, 8];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // Write the record #1
        file_writer.write_record_i16(VAR_I16_NAME, 1, &RECORD_1_I16_DATA).unwrap();
        // Write the record #3
        file_writer.write_record_i16(VAR_I16_NAME, 3, &RECORD_3_I16_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I16_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I16_NAME).unwrap();
            assert_eq!(DataType::I16, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i16> = file_reader.read_var_i16(VAR_I16_NAME).unwrap();
        file_reader.close();

        let record_0: &[i16] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[i16] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[i16] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[i16] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[i16] = &var_data[4 * chunk_len..5 * chunk_len];

        assert_eq!(vec![NC_FILL_I16; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_I16_DATA, record_1);
        assert_eq!(vec![NC_FILL_I16; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_I16_DATA, record_3);
        assert_eq!(vec![NC_FILL_I16; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i16_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i16_errors.nc";
    const VAR_I16_NAME: &str = "var_i16";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_I16_DATA: [i16; FIXED_DIM_SIZE] = [1, 2, 3, 4];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_i16(UNDEF_VAR_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::I16
            },
            file_writer.write_record_i16(VAR_I8_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I16
            },
            file_writer.write_record_i16(VAR_U8_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::I16
            },
            file_writer.write_record_i16(VAR_I32_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I16
            },
            file_writer.write_record_i16(VAR_F32_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I16
            },
            file_writer.write_record_i16(VAR_F64_NAME, 1, &RECORD_I16_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_i16(VAR_I16_NAME, UNLIM_DIM_SIZE, &RECORD_I16_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_I16_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_i16(VAR_I16_NAME, 1, &RECORD_I16_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I16_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I16_NAME).unwrap();
            assert_eq!(DataType::I16, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i16> = file_reader.read_var_i16(VAR_I16_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I16; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i32() {
    const TEST_FILE_NAME: &str = "test_write_record_i32.nc";

    const VAR_I32_NAME: &str = "var_i32";
    const RECORD_1_I32_DATA: [i32; FIXED_DIM_SIZE] = [1, 2, 3, 4];
    const RECORD_3_I32_DATA: [i32; FIXED_DIM_SIZE] = [5, 6, 7, 8];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // Write the record #1
        file_writer.write_record_i32(VAR_I32_NAME, 1, &RECORD_1_I32_DATA).unwrap();
        // Write the record #3
        file_writer.write_record_i32(VAR_I32_NAME, 3, &RECORD_3_I32_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I32_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I32_NAME).unwrap();
            assert_eq!(DataType::I32, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i32> = file_reader.read_var_i32(VAR_I32_NAME).unwrap();
        file_reader.close();

        let record_0: &[i32] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[i32] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[i32] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[i32] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[i32] = &var_data[4 * chunk_len..5 * chunk_len];

        assert_eq!(vec![NC_FILL_I32; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_I32_DATA, record_1);
        assert_eq!(vec![NC_FILL_I32; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_I32_DATA, record_3);
        assert_eq!(vec![NC_FILL_I32; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_i32_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i32_errors.nc";
    const VAR_I32_NAME: &str = "var_i32";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_I32_DATA: [i32; FIXED_DIM_SIZE] = [1, 2, 3, 4];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_i32(UNDEF_VAR_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::I32
            },
            file_writer.write_record_i32(VAR_I8_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I32
            },
            file_writer.write_record_i32(VAR_U8_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::I32
            },
            file_writer.write_record_i32(VAR_I16_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I32
            },
            file_writer.write_record_i32(VAR_F32_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I32
            },
            file_writer.write_record_i32(VAR_F64_NAME, 1, &RECORD_I32_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_i32(VAR_I32_NAME, UNLIM_DIM_SIZE, &RECORD_I32_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_I32_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_i32(VAR_I32_NAME, 1, &RECORD_I32_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I32_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I32_NAME).unwrap();
            assert_eq!(DataType::I32, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<i32> = file_reader.read_var_i32(VAR_I32_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I32; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_f32() {
    const TEST_FILE_NAME: &str = "test_write_record_f32.nc";

    const VAR_F32_NAME: &str = "var_f32";
    const RECORD_1_F32_DATA: [f32; FIXED_DIM_SIZE] = [1.0, 2.0, 3.0, 4.0];
    const RECORD_3_F32_DATA: [f32; FIXED_DIM_SIZE] = [5.0, 6.0, 7.0, 8.0];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // write record #1
        file_writer.write_record_f32(VAR_F32_NAME, 1, &RECORD_1_F32_DATA).unwrap();
        // write record #3
        file_writer.write_record_f32(VAR_F32_NAME, 3, &RECORD_3_F32_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F32_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F32_NAME).unwrap();
            assert_eq!(DataType::F32, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<f32> = file_reader.read_var_f32(VAR_F32_NAME).unwrap();
        file_reader.close();

        let record_0: &[f32] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[f32] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[f32] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[f32] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[f32] = &var_data[4 * chunk_len..5 * chunk_len];
        assert_eq!(vec![NC_FILL_F32; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_F32_DATA, record_1);
        assert_eq!(vec![NC_FILL_F32; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_F32_DATA, record_3);
        assert_eq!(vec![NC_FILL_F32; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_f32_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_f32_errors.nc";
    const VAR_F32_NAME: &str = "var_f32";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_F32_DATA: [f32; FIXED_DIM_SIZE] = [1.0, 2.0, 3.0, 4.0];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_f32(UNDEF_VAR_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::F32
            },
            file_writer.write_record_f32(VAR_I8_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::F32
            },
            file_writer.write_record_f32(VAR_U8_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::F32
            },
            file_writer.write_record_f32(VAR_I16_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::F32
            },
            file_writer.write_record_f32(VAR_I32_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::F32
            },
            file_writer.write_record_f32(VAR_F64_NAME, 1, &RECORD_F32_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_f32(VAR_F32_NAME, UNLIM_DIM_SIZE, &RECORD_F32_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_F32_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_f32(VAR_F32_NAME, 1, &RECORD_F32_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F32_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F32_NAME).unwrap();
            assert_eq!(DataType::F32, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<f32> = file_reader.read_var_f32(VAR_F32_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_F32; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_f64() {
    const TEST_FILE_NAME: &str = "test_write_record_f64.nc";

    const VAR_F64_NAME: &str = "var_f64";
    const RECORD_1_F64_DATA: [f64; FIXED_DIM_SIZE] = [1.0, 2.0, 3.0, 4.0];
    const RECORD_3_F64_DATA: [f64; FIXED_DIM_SIZE] = [5.0, 6.0, 7.0, 8.0];

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        // write record #1
        file_writer.write_record_f64(VAR_F64_NAME, 1, &RECORD_1_F64_DATA).unwrap();
        // write record #3
        file_writer.write_record_f64(VAR_F64_NAME, 3, &RECORD_3_F64_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F64_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F64_NAME).unwrap();
            assert_eq!(DataType::F64, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<f64> = file_reader.read_var_f64(VAR_F64_NAME).unwrap();
        file_reader.close();

        let record_0: &[f64] = &var_data[0 * chunk_len..1 * chunk_len];
        let record_1: &[f64] = &var_data[1 * chunk_len..2 * chunk_len];
        let record_2: &[f64] = &var_data[2 * chunk_len..3 * chunk_len];
        let record_3: &[f64] = &var_data[3 * chunk_len..4 * chunk_len];
        let record_4: &[f64] = &var_data[4 * chunk_len..5 * chunk_len];
        assert_eq!(vec![NC_FILL_F64; FIXED_DIM_SIZE], record_0);
        assert_eq!(RECORD_1_F64_DATA, record_1);
        assert_eq!(vec![NC_FILL_F64; FIXED_DIM_SIZE], record_2);
        assert_eq!(RECORD_3_F64_DATA, record_3);
        assert_eq!(vec![NC_FILL_F64; FIXED_DIM_SIZE], record_4);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_record_f64_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_f64_errors.nc";
    const VAR_F64_NAME: &str = "var_f64";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 3;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const UNDEF_VAR_NAME: &str = "undef_var";

        const RECORD_F64_DATA: [f64; FIXED_DIM_SIZE] = [1.0, 2.0, 3.0, 4.0];

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_record_f64(UNDEF_VAR_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::F64
            },
            file_writer.write_record_f64(VAR_I8_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::F64
            },
            file_writer.write_record_f64(VAR_U8_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::F64
            },
            file_writer.write_record_f64(VAR_I16_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::F64
            },
            file_writer.write_record_f64(VAR_I32_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::F64
            },
            file_writer.write_record_f64(VAR_F32_NAME, 1, &RECORD_F64_DATA[..]).unwrap_err()
        );

        assert_eq!(
            WriteError::RecordIndexExceeded {
                index: UNLIM_DIM_SIZE,
                num_records: UNLIM_DIM_SIZE
            },
            file_writer
                .write_record_f64(VAR_F64_NAME, UNLIM_DIM_SIZE, &RECORD_F64_DATA[..])
                .unwrap_err()
        );

        assert_eq!(
            WriteError::RecordMismatchDataLength {
                var_name: VAR_F64_NAME.to_string(),
                req: FIXED_DIM_SIZE,
                get: FIXED_DIM_SIZE - 1
            },
            file_writer
                .write_record_f64(VAR_F64_NAME, 1, &RECORD_F64_DATA[..FIXED_DIM_SIZE - 1])
                .unwrap_err()
        );

        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F64_NAME));
        let chunk_len: usize;
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F64_NAME).unwrap();
            assert_eq!(DataType::F64, var.data_type());
            assert_eq!(true, var.is_record_var());

            chunk_len = var.chunk_len();
            assert_eq!(UNLIM_DIM_SIZE, var.num_chunks());
            assert_eq!(FIXED_DIM_SIZE, chunk_len);
        }

        let var_data: Vec<f64> = file_reader.read_var_f64(VAR_F64_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_F64; UNLIM_DIM_SIZE * FIXED_DIM_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_dims_list() {
    // Empty dimension list
    {
        let bytes: Vec<u8> = {
            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[]).unwrap();
            bytes
        };

        assert_eq!(ABSENT_TAG.len(), bytes.len());
        assert_eq!(&ABSENT_TAG[..], &bytes[..]);
    }

    // One *fixed_size* dimension list
    {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;
        let mut cursor: Cursor<Vec<u8>> = {
            let dim_1 = Rc::new(Dimension::new_fixed_size(DIM_NAME, DIM_SIZE).unwrap());

            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[dim_1]).unwrap();
            Cursor::new(bytes)
        };

        let mut buffer: Vec<u8> = vec![0_u8; 4];
        cursor.read(&mut buffer).unwrap();
        assert_eq!(&DIMENSION_TAG[..], &buffer[..]);
        // the number of dimensions
        assert_eq!(1, cursor.read_i32::<BigEndian>().unwrap());
        // the number of useful bytes for the dim name
        assert_eq!(5, cursor.read_i32::<BigEndian>().unwrap());
        // the dim name bytes
        assert_eq!('d' as u8, cursor.read_u8().unwrap());
        assert_eq!('i' as u8, cursor.read_u8().unwrap());
        assert_eq!('m' as u8, cursor.read_u8().unwrap());
        assert_eq!('_' as u8, cursor.read_u8().unwrap());
        assert_eq!('1' as u8, cursor.read_u8().unwrap());
        // the zero padding bytes
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        // the dimension size
        assert_eq!(DIM_SIZE as i32, cursor.read_i32::<BigEndian>().unwrap());
        // no byte remaining
        assert_eq!(0, cursor.read_to_end(&mut buffer).unwrap());
    }

    // One *unlimited_size* dimension list
    {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;
        let mut cursor: Cursor<Vec<u8>> = {
            let dim_1 = Rc::new(Dimension::new_unlimited_size(DIM_NAME, DIM_SIZE).unwrap());

            let mut bytes: Vec<u8> = vec![];
            let _ = FileWriter::write_dims_list(&mut bytes, &[dim_1]).unwrap();
            Cursor::new(bytes)
        };

        let mut buffer: Vec<u8> = vec![0_u8; 4];
        cursor.read(&mut buffer).unwrap();
        assert_eq!(&DIMENSION_TAG[..], &buffer[..]);
        // the number of dimensions
        assert_eq!(1, cursor.read_i32::<BigEndian>().unwrap());
        // the number of useful bytes for the dim name
        assert_eq!(5, cursor.read_i32::<BigEndian>().unwrap());
        // the dim name bytes
        assert_eq!('d' as u8, cursor.read_u8().unwrap());
        assert_eq!('i' as u8, cursor.read_u8().unwrap());
        assert_eq!('m' as u8, cursor.read_u8().unwrap());
        assert_eq!('_' as u8, cursor.read_u8().unwrap());
        assert_eq!('1' as u8, cursor.read_u8().unwrap());
        // the zero padding bytes
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        // the dimension size: the unlimited dimension is not record here
        assert_eq!(0 as i32, cursor.read_i32::<BigEndian>().unwrap());
        // no byte remaining
        assert_eq!(0, cursor.read_to_end(&mut buffer).unwrap());
    }
}

#[test]
fn test_write_name_string() {
    // 1 ASCII character string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "a").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(8, cursor.get_ref().len());
        assert_eq!(1, cursor.read_i32::<BigEndian>().unwrap()); // the number of useful bytes
        assert_eq!('a' as u8, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
    }

    // 4 ASCII characters string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "abcd").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(8, cursor.get_ref().len());
        assert_eq!(4, cursor.read_i32::<BigEndian>().unwrap()); // the number of useful bytes
        assert_eq!('a' as u8, cursor.read_u8().unwrap());
        assert_eq!('b' as u8, cursor.read_u8().unwrap());
        assert_eq!('c' as u8, cursor.read_u8().unwrap());
        assert_eq!('d' as u8, cursor.read_u8().unwrap());
    }

    // 5 ASCII characters string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "abcde").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(12, cursor.get_ref().len());
        assert_eq!(5, cursor.read_i32::<BigEndian>().unwrap()); // the number of useful cursor
        assert_eq!('a' as u8, cursor.read_u8().unwrap());
        assert_eq!('b' as u8, cursor.read_u8().unwrap());
        assert_eq!('c' as u8, cursor.read_u8().unwrap());
        assert_eq!('d' as u8, cursor.read_u8().unwrap());
        assert_eq!('e' as u8, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
    }

    // UTF-8 encoded string
    {
        let mut cursor: Cursor<Vec<u8>> = {
            let mut bytes: Vec<u8> = vec![];
            FileWriter::write_name_string(&mut bytes, "café").unwrap();
            Cursor::new(bytes)
        };

        assert_eq!(12, cursor.get_ref().len());
        assert_eq!(5, cursor.read_i32::<BigEndian>().unwrap()); // the number of useful cursor
        assert_eq!('c' as u8, cursor.read_u8().unwrap());
        assert_eq!('a' as u8, cursor.read_u8().unwrap());
        assert_eq!('f' as u8, cursor.read_u8().unwrap());
        assert_eq!(0xc3, cursor.read_u8().unwrap());
        assert_eq!(0xa9, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
        assert_eq!(0, cursor.read_u8().unwrap());
    }
}

#[test]
fn test_write_var_i8() {
    const TEST_FILE_NAME: &str = "test_write_var_i8.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I8_NAME: &str = "var_i8";
    const VAR_I8_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I8_DATA: [i8; VAR_I8_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_i8(VAR_I8_NAME, &VAR_I8_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I8_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_I8_NAME).unwrap();
        assert_eq!(DataType::I8, var.data_type());

        let var_data: Vec<i8> = file_reader.read_var_i8(VAR_I8_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_I8_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_i8_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i8_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I8_NAME: &str = "var_i8";
    const VAR_I8_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I8_DATA: [i8; VAR_I8_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_i8(UNDEF_VAR_NAME, &VAR_I8_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I8
            },
            file_writer.write_var_i8(VAR_U8_NAME, &VAR_I8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::I8
            },
            file_writer.write_var_i8(VAR_I16_NAME, &VAR_I8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::I8
            },
            file_writer.write_var_i8(VAR_I32_NAME, &VAR_I8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I8
            },
            file_writer.write_var_i8(VAR_F32_NAME, &VAR_I8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I8
            },
            file_writer.write_var_i8(VAR_F64_NAME, &VAR_I8_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I8_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I8_NAME).unwrap();
            assert_eq!(DataType::I8, var.data_type());
        }

        let var_data: Vec<i8> = file_reader.read_var_i8(VAR_I8_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I8; VAR_I8_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_u8() {
    const TEST_FILE_NAME: &str = "test_write_var_u8.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_U8_NAME: &str = "var_u8";
    const VAR_U8_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_U8_DATA: [u8; VAR_U8_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_u8(VAR_U8_NAME, &VAR_U8_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_U8_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_U8_NAME).unwrap();
        assert_eq!(DataType::U8, var.data_type());

        let var_data: Vec<u8> = file_reader.read_var_u8(VAR_U8_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_U8_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_u8_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_u8_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_U8_NAME: &str = "var_u8";
    const VAR_U8_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_U8_DATA: [u8; VAR_U8_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_u8(UNDEF_VAR_NAME, &VAR_U8_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::U8
            },
            file_writer.write_var_u8(VAR_I8_NAME, &VAR_U8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::U8
            },
            file_writer.write_var_u8(VAR_I16_NAME, &VAR_U8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::U8
            },
            file_writer.write_var_u8(VAR_I32_NAME, &VAR_U8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::U8
            },
            file_writer.write_var_u8(VAR_F32_NAME, &VAR_U8_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::U8
            },
            file_writer.write_var_u8(VAR_F64_NAME, &VAR_U8_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_U8_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_U8_NAME).unwrap();
            assert_eq!(DataType::U8, var.data_type());
        }

        let var_data: Vec<u8> = file_reader.read_var_u8(VAR_U8_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_U8; VAR_U8_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_i16() {
    const TEST_FILE_NAME: &str = "test_write_var_i16.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I16_NAME: &str = "var_i16";
    const VAR_I16_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I16_DATA: [i16; VAR_I16_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_i16(VAR_I16_NAME, &VAR_I16_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I16_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_I16_NAME).unwrap();
        assert_eq!(DataType::I16, var.data_type());

        let var_data: Vec<i16> = file_reader.read_var_i16(VAR_I16_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_I16_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_i16_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i16_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I16_NAME: &str = "var_i16";
    const VAR_I16_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I16_DATA: [i16; VAR_I16_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_i16(UNDEF_VAR_NAME, &VAR_I16_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::I16
            },
            file_writer.write_var_i16(VAR_I8_NAME, &VAR_I16_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I16
            },
            file_writer.write_var_i16(VAR_U8_NAME, &VAR_I16_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::I16
            },
            file_writer.write_var_i16(VAR_I32_NAME, &VAR_I16_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I16
            },
            file_writer.write_var_i16(VAR_F32_NAME, &VAR_I16_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I16
            },
            file_writer.write_var_i16(VAR_F64_NAME, &VAR_I16_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I16_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I16_NAME).unwrap();
            assert_eq!(DataType::I16, var.data_type());
        }

        let var_data: Vec<i16> = file_reader.read_var_i16(VAR_I16_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I16; VAR_I16_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_i32() {
    const TEST_FILE_NAME: &str = "test_write_var_i32.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I32_NAME: &str = "var_i32";
    const VAR_I32_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I32_DATA: [i32; VAR_I32_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_i32(VAR_I32_NAME, &VAR_I32_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I32_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_I32_NAME).unwrap();
        assert_eq!(DataType::I32, var.data_type());

        let var_data: Vec<i32> = file_reader.read_var_i32(VAR_I32_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_I32_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_i32_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_i32_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_I32_NAME: &str = "var_i32";
    const VAR_I32_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_I32_DATA: [i32; VAR_I32_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_F32_NAME: &str = "var_f32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_i32(UNDEF_VAR_NAME, &VAR_I32_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::I32
            },
            file_writer.write_var_i32(VAR_I8_NAME, &VAR_I32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::I32
            },
            file_writer.write_var_i32(VAR_U8_NAME, &VAR_I32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::I32
            },
            file_writer.write_var_i32(VAR_I16_NAME, &VAR_I32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::I32
            },
            file_writer.write_var_i32(VAR_F32_NAME, &VAR_I32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::I32
            },
            file_writer.write_var_i32(VAR_F64_NAME, &VAR_I32_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_I32_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_I32_NAME).unwrap();
            assert_eq!(DataType::I32, var.data_type());
        }

        let var_data: Vec<i32> = file_reader.read_var_i32(VAR_I32_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_I32; VAR_I32_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_f32() {
    const TEST_FILE_NAME: &str = "test_write_var_f32.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_F32_NAME: &str = "var_f32";
    const VAR_F32_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_F32_DATA: [f32; VAR_F32_SIZE] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
    ];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_f32(VAR_F32_NAME, &VAR_F32_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F32_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_F32_NAME).unwrap();
        assert_eq!(DataType::F32, var.data_type());

        let var_data: Vec<f32> = file_reader.read_var_f32(VAR_F32_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_F32_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_f32_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_f32_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_F32_NAME: &str = "var_f32";
    const VAR_F32_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_F32_DATA: [f32; VAR_F32_SIZE] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
    ];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F64_NAME: &str = "var_f64";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_f32(UNDEF_VAR_NAME, &VAR_F32_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::F32
            },
            file_writer.write_var_f32(VAR_I8_NAME, &VAR_F32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::F32
            },
            file_writer.write_var_f32(VAR_U8_NAME, &VAR_F32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::F32
            },
            file_writer.write_var_f32(VAR_I16_NAME, &VAR_F32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::F32
            },
            file_writer.write_var_f32(VAR_I32_NAME, &VAR_F32_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F64_NAME.to_string(),
                req: DataType::F64,
                get: DataType::F32
            },
            file_writer.write_var_f32(VAR_F64_NAME, &VAR_F32_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F32_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F32_NAME).unwrap();
            assert_eq!(DataType::F32, var.data_type());
        }

        let var_data: Vec<f32> = file_reader.read_var_f32(VAR_F32_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_F32; VAR_F32_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_f64() {
    const TEST_FILE_NAME: &str = "test_write_var_f64.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_F64_NAME: &str = "var_f64";
    const VAR_F64_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_F64_DATA: [f64; VAR_F64_SIZE] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
    ];

    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create and write a new NetCDF-3 file
    {
        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();

        file_writer.set_def(&data_set, Version::Classic, 0).unwrap();
        file_writer.write_var_f64(VAR_F64_NAME, &VAR_F64_DATA).unwrap();
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet file
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F64_NAME));
        let var: &Variable = file_reader.data_set().get_var(VAR_F64_NAME).unwrap();
        assert_eq!(DataType::F64, var.data_type());

        let var_data: Vec<f64> = file_reader.read_var_f64(VAR_F64_NAME).unwrap();
        file_reader.close();
        assert_eq!(&VAR_F64_DATA[..], &var_data[..]);
    }
    tmp_dir.close().unwrap();
}

#[test]
fn test_write_var_f64_errors() {
    const TEST_FILE_NAME: &str = "test_write_record_f64_errors.nc";

    const UNLIM_DIM_NAME: &str = "unlimited_dim";
    const UNLIM_DIM_SIZE: usize = 5;
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 4;

    const VAR_F64_NAME: &str = "var_f64";
    const VAR_F64_SIZE: usize = UNLIM_DIM_SIZE * FIXED_DIM_SIZE;
    const VAR_F64_DATA: [f64; VAR_F64_SIZE] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
    ];
    let tmp_dir: TempDir = TempDir::new(TMP_DIR_PREFIX).unwrap();
    let test_file_path: PathBuf = tmp_dir.path().join(TEST_FILE_NAME);
    assert_eq!(false, test_file_path.exists());

    // First create a new NetCDF-3 file and try to write wrong invalid records
    {
        const VAR_I8_NAME: &str = "var_i8";
        const VAR_U8_NAME: &str = "var_u8";
        const VAR_I16_NAME: &str = "var_i16";
        const VAR_I32_NAME: &str = "var_i32";
        const VAR_F32_NAME: &str = "var_f32";
        const UNDEF_VAR_NAME: &str = "undef_var";

        let mut file_writer: FileWriter = FileWriter::create_new(&test_file_path).unwrap();
        let mut data_set = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
        data_set.add_var_i8(VAR_I8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_u8(VAR_U8_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i16(VAR_I16_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_i32(VAR_I32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f32(VAR_F32_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        data_set.add_var_f64(VAR_F64_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
        file_writer.set_def(&data_set, Version::Classic, 0).unwrap(); // set an empty data set

        assert_eq!(
            WriteError::VariableNotDefined(UNDEF_VAR_NAME.to_string()),
            file_writer.write_var_f64(UNDEF_VAR_NAME, &VAR_F64_DATA).unwrap_err()
        );

        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I8_NAME.to_string(),
                req: DataType::I8,
                get: DataType::F64
            },
            file_writer.write_var_f64(VAR_I8_NAME, &VAR_F64_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_U8_NAME.to_string(),
                req: DataType::U8,
                get: DataType::F64
            },
            file_writer.write_var_f64(VAR_U8_NAME, &VAR_F64_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I16_NAME.to_string(),
                req: DataType::I16,
                get: DataType::F64
            },
            file_writer.write_var_f64(VAR_I16_NAME, &VAR_F64_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_I32_NAME.to_string(),
                req: DataType::I32,
                get: DataType::F64
            },
            file_writer.write_var_f64(VAR_I32_NAME, &VAR_F64_DATA).unwrap_err()
        );
        assert_eq!(
            WriteError::VariableMismatchDataType {
                var_name: VAR_F32_NAME.to_string(),
                req: DataType::F32,
                get: DataType::F64
            },
            file_writer.write_var_f64(VAR_F32_NAME, &VAR_F64_DATA).unwrap_err()
        );
        file_writer.close().unwrap();
    }
    assert_eq!(true, test_file_path.exists());

    // Then read the outlet variable
    {
        let mut file_reader: FileReader = FileReader::open(test_file_path).unwrap();
        assert_eq!(true, file_reader.data_set().has_var(VAR_F64_NAME));
        {
            let var: &Variable = file_reader.data_set().get_var(VAR_F64_NAME).unwrap();
            assert_eq!(DataType::F64, var.data_type());
        }

        let var_data: Vec<f64> = file_reader.read_var_f64(VAR_F64_NAME).unwrap();
        file_reader.close();
        assert_eq!(vec![NC_FILL_F64; VAR_F64_SIZE], var_data);
    }

    tmp_dir.close().unwrap();
}
