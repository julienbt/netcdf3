# netcdf3

![Crates.io](https://img.shields.io/crates/l/netcdf3)
[![Crates.io Version](https://img.shields.io/crates/v/netcdf3.svg)](https://crates.io/crates/netcdf3)
[![Documentation](https://docs.rs/netcdf3/badge.svg)](https://docs.rs/netcdf3)
[![Build Status](https://travis-ci.com/julienbt/netcdf3.svg?branch=main)](https://app.travis-ci.com/github/julienbt/netcdf3)
[![codecov](https://codecov.io/gh/julienbt/netcdf3/branch/main/graph/badge.svg?token=XTHF1A50ZG)](https://codecov.io/gh/julienbt/netcdf3)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51.0+-lightgray.svg)](#rust-version-requirements)

## Description

A pure Rust library for reading and writing NetCDF-3 files.

## Technical features

- [X] Define a NetCDF-3 data set :
    - [X] Create, get, rename, and remove global attributes.
    - [X] Create, get, rename, and remove dimensions.
    - [X] Create, get, rename, and remove variables.
    - [X] Create, get, rename, and remove variable attributes.
- [X] Read a NetCDF-3 file :
    - [X] Read all data of a variable.
    - [X] Read all data of a record (a part of a variable defined on one NetCDF-3 record).
    - [ ] Read a slice of data.
    - [ ] Read a variable's data into a N-dimensional array (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).
- [X] Write a NetCDF-3 file :
    - [X] Write all data of a variable.
    - [X] Write all data of a record (a part of a variable defined on one NetCDF-3 record).
    - [ ] Write a slice of data.
    - [ ] Write a variable's data from a N-dimensional array (using the crate [ndarray](https://github.com/rust-ndarray/ndarray)).

# Notes

- Validations are done by comparing files produced by this crate and files produced by the Python library [netCDF4](https://github.com/Unidata/netcdf4-python)(see the Python script `pyscripts/create_test_nc3_files.py` and the Rust test file `tests/tests_write_nc3_files.rs`).
- If the number of records `numrecs` is greater than `std::i32::MAX` then this value is considered as indeterminate and the actually written value is `numrecs = 2^32 - 1`(see the [File Format Specifications][File_Format_Specs]).
- If the chunk size of a given variable `vsize` is greater the `std::i32::MAX` then its value is considered as indeterminate and the actually written value is `vsize = 2^32 - 1` (see the [File Format Specifications][File_Format_Specs]).

## Known limitations

- Cannot read/write a subset of a variable data yet.
- Cannot rewrite a NetCDF-3 file.

## Examples

Various examples are available [here](https://docs.rs/netcdf3).

[File_Format_Specs]: https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html
