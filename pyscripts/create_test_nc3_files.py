"""
This Python script allows to produce the NetCDF-3 files used by tests of the Rust crate `netcdf3`.
"""
import os.path
import argparse

import numpy as np

import netCDF4


#: Empty data set
EMPTY_DATA_SET_FILE_NAME = "empty_data_set.nc"
#: NetCDF-3 (classic version)
NC3_CLASSIC_FILE_NAME = "temp_3D_classic.nc"
#: NetCDF-3 (64-bit offset version)
NC3_64BIT_OFFSET_FILE_NAME = "temp_3D_64bit_offset.nc"
#: Scalar variables contaning the default `NC_FILL` values
NC3_FILL_VALUES_FILE_NAME = "nc_fill_values.nc"
#: Scalar variables
SCALAR_VARIABLES_FILE_NAME = "scalar_vars.nc"
#: Another NetCDF-3 (classic version)
NC3_LIGHT_CLASSIC_FILE_NAME = "temp_3D_classic_light.nc"
#: NetCDF-3 file containing a zero-sized unlimited dimension
NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME = "zero_sized_unlimited_dim.nc"
#: NetCDF-3 file containing default fill values (unset values).
NC3_CONTAINING_DEFAULT_FILL_VALUES_FILE_NAME = "containing_default_fill_values.nc"


def write_file_nc_fill_values(file_path: str):
    """
    Create a data set containing the default `NC_FILL` values.
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        ds.createVariable("nc_fill_value_i8", datatype=np.int8, dimensions=())
        ds.createVariable("nc_fill_value_u8", datatype='c', dimensions=())
        ds.createVariable("nc_fill_value_i16", datatype=np.int16, dimensions=())
        ds.createVariable("nc_fill_value_i32", datatype=np.int32, dimensions=())
        ds.createVariable("nc_fill_value_f32", datatype=np.float32, dimensions=())
        ds.createVariable("nc_fill_value_f64", datatype=np.float64, dimensions=())


def write_file_scalar_vars(file_path: str):
    """
    Create a data set containing scalar variables
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        var = ds.createVariable("scalar_value_i8", datatype=np.int8, dimensions=())
        var[0] = np.int8(42)
        var = ds.createVariable("scalar_value_u8", datatype='c', dimensions=())
        var[0] = b'\x2a'
        var = ds.createVariable("scalar_value_i16", datatype=np.int16, dimensions=())
        var[0] = np.int16(42)
        var = ds.createVariable("scalar_value_i32", datatype=np.int32, dimensions=())
        var[0] = np.int32(42)
        var = ds.createVariable("scalar_value_f32", datatype=np.float32, dimensions=())
        var[0] = np.float32(42.0)
        var = ds.createVariable("scalar_value_f64", datatype=np.float64, dimensions=())
        var[0] = np.float64(42.0)


def write_file_empty_data_set(file_path: str):
    """
    Create an empty data set file
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w"):
        pass


def write_file_basic_nc3_classic_light(file_path: str):
    """
    A basic and light NetCDF-3 file (classic version), used in examples of the crate docu
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        ds.createDimension("latitude", 3)
        ds.createDimension("longitude", 4)
        var = ds.createVariable("latitude", datatype=np.float32, dimensions=("latitude"))
        var[:] = np.array([0.0, 1.0, 2.0], dtype=np.float32)
        var = ds.createVariable("longitude", datatype=np.float32, dimensions=("longitude"))
        var[:] = np.array([0.0, 1.0, 2.0, 3.0], dtype=np.float32)
        var = ds.createVariable("temperature", datatype=np.float64, dimensions=("latitude", "longitude"))
        var[:] = np.reshape(
            np.array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0], dtype=np.float64),
            newshape=(3, 4),
        )


def write_file_zero_sized_unlimited_dim(file_path: str):
    """
    Create a NetCDF-3 file containing a zero-sized unlimited dimension
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        ds.createDimension("unlim_dim")

def define_temperatures_dataset(ds):

    TEMPERATURE_VAR_ATTRS = {
        "standard_name": "air_temperature",
        "long_name": "TEMPERATURE",
        "units": "Celsius",
    }

    LATITUDE_VAR_NAME = "latitude"
    LATITUDE_VAR_VALUES = np.array([0.0, 0.5, 1.0], dtype=np.float32)

    LONGITUDE_VAR_NAME = "longitude"
    LONGITUDE_VAR_VALUES = np.array([0.0, 0.5, 1.0, 1.5, 2.0], dtype=np.float32)

    TIME_VAR_NAME = "time"
    TIME_VAR_VALUES = np.array([438_300.0, 438_324.0], dtype=np.float32)

    TEMPERATURE_I8_VAR_NAME = "temperature_i8"
    TEMPERATURE_U8_VAR_NAME = "temperature_u8"
    TEMPERATURE_I16_VAR_NAME = "temperature_i16"
    TEMPERATURE_I32_VAR_NAME = "temperature_i32"
    TEMPERATURE_F32_VAR_NAME = "temperature_f32"
    TEMPERATURE_F64_VAR_NAME = "temperature_f64"

    TEMPERATURE_DIMS_LIST = (TIME_VAR_NAME, LATITUDE_VAR_NAME, LONGITUDE_VAR_NAME)
    TEMPERATURE_NUM_VALUES = TIME_VAR_VALUES.size * LATITUDE_VAR_VALUES.size * LONGITUDE_VAR_VALUES.size
    TEMPERATURE_SHAPE = (TIME_VAR_VALUES.size, LATITUDE_VAR_VALUES.size, LONGITUDE_VAR_VALUES.size)
    TEMPERATURE_VAR_VALUES = np.reshape(np.arange(TEMPERATURE_NUM_VALUES, dtype=np.int64), TEMPERATURE_SHAPE)

    # First define the dimensions
    # ---------------------------
    ds.createDimension(LATITUDE_VAR_NAME, LATITUDE_VAR_VALUES.size)
    ds.createDimension(LONGITUDE_VAR_NAME, LONGITUDE_VAR_VALUES.size)
    ds.createDimension(TIME_VAR_NAME)

    # Second define the variables, their attributes and set their data
    # ----------------------------------------------------------------
    # `latitude`
    latitude_var = ds.createVariable(LATITUDE_VAR_NAME, datatype=np.float32, dimensions=LATITUDE_VAR_NAME)
    latitude_var.setncattr("standard_name", "latitude")
    latitude_var.setncattr("long_name", "LATITUDE")
    latitude_var.setncattr("units", "degrees_north")
    latitude_var.setncattr("axis", "Y")
    latitude_var[:] = LATITUDE_VAR_VALUES
    # `longitude`
    longitude_var = ds.createVariable(LONGITUDE_VAR_NAME, datatype=np.float32, dimensions=LONGITUDE_VAR_NAME)
    longitude_var.setncattr("standard_name", "longitude")
    longitude_var.setncattr("long_name", "LONGITUDE")
    longitude_var.setncattr("units", "degrees_east")
    longitude_var.setncattr("axis", "X")
    longitude_var[:] = LONGITUDE_VAR_VALUES
    # `time`
    time_var = ds.createVariable(TIME_VAR_NAME, datatype=np.float32, dimensions=TIME_VAR_NAME)
    time_var.setncattr("standard_name", "time")
    time_var.setncattr("long_name", "TIME")
    time_var.setncattr("units", "hours since 1970-01-01 00:00:00")
    time_var.setncattr("calendar", "gregorian")
    time_var.setncattr("axis", "T")
    time_var[:] = TIME_VAR_VALUES
    # `temperature_i8`
    temperature_i8_var = ds.createVariable(TEMPERATURE_I8_VAR_NAME, datatype=np.int8, dimensions=TEMPERATURE_DIMS_LIST)
    temperature_i8_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_i8_var[:] = np.asarray(TEMPERATURE_VAR_VALUES, dtype=np.int8)

    temperature_u8_var = ds.createVariable(TEMPERATURE_U8_VAR_NAME, datatype='c', dimensions=TEMPERATURE_DIMS_LIST)
    temperature_u8_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_u8_var[0, 0, 0] = b'\x00'
    temperature_u8_var[0, 0, 1] = b'\x01'
    temperature_u8_var[0, 0, 2] = b'\x02'
    temperature_u8_var[0, 0, 3] = b'\x03'
    temperature_u8_var[0, 0, 4] = b'\x04'
    temperature_u8_var[0, 1, 0] = b'\x05'
    temperature_u8_var[0, 1, 1] = b'\x06'
    temperature_u8_var[0, 1, 2] = b'\x07'
    temperature_u8_var[0, 1, 3] = b'\x08'
    temperature_u8_var[0, 1, 4] = b'\x09'
    temperature_u8_var[0, 2, 0] = b'\x0a'
    temperature_u8_var[0, 2, 1] = b'\x0b'
    temperature_u8_var[0, 2, 2] = b'\x0c'
    temperature_u8_var[0, 2, 3] = b'\x0d'
    temperature_u8_var[0, 2, 4] = b'\x0e'
    temperature_u8_var[1, 0, 0] = b'\x0f'
    temperature_u8_var[1, 0, 1] = b'\x10'
    temperature_u8_var[1, 0, 2] = b'\x11'
    temperature_u8_var[1, 0, 3] = b'\x12'
    temperature_u8_var[1, 0, 4] = b'\x13'
    temperature_u8_var[1, 1, 0] = b'\x14'
    temperature_u8_var[1, 1, 1] = b'\x15'
    temperature_u8_var[1, 1, 2] = b'\x16'
    temperature_u8_var[1, 1, 3] = b'\x17'
    temperature_u8_var[1, 1, 4] = b'\x18'
    temperature_u8_var[1, 2, 0] = b'\x19'
    temperature_u8_var[1, 2, 1] = b'\x1a'
    temperature_u8_var[1, 2, 2] = b'\x1b'
    temperature_u8_var[1, 2, 3] = b'\x1c'
    temperature_u8_var[1, 2, 4] = b'\x1d'

    temperature_i16_var = ds.createVariable(TEMPERATURE_I16_VAR_NAME, datatype=np.int16, dimensions=TEMPERATURE_DIMS_LIST)
    temperature_i16_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_i16_var[:] = np.asarray(TEMPERATURE_VAR_VALUES, dtype=np.int16)

    temperature_i32_var = ds.createVariable(TEMPERATURE_I32_VAR_NAME, datatype=np.int32, dimensions=TEMPERATURE_DIMS_LIST)
    temperature_i32_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_i32_var[:] = np.asarray(TEMPERATURE_VAR_VALUES, dtype=np.int32)

    temperature_f32_var = ds.createVariable(TEMPERATURE_F32_VAR_NAME, datatype=np.float32, dimensions=TEMPERATURE_DIMS_LIST)
    temperature_f32_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_f32_var[:] = np.asarray(TEMPERATURE_VAR_VALUES, dtype=np.float32)

    temperature_f64_var = ds.createVariable(TEMPERATURE_F64_VAR_NAME, datatype=np.float64, dimensions=TEMPERATURE_DIMS_LIST)
    temperature_f64_var.setncatts(TEMPERATURE_VAR_ATTRS)
    temperature_f64_var[:] = np.asarray(TEMPERATURE_VAR_VALUES, dtype=np.float64)


def write_file_nc3_classic(file_path: str):
    """
    Create a `NETCDF3_CLASSIC` data file
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        # Define global attributes
        ds.setncatts({
            "title": "Example of NETCDF3_CLASSIC file",
            "Conventions": "CF-1.8",
        })
        define_temperatures_dataset(ds)


def write_file_nc3_64bit_offset(file_path: str):
    """
    Create a `NETCDF3_64BIT_OFFSET` data file.
    """
    with netCDF4.Dataset(file_path, format="NETCDF3_64BIT_OFFSET", mode="w") as ds:
        ds.setncatts({
            "title": "Example of NETCDF3_64BIT_OFFSET file",
            "Conventions": "CF-1.8",
        })
        define_temperatures_dataset(ds)


def write_file_nc3_containing_default_fill_values(file_path: str):
    """
    Create a NetCDF3 file containing default `_FillValue`s of each data type.
    """
    def define_dataset_containing_default_fill_values(ds: netCDF4.Dataset):
        DIM_NAME = "dimension_0"

        ds.createDimension(DIM_NAME)

        dim_var = ds.createVariable(DIM_NAME, datatype=np.int32, dimensions=DIM_NAME)
        dim_var[:] = np.asarray([1, 2, 3], dtype=np.int32)

        var_i8 = ds.createVariable("var_i8", datatype=np.int8, dimensions=DIM_NAME)
        var_i8[0] = 1
        # var_i8[1] = 2
        var_i8[2] = 3

        var_u8 = ds.createVariable("var_u8", datatype='c', dimensions=DIM_NAME)
        var_u8[0] = b'\x01'
        # var_u8[1] = b'\x02'
        var_u8[2] = b'\x03'

        var_i16 = ds.createVariable("var_i16", datatype=np.int16, dimensions=DIM_NAME)
        var_i16[0] = 1
        # var_i16[1] = 2
        var_i16[2] = 3

        var_i32 = ds.createVariable("var_i32", datatype=np.int32, dimensions=DIM_NAME)
        var_i32[0] = 1
        # var_i32[1] = 2
        var_i32[2] = 3

        var_f32 = ds.createVariable("var_f32", datatype=np.float32, dimensions=DIM_NAME)
        var_f32[0] = 1.0
        # var_f32[1] = 2.0
        var_f32[2] = 3.0

        var_f64 = ds.createVariable("var_f64", datatype=np.float64, dimensions=DIM_NAME)
        var_f64[0] = 1.0
        # var_f64[1] = 2.0
        var_f64[2] = 3.0

    with netCDF4.Dataset(file_path, format="NETCDF3_CLASSIC", mode="w") as ds:
        define_dataset_containing_default_fill_values(ds)


def init_parser():
    """
    Initialze the command line parser
    """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "output_dir",
        metavar="OUTPUT_DIR",
        nargs=1,
        type=str,
        help="Path of the output data directory"
    )
    return parser


if __name__ == "__main__":
    parser = init_parser()
    args = parser.parse_args()
    output_dir = args.output_dir[0]

    # Create an empty data set file
    write_file_empty_data_set(os.path.join(output_dir, EMPTY_DATA_SET_FILE_NAME))

    # Create a `NETCDF3_CLASSIC` data file
    write_file_nc3_classic(os.path.join(output_dir, NC3_CLASSIC_FILE_NAME))

    # Create a `NETCDF3_64BIT_OFFSET` data file
    write_file_nc3_64bit_offset(os.path.join(output_dir, NC3_64BIT_OFFSET_FILE_NAME))

    # Create a data set containing the default `NC_FILL` values
    write_file_nc_fill_values(os.path.join(output_dir, NC3_FILL_VALUES_FILE_NAME))

    # Create a data set containing scalar variables
    write_file_scalar_vars(os.path.join(output_dir, SCALAR_VARIABLES_FILE_NAME))

    # Create another basic `NETCDF3_CLASSIC` data file
    write_file_basic_nc3_classic_light(os.path.join(output_dir, NC3_LIGHT_CLASSIC_FILE_NAME))

    # Create another basic `NETCDF3_CLASSIC` data file
    write_file_zero_sized_unlimited_dim(os.path.join(output_dir, NC3_ZERO_SIZED_UNLIMITED_DIM_FILE_NAME))

    # Create a data set containing the default `_FillValue` (unset values)
    write_file_nc3_containing_default_fill_values(os.path.join(output_dir, NC3_CONTAINING_DEFAULT_FILL_VALUES_FILE_NAME))
