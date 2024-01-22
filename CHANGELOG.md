# Changelog

## [Unreleased] - [unreleased]

### Added

- Initialize and apply the *Rust* formatter `rustfmt` to the source code.
- Initialize the *GitHub* CI workflows

### Changed

- Upgrade the minimum version of rustc  from *1.44.0* to *1.51.0*

## 0.5.2 - 2022-02-14

### Changed

- Move all unit tests into `tests.rs` files to not reduce the code coverage involuntarily.
- Upgrade the version of the dependency `nom` to *7.1.0*.

## 0.5.1 - 2020-12-22

### Added

- Add the `struct error::ParseHeaderError` to the public API.

### Changed

- Improve and debug the method `FileReader::parse_header`, can parse truncated NetCDF-3 headers without panics.

## 0.5.0 - 2020-12-13

### Changed

- Rename methods `FileReader::read_var_to_XX` to `FileReader::read_var_XX`, namely :
    - `read_var_i8`
    - `read_var_u8`
    - `read_var_i16`
    - `read_var_i32`
    - `read_var_f32`
    - `read_var_f64`

### Added

- Add methods `FileReader::read_record_XX` , namely :
    - `read_record`
    - `read_record_i8`
    - `read_record_u8`
    - `read_record_i16`
    - `read_record_i32`
    - `read_record_f32`
    - `read_record_f64`

- Add methods `FileWriter::write_record_XX` , namely:
    - `write_record_i8`
    - `write_record_u8`
    - `write_record_i16`
    - `write_record_i32`
    - `write_record_f32`
    - `write_record_f64`

## 0.4.0 - 2020-05-27

### Added

- Manage the indeterminated value of the number of records (`numrecs = 2^32 - 1`) while the reading and the writing ([File Format Specifications][File_Format_Specs]).
- Manage the indeterminated value of the chunk size for each variable (`vsize = 2^32 - 1`) while the reading and the writing ([File Format Specifications][File_Format_Specs]).
- Set the maximum size of the NetCDF-3 names (`NC_MAX_NAME_SIZE = 256`).

[File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html

## 0.3.1 - 2020-05-22

### Changed

- Correct the file `README.md`.

## 0.3.0 - 2020-05-22

### Changed

- Set the library is under the licenses `MIT OR Apache-2.0`.
- Change the `struct DataSet`. It does not contain the variable data.
- Change the error `enum`s.
- Set the maximum size of the *fixed-size* dimensions (`NC_MAX_DIM_SIZE = 2_147_483_644`).
- Set the maximum number of dimensions per variable (`NC_MAX_VAR_DIMS = 1_024`).

### Added

- Add the `struct FileWriter`. It allows to write the NetCDF-3 file.

## 0.2.0 - 2020-05-04

### Changed

- Change the error `enum`s.
- Change the `struct DataSet`.
- Change the `struct FileReader`.

## 0.1.0 - 2020-04-28 [YANKED]

### Added

- Initial release
