mod dimension;
pub(crate) use dimension::DimensionSize;
pub use dimension::{Dimension, DimensionType};

mod attribute;
pub use attribute::Attribute;

mod variable;
pub use variable::Variable;

mod tests;

use std::{cell::RefMut, ops::Deref, rc::Rc};

use crate::data_vector::DataVector;
use crate::{DataType, InvalidDataSet};

/// Default fill value for the `i8` elements (same value as `NC_FILL_BYTE` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_I8;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x81], NC_FILL_I8.to_be_bytes());
/// ```
pub const NC_FILL_I8: i8 = -127;
/// Default fill value for the `u8` elements (same value as `NC_FILL_CHAR` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_U8;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x00], NC_FILL_U8.to_be_bytes());
/// ```
pub const NC_FILL_U8: u8 = 0;
/// Default fill value for the `i16` elements (same value as `NC_FILL_SHORT` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_I16;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x80, 0x01], NC_FILL_I16.to_be_bytes());
/// ```
pub const NC_FILL_I16: i16 = -32767;
/// Default fill value for the `i32` elements (same value as `NC_FILL_INT` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_I32;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x80, 0x00, 0x00, 0x01], NC_FILL_I32.to_be_bytes());
/// ```
pub const NC_FILL_I32: i32 = -2147483647;
/// Default fill value for the `f32` elements (same value as `NC_FILL_FLOAT` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_F32;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x7c, 0xf0, 0x00, 0x00], NC_FILL_F32.to_be_bytes());
/// ```
pub const NC_FILL_F32: f32 = 9.9692099683868690e+36;
/// Default fill value for the `f64` elements (same value as `NC_FILL_DOUBLE` defined in the header file [netcdf.h](https://www.unidata.ucar.edu/software/netcdf/docs/netcdf_8h.html))
///
/// ```
/// # use netcdf3::NC_FILL_F64;
/// // Written bytes in the NetCDF-3 files
/// assert_eq!([0x47, 0x9e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], NC_FILL_F64.to_be_bytes());
/// ```
pub const NC_FILL_F64: f64 = 9.9692099683868690e+36;

/// Maximum length of the *fixed-size* dimensions
///
/// # Example: define a valid *fixed-size* dimension
///
/// ```
/// use netcdf3::{DataSet, NC_MAX_DIM_SIZE};
///
/// const DIM_NAME: &str = "dim_1";
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// assert_eq!(0,                       data_set.num_dims());
/// assert_eq!(false,                   data_set.has_dim(DIM_NAME));
/// assert_eq!(None,                    data_set.dim_size(DIM_NAME));
///
/// // Add a *fixed-size* dimension with the maximum size allowed.
/// data_set.add_fixed_dim(DIM_NAME, NC_MAX_DIM_SIZE).unwrap();
///
/// assert_eq!(1,                       data_set.num_dims());
/// assert_eq!(true,                    data_set.has_dim(DIM_NAME));
/// assert_eq!(Some(NC_MAX_DIM_SIZE),   data_set.dim_size(DIM_NAME));
/// ```
///
/// # Error: try to define a too lonng valid *fixed-size* dimension
///
/// ```
/// use netcdf3::{DataSet, NC_MAX_DIM_SIZE};
/// use netcdf3::error::InvalidDataSet;
///
/// const DIM_NAME: &str = "loo_long_dim";
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// assert_eq!(0,                       data_set.num_dims());
/// assert_eq!(false,                   data_set.has_dim(DIM_NAME));
/// assert_eq!(None,                    data_set.dim_size(DIM_NAME));
///
///
/// // Try to add another *fixed-size* dimension with a too long size.
/// assert_eq!(
///     InvalidDataSet::MaximumFixedDimensionSizeExceeded{
///         dim_name: String::from(DIM_NAME),
///         get: NC_MAX_DIM_SIZE + 1,
///     },
///     data_set.add_fixed_dim(DIM_NAME, NC_MAX_DIM_SIZE + 1).unwrap_err()
/// );
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// assert_eq!(0,                       data_set.num_dims());
/// assert_eq!(false,                   data_set.has_dim(DIM_NAME));
/// assert_eq!(None,                    data_set.dim_size(DIM_NAME));
/// ```
pub const NC_MAX_DIM_SIZE: usize = (std::i32::MAX - 3) as usize;

/// Maximum number of dimensions per variable
///
/// # Example
///
/// ```
/// use netcdf3::{DataSet, NC_MAX_VAR_DIMS};
///
/// const VAR_NAME: &str = "valid_var";
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// assert_eq!(0,                       data_set.num_dims());
/// assert_eq!(0,                       data_set.num_vars());
/// assert_eq!(false,                   data_set.has_var(VAR_NAME));
///
/// // Add enough dimensions
/// let mut dim_list: Vec<String> = Vec::with_capacity(NC_MAX_VAR_DIMS);
/// for i in 0..(NC_MAX_VAR_DIMS) {
///     let dim_name: String = format!("dim_{:0>4}", i);
///     data_set.add_fixed_dim(&dim_name, 1);
///     dim_list.push(dim_name);
/// }
/// assert_eq!(NC_MAX_VAR_DIMS,         data_set.num_dims());
/// assert_eq!(0,                       data_set.num_vars());
/// assert_eq!(false,                   data_set.has_var(VAR_NAME));
///
/// // Add a valid variable
/// assert_eq!(NC_MAX_VAR_DIMS,             dim_list.len());
/// data_set.add_var_i32(VAR_NAME, &dim_list).unwrap();
///
/// assert_eq!(NC_MAX_VAR_DIMS,         data_set.num_dims());
/// assert_eq!(1,                       data_set.num_vars());
/// assert_eq!(true,                    data_set.has_var(VAR_NAME));
///
/// ```
///
/// # Error: try to add a variable defined over too much dimensions
///
/// ```
/// use netcdf3::{DataSet, NC_MAX_VAR_DIMS};
/// use netcdf3::error::InvalidDataSet;
///
/// const VAR_NAME: &str = "invalid_var";
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// assert_eq!(0,                       data_set.num_dims());
/// assert_eq!(0,                       data_set.num_vars());
/// assert_eq!(false,                   data_set.has_var(VAR_NAME));
///
/// // Add enough dimensions
/// let mut too_long_dim_list: Vec<String> = Vec::with_capacity(NC_MAX_VAR_DIMS + 1);
/// for i in 0..(NC_MAX_VAR_DIMS + 1) {
///     let dim_name: String  = format!("dim_{:0>4}", i);
///     data_set.add_fixed_dim(&dim_name, 1);
///     too_long_dim_list.push(dim_name);
/// }
/// assert_eq!(NC_MAX_VAR_DIMS + 1,     data_set.num_dims());
/// assert_eq!(0,                       data_set.num_vars());
/// assert_eq!(false,                   data_set.has_var(VAR_NAME));
///
/// // Try to add a variable defined over top much dimensions
/// assert_eq!(NC_MAX_VAR_DIMS + 1,         too_long_dim_list.len());
/// assert_eq!(
///     InvalidDataSet::MaximumDimensionsPerVariableExceeded{
///         var_name: String::from(VAR_NAME),
///         num_dims: NC_MAX_VAR_DIMS + 1
///     },
///     data_set.add_var_i32(VAR_NAME, &too_long_dim_list).unwrap_err(),
/// );
/// assert_eq!(NC_MAX_VAR_DIMS + 1,     data_set.num_dims());
/// assert_eq!(0,                       data_set.num_vars());
/// assert_eq!(false,                   data_set.has_var(VAR_NAME));
/// ```
pub const NC_MAX_VAR_DIMS: usize = 1024;

/// Allows to define the NetCDF-3 data sets
///
/// # Examples
///
/// # Define a data set
///
/// ```
/// use std::rc::Rc;
/// use netcdf3::{DataSet, Dimension, DataType, InvalidDataSet};
///
/// const LATITUDE_DIM_SIZE: usize = 180;
/// const LONGITUDE_DIM_SIZE: usize = 360;
/// const TIME_DIM_SIZE: usize = 24;
/// const AIR_TEMPERATURE_VAR_LEN: usize = LATITUDE_DIM_SIZE * LONGITUDE_DIM_SIZE * TIME_DIM_SIZE;
///
/// // First create the data set
/// // -------------------------
/// let mut data_set = DataSet::new();
///
/// // Define global attributes
/// // ------------------------
/// data_set.add_global_attr_u8("title",        String::from("Air temperature measurements").into_bytes().to_vec()).unwrap();
/// data_set.add_global_attr_u8("Conventions",  String::from("CF-1.8").into_bytes().to_vec()).unwrap();
///
/// // Define dimensions
/// // -----------------
/// data_set.add_fixed_dim("latitude",          LATITUDE_DIM_SIZE).unwrap();
/// data_set.add_fixed_dim("longitude",         LONGITUDE_DIM_SIZE).unwrap();
/// data_set.set_unlimited_dim("time",          TIME_DIM_SIZE).unwrap();
///
/// // Define variables and their attributes
/// // -------------------------------------
/// // latitude
/// data_set.add_var_f32("latitude",            &["latitude"]).unwrap();
/// data_set.add_var_attr_u8("latitude",        "standard_name", String::from("latitude").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("latitude",        "long_name", String::from("LATITUDE").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("latitude",        "units", String::from("degrees_north").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("latitude",        "axis", String::from("Y").into_bytes()).unwrap();
/// // longitude
/// data_set.add_var_f32("longitude",           &["longitude"]).unwrap();
/// data_set.add_var_attr_u8("longitude",       "standard_name", String::from("longitude").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("longitude",       "long_name", String::from("LONGITUDE").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("longitude",       "units", String::from("degrees_east").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("longitude",       "axis", String::from("X").into_bytes()).unwrap();
/// // time
/// data_set.add_var_f32("time",                &["time"]).unwrap();
/// data_set.add_var_attr_u8("time",            "standard_name", String::from("time").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("time",            "long_name", String::from("TIME").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("time",            "units", String::from("hours since 1970-01-01 00:00:00").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("time",            "calendar", String::from("gregorian").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("time",            "axis", String::from("T").into_bytes()).unwrap();
/// // air_temperature
/// data_set.add_var_f64("air_temperature",     &["time", "latitude", "longitude"]).unwrap();
/// data_set.add_var_attr_u8("air_temperature", "standard_name", String::from("air_temperature").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("air_temperature", "long_name", String::from("AIR TEMPERATURE").into_bytes()).unwrap();
/// data_set.add_var_attr_u8("air_temperature", "units", String::from("Celsius").into_bytes()).unwrap();
///
/// ```
#[derive(Debug, PartialEq)]
pub struct DataSet {
    pub(crate) unlimited_dim: Option<Rc<Dimension>>,
    pub(crate) dims: Vec<Rc<Dimension>>,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vars: Vec<Variable>,
}

impl DataSet {
    // Creates an new empty NetCDF-3 dataset.
    pub fn new() -> DataSet {
        DataSet {
            unlimited_dim: None,
            dims: vec![],
            attrs: vec![],
            vars: vec![],
        }
    }

    // ----------------------------------------------------------------
    //
    //                          Dimensions
    //
    // ----------------------------------------------------------------
    /// Appends a new *fixed size* dimension in the dataset.
    ///
    /// Returns a error if an other dimension with the same name is already defined.
    pub fn add_fixed_dim<T: std::convert::AsRef<str>>(&mut self, dim_name: T, dim_size: usize) -> Result<(), InvalidDataSet> {
        let dim_name: &str = dim_name.as_ref();
        if self.dims.iter().position(|dim| *dim.name.borrow() == dim_name).is_some() {
            return Err(InvalidDataSet::DimensionAlreadyExists(dim_name.to_string()));
        }
        let new_fixed_size_dim = Rc::new(Dimension::new_fixed_size(dim_name, dim_size)?);
        self.dims.push(new_fixed_size_dim);
        return Ok(());
    }

    /// Initializes the *unlimited size* dimension of the dataset.
    ///
    /// Returns a error if :
    ///  1. the *unlimited size* is already defined
    ///  2. if an other dimension with the same name is already defined
    pub fn set_unlimited_dim<T: std::convert::AsRef<str>>(&mut self, dim_name: T, dim_size: usize) -> Result<(), InvalidDataSet> {
        let dim_name: &str = dim_name.as_ref();
        if let Some(unlimited_dim) = &self.unlimited_dim {
            return Err(InvalidDataSet::UnlimitedDimensionAlreadyExists(unlimited_dim.name()));
        }
        if self.dims.iter().position(|dim| *dim.name.borrow() == dim_name).is_some() {
            return Err(InvalidDataSet::DimensionAlreadyExists(dim_name.to_string()));
        }
        let new_unlimited_dim = Rc::new(Dimension::new_unlimited_size(dim_name, dim_size)?);
        self.dims.push(Rc::clone(&new_unlimited_dim));
        self.unlimited_dim = Some(new_unlimited_dim);
        return Ok(());
    }

    /// Returns the number of dimensions defined in the data set.
    pub fn num_dims(&self) -> usize {
        return self.dims.len();
    }

    /// Returns :
    ///
    ///  - `true` if the dimension is defined.
    ///  - `false` otherwise.
    pub fn has_dim(&self, dim_name: &str) -> bool {
        return self.find_dim_from_name(dim_name).is_some();
    }

    /// Returns a reference to the dimension.
    ///
    /// Returns `None` if the dimension is not defined.
    pub fn get_dim(&self, dim_name: &str) -> Option<Rc<Dimension>> {
        self.find_dim_from_name(dim_name)
            .map(|(_dim_index, dim): (usize, &Rc<Dimension>)| Rc::clone(dim))
    }

    /// Returns the references of all the dimensions defined in the data set.
    pub fn get_dims(&self) -> Vec<Rc<Dimension>> {
        return self.dims.iter().map(|dim: &Rc<Dimension>| Rc::clone(dim)).collect();
    }

    /// Returns the names all the dimensions defined in the data set.
    pub fn dim_names(&self) -> Vec<String> {
        self.dims.iter().map(|dim| dim.name().to_string()).collect()
    }

    /// Returns `true` if the *unlimited-size* dimension exists.
    pub fn has_unlimited_dim(&self) -> bool {
        return self.unlimited_dim.is_some();
    }

    /// Returns the *unlimited-size* dimension if it is defined, otherwise return `None`.
    ///
    /// Returns `None` if the *unlimited-size* dimension does not exist.
    pub fn get_unlimited_dim(&self) -> Option<Rc<Dimension>> {
        return self.unlimited_dim.as_ref().map(|rc_dim: &Rc<Dimension>| Rc::clone(rc_dim));
    }

    /// Returns the length of the dimension.
    ///
    /// Returns `None` if the dimension does not exist.
    pub fn dim_size(&self, dim_name: &str) -> Option<usize> {
        self.find_dim_from_name(dim_name).map(|(_dim_index, dim)| dim.size())
    }

    /// Returns the type of the dimension (*fixed-size* or *unlimited-size*).
    ///
    /// Returns `None` if the dimension does not exist.
    pub fn dim_type(&self, dim_name: &str) -> Option<DimensionType> {
        self.find_dim_from_name(dim_name).map(|(_dim_index, dim)| dim.dim_type())
    }

    /// Removes and returns the dimension.
    ///
    /// Returns an error if:
    ///
    /// - the dimension is not already defined
    /// - the dimension is yet used by a variable of the dataset
    pub fn remove_dim(&mut self, dim_name: &str) -> Result<Rc<Dimension>, InvalidDataSet> {
        let removed_dim_index: usize = match self.find_dim_from_name(dim_name) {
            None => {
                return Err(InvalidDataSet::DimensionNotDefined(dim_name.to_string()));
            },
            Some((index, _)) => index,
        };
        let mut variables_using_removed_dim: Vec<String> = vec![];
        for current_var in self.vars.iter() {
            if current_var.use_dim(dim_name) {
                variables_using_removed_dim.push(current_var.name.clone());
            }
        }
        if !variables_using_removed_dim.is_empty() {
            return Err(InvalidDataSet::DimensionYetUsed {
                var_names: variables_using_removed_dim,
                dim_name: dim_name.to_string(),
            });
        }

        let removed_dim: Rc<Dimension> = self.dims.remove(removed_dim_index);

        // Remove the *unlimited-size* dimension if necessary
        if removed_dim.is_unlimited() {
            self.unlimited_dim = None;
        }
        return Ok(removed_dim);
    }

    /// Rename the dimension or return en error if :
    /// - no dimension named `old_dim_name` already exists
    /// - an other dimension named `new_dim_name` already exists
    /// - the `new_dim_name` is not a NetCDF-3 valid name
    ///
    /// **Nothing is done if `old_dim_name` and `new_dim_name` are the same.**
    pub fn rename_dim(&mut self, old_dim_name: &str, new_dim_name: &str) -> Result<(), InvalidDataSet> {
        if old_dim_name == new_dim_name {
            // nothing is done
            return Ok(());
        }

        let (_dim_position, renamed_dim): (usize, &Rc<Dimension>) = match self.find_dim_from_name(old_dim_name) {
            None => {
                return Err(InvalidDataSet::DimensionNotDefined(old_dim_name.to_string()));
            },
            Some(rc_dim) => rc_dim,
        };

        if self.find_dim_from_name(new_dim_name).is_some() {
            return Err(InvalidDataSet::DimensionAlreadyExists(new_dim_name.to_string()));
        }

        Dimension::check_dim_name(new_dim_name)?;

        let mut dim_name: RefMut<String> = renamed_dim.name.borrow_mut();
        *dim_name = new_dim_name.to_string();
        return Ok(());
    }

    /// Find a dataset's dimension from is name.
    fn find_dim_from_name(&self, dim_name: &str) -> Option<(usize, &Rc<Dimension>)> {
        return self
            .dims
            .iter()
            .position(|dim| {
                return dim.name.borrow().deref() == dim_name;
            })
            .map(|index| {
                return (index, &self.dims[index]);
            });
    }

    pub fn get_dims_from_dim_ids(&self, dim_ids: &[usize]) -> Result<Vec<Rc<Dimension>>, InvalidDataSet> {
        let searched_dim_ids = dim_ids;
        let not_found_dim_ids: Vec<usize> = dim_ids
            .iter()
            .filter(|dim_id: &&usize| self.dims.get(**dim_id).is_none())
            .map(|i| i.clone())
            .collect();
        if !not_found_dim_ids.is_empty() {
            return Err(InvalidDataSet::DimensionIdsNotFound {
                defined: (0..self.dims.len()).collect(),
                searched: searched_dim_ids.to_vec(),
                not_found: not_found_dim_ids,
            });
        }
        Ok(dim_ids.iter().map(|dim_id: &usize| Rc::clone(&self.dims[*dim_id])).collect())
    }

    pub(crate) fn get_var_dim_ids(&self, var_name: &str) -> Option<Vec<usize>> {
        let var: &Variable = self.find_var_from_name(var_name).ok()?.1;
        let var_dims: &[Rc<Dimension>] = &var.dims;
        let var_dim_ids: Vec<usize> = var_dims.iter().map(|var_dim: &Rc<Dimension>| {
            self.dims.iter()
                .position(|data_set_dim: &Rc<Dimension>| Rc::ptr_eq(data_set_dim, var_dim))
                .expect("Shouldn't have occurred! All variable dimensions are defined in the data set, their positions should have been found.")
            // Can't panic :all dimensions
        }).collect();
        Some(var_dim_ids)
    }
    // ----------------------------------------------------------------
    //
    //                           Variables
    //
    // ----------------------------------------------------------------

    /// Add a new variable in the dataset defined over named dimensions.
    ///
    /// # Examples
    ///
    /// Add a 2D variable
    ///
    /// ```
    /// use netcdf3::{DataSet, DataType};
    ///
    /// let mut data_set = DataSet::new();
    /// let _ = data_set.add_fixed_dim("latitude", 181).unwrap();
    /// let _ = data_set.add_fixed_dim("longitude", 361).unwrap();
    /// let _ = data_set.set_unlimited_dim("time", 2).unwrap();
    ///
    /// assert_eq!(0, data_set.num_vars());
    /// let _ = data_set.add_var("sea_level_temperature", &["latitude", "longitude"], DataType::F64).unwrap();
    /// assert_eq!(1, data_set.num_vars());
    /// ```
    ///
    /// Add a scalar variable
    ///
    /// ```
    /// use netcdf3::{DataSet, DataType};
    ///
    /// const SCALAR_VAR_NAME: &str = "scalar_var";
    ///
    /// let mut data_set = DataSet::new();
    ///
    /// assert_eq!(0,                   data_set.num_vars());
    /// assert_eq!(None,                data_set.var_len(SCALAR_VAR_NAME));
    ///
    /// let _ = data_set.add_var(SCALAR_VAR_NAME, &[] as &[&str] /* no dimensions*/, DataType::I32).unwrap();
    ///
    /// assert_eq!(1,                   data_set.num_vars());
    /// assert_eq!(Some(1),             data_set.var_len(SCALAR_VAR_NAME));
    /// ```
    pub fn add_var<T: std::convert::AsRef<str>>(
        &mut self,
        var_name: &str,
        dims_name: &[T],
        data_type: DataType,
    ) -> Result<(), InvalidDataSet> {
        let var_dims: Vec<&Rc<Dimension>> = {
            let mut var_dims: Vec<&Rc<Dimension>> = vec![];
            let mut undefined_dims: Vec<String> = vec![];
            for dim_name in dims_name.iter() {
                let dim_name: &str = dim_name.as_ref();
                match self.find_dim_from_name(dim_name) {
                    None => {
                        undefined_dims.push(dim_name.to_string());
                    },
                    Some((_index, dim)) => {
                        var_dims.push(dim);
                    },
                }
            }
            if !undefined_dims.is_empty() {
                return Err(InvalidDataSet::DimensionsNotDefined {
                    var_name: var_name.to_string(),
                    undef_dim_names: undefined_dims,
                });
            }
            var_dims
        };
        if self.find_var_from_name(var_name).is_ok() {
            return Err(InvalidDataSet::VariableAlreadyExists(var_name.to_string()));
        }
        let var_dims: Vec<Rc<Dimension>> = var_dims.into_iter().map(|ref dim| Rc::clone(dim)).collect();
        self.add_var_using_dim_refs(var_name, var_dims, data_type.clone())?;
        Ok(())
    }

    pub(crate) fn add_var_using_dim_refs(
        &mut self,
        var_name: &str,
        var_dims: Vec<Rc<Dimension>>,
        data_type: DataType,
    ) -> Result<&Variable, InvalidDataSet> {
        let _ = self.vars.push(Variable::new(var_name, var_dims, data_type)?);
        Ok(self.vars.last().unwrap())
    }

    /// Add a new `i8` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i8<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I8)
    }

    /// Add a new `u8` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_u8<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::U8)
    }

    /// Add a new `i16` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i16<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I16)
    }

    /// Add a new `i32` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_i32<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::I32)
    }

    /// Add a new `f32` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_f32<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::F32)
    }

    /// Add a new `f64` type variable  defined over named dimensions (see the [add_var](struct.DataSet.html#method.add_var) method).
    pub fn add_var_f64<T: std::convert::AsRef<str>>(&mut self, var_name: &str, dims_name: &[T]) -> Result<(), InvalidDataSet> {
        self.add_var(var_name, dims_name, DataType::F64)
    }

    /// Returns the number of defined variables.
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }

    /// Returns :
    ///  - `true` if the variable is defined.
    ///  - `false` otherwise.
    pub fn has_var(&self, var_name: &str) -> bool {
        return self.find_var_from_name(var_name).is_ok();
    }

    pub fn is_record_var(&self, var_name: &str) -> Option<bool> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var.is_record_var())
            .ok();
    }

    /// Returns the length (total number of elements) of the variable.
    pub fn var_len(&self, var_name: &str) -> Option<usize> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var.len())
            .ok();
    }

    /// Returns the data type of the variable, or `None`.
    pub fn var_data_type(&self, var_name: &str) -> Option<DataType> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var.data_type())
            .ok();
    }

    /// Returns a reference to the variable, or `None`.
    pub fn get_var(&self, var_name: &str) -> Option<&Variable> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var)
            .ok();
    }

    /// Returns a mutable reference to the variable
    pub fn get_var_mut(&mut self, var_name: &str) -> Option<&mut Variable> {
        return self
            .find_var_from_name(var_name)
            .map(|(var_index, _ref_var)| var_index)
            .map(move |var_index: usize| &mut self.vars[var_index])
            .ok();
    }

    /// Returns the references all the variables defined in the dataset.
    pub fn get_vars(&self) -> Vec<&Variable> {
        return self.vars.iter().collect();
    }

    /// Returns the names all the variables defined in the dataset.
    pub fn get_var_names(&self) -> Vec<String> {
        return self.vars.iter().map(|var: &Variable| var.name().to_string()).collect();
    }

    /// Renames a variable.
    ///
    /// Nothing is do if `old_var_name` and `new_var_name` the same.
    ///
    /// Returns an error if :
    /// - no variable `old_var_name` exists
    /// - an other variable `new_var_name` already exists
    /// - `new_var_name` is a NetCDF-3 valid name
    pub fn rename_var(&mut self, old_var_name: &str, new_var_name: &str) -> Result<(), InvalidDataSet> {
        // If the names are same then nothing of done
        if old_var_name == new_var_name {
            return Ok(());
        }
        // Get the index of the renamed variable
        let renamed_var_index: usize = self.find_var_from_name(old_var_name)?.0;

        // Check that an other variable has already been defined with `new_var_name`
        if self.find_var_from_name(new_var_name).is_ok() {
            return Err(InvalidDataSet::VariableAlreadyExists(new_var_name.to_string()));
        }
        // Check the validity of the new name
        let _ = Variable::check_var_name(new_var_name)?;

        // Then rename the variable
        self.vars[renamed_var_index].name = new_var_name.to_string();

        return Ok(());
    }

    /// Remove the variable.
    pub fn remove_var(&mut self, var_name: &str) -> Result<Variable, InvalidDataSet> {
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        let removed_var: Variable = self.vars.remove(var_index);
        return Ok(removed_var);
    }

    /// Finds the dataset's variable from his name, and returns a tuple containing :
    ///
    /// - 0 : the index of the variable
    /// - 1 : a reference to the variable
    pub(crate) fn find_var_from_name(&self, var_name: &str) -> Result<(usize, &Variable), InvalidDataSet> {
        return self
            .vars
            .iter()
            .position(|var: &Variable| var.name == var_name)
            .map(|var_index| (var_index, &self.vars[var_index]))
            .ok_or(InvalidDataSet::VariableNotDefined(var_name.to_string()));
    }

    // ----------------------------------------------------------------
    //
    //                  Variable attributes
    //
    // ----------------------------------------------------------------
    // Add a `i8` attribute in the variable.
    pub fn add_var_attr_i8(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<i8>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_i8(attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `u8` attribute in the variable.
    pub fn add_var_attr_u8(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<u8>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_u8(attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `u8` attribute in the variable from a UTF-8 `String`.
    pub fn add_var_attr_string<T: AsRef<str>>(&mut self, var_name: &str, attr_name: &str, var_attr_value: T) -> Result<(), InvalidDataSet> {
        self.add_var_attr_u8(var_name, attr_name, String::from(var_attr_value.as_ref()).into_bytes())
    }

    // Add a `i16` attribute in the variable.
    pub fn add_var_attr_i16(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<i16>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_i16(attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `i32` attribute in the variable.
    pub fn add_var_attr_i32(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<i32>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_i32(attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `f32` attribute in the variable.
    pub fn add_var_attr_f32(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<f32>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_f32(attr_name, var_attr_value)?;
        Ok(())
    }

    // Add a `f64` attribute in the variable.
    pub fn add_var_attr_f64(&mut self, var_name: &str, attr_name: &str, var_attr_value: Vec<f64>) -> Result<(), InvalidDataSet> {
        // Check that the variable is defined
        let var_index: usize = self.find_var_from_name(var_name)?.0;
        // Append the new attribute
        let var: &mut Variable = &mut self.vars[var_index];
        var.add_attr_f64(attr_name, var_attr_value)?;
        Ok(())
    }

    /// Returns a reference of variable attribute.
    pub fn get_var_attr(&self, var_name: &str, attr_name: &str) -> Option<&Attribute> {
        return self
            .find_var_attr_from_name(var_name, attr_name)
            .map(|((_var_index, _var), (_attr_index, attr)): ((usize, &Variable), (usize, &Attribute))| attr)
            .ok();
    }

    /// Returns the length (number of elements) of the variable attribute.
    pub fn get_var_attr_len(&self, var_name: &str, attr_name: &str) -> Option<usize> {
        return self
            .find_var_attr_from_name(var_name, attr_name)
            .map(|((_var_index, _var), (_attr_index, attr)): ((usize, &Variable), (usize, &Attribute))| attr.len())
            .ok();
    }

    /// Returns the data type of the variable attribute.
    pub fn get_var_attr_data_type(&self, var_name: &str, attr_name: &str) -> Option<DataType> {
        return self
            .find_var_attr_from_name(var_name, attr_name)
            .map(|((_var_index, _var), (_attr_index, attr)): ((usize, &Variable), (usize, &Attribute))| attr.data_type())
            .ok();
    }

    /// Returns all attributes of a variable.
    ///
    /// Returns `None` if the variable is not defined.
    ///
    pub fn get_var_attrs(&self, var_name: &str) -> Option<Vec<&Attribute>> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, ref_var): (usize, &Variable)| ref_var)
            .ok()
            .map(|ref_var: &Variable| ref_var.get_attrs());
    }

    /// Returns :
    ///
    ///  - `true` if the variable attribute is defined.
    ///  - `false` otherwise.
    pub fn has_var_attr(&self, var_name: &str, attr_name: &str) -> Option<bool> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var.has_attr(attr_name))
            .ok();
    }

    /// Returns the number of attributes of the variable.
    ///
    /// Returns `None` if the variable does not exist.
    pub fn num_var_attrs(&self, var_name: &str) -> Option<usize> {
        return self
            .find_var_from_name(var_name)
            .map(|(_var_index, var): (usize, &Variable)| var.num_attrs())
            .ok();
    }

    /// Rename the variable attribute.
    pub fn rename_var_attr(&mut self, var_name: &str, old_attr_name: &str, new_attr_name: &str) -> Result<(), InvalidDataSet> {
        let var_index = self.find_var_from_name(var_name)?.0;
        let var: &mut Variable = &mut self.vars[var_index];
        var.rename_attr(old_attr_name, new_attr_name)?;
        Ok(())
    }

    /// Remove the attribute from the variable.
    pub fn remove_var_attr(&mut self, var_name: &str, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        let var_index = self.find_var_from_name(var_name)?.0;
        let var: &mut Variable = &mut self.vars[var_index];
        var.remove_attr(attr_name)
    }

    fn find_var_attr_from_name(
        &self,
        var_name: &str,
        attr_name: &str,
    ) -> Result<((usize, &Variable), (usize, &Attribute)), InvalidDataSet> {
        // Check that the variable is defined
        let (var_index, ref_var): (usize, &Variable) = self.find_var_from_name(var_name)?;
        let (var_attr_index, ref_var_attr): (usize, &Attribute) = ref_var.find_attr_from_name(attr_name)?;
        Ok(((var_index, ref_var), (var_attr_index, ref_var_attr)))
    }

    /// Returns the attribute value as a `&[i8]`.
    ///
    /// Also see the method [Attribute::get_i8](struct.Attribute.html#method.get_i8).
    pub fn get_var_attr_i8(&self, var_name: &str, attr_name: &str) -> Option<&[i8]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_i8()
    }

    /// Returns the attribute value as a `&[u8]`.
    ///
    /// Also see the method [Attribute::get_u8](struct.Attribute.html#method.get_u8).8))
    pub fn get_var_attr_u8(&self, var_name: &str, attr_name: &str) -> Option<&[u8]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_u8()
    }

    /// Returns the attribute value as a `String`.
    ///
    /// Also see the method [Attribute::get_as_string](struct.Attribute.html#method.get_as_string)
    pub fn get_var_attr_as_string(&self, var_name: &str, attr_name: &str) -> Option<String> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_as_string()
    }

    /// Returns the attribute value as a `&[i16]`.
    ///
    /// Also see the method [Attribute::get_i16](struct.Attribute.html#method.get_i16).
    pub fn get_var_attr_i16(&self, var_name: &str, attr_name: &str) -> Option<&[i16]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_i16()
    }

    /// Returns the attribute value as a `&[i32]`.
    ///
    /// Also see the method [Attribute::get_i32](struct.Attribute.html#method.get_i32).
    pub fn get_var_attr_i32(&self, var_name: &str, attr_name: &str) -> Option<&[i32]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_i32()
    }

    /// Returns the attribute value as a `&[f32]`.
    ///
    /// Also see the method [Attribute::get_f32](struct.Attribute.html#method.get_f32).
    pub fn get_var_attr_f32(&self, var_name: &str, attr_name: &str) -> Option<&[f32]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_f32()
    }

    /// Returns the attribute value as a `&[f64]`.
    ///
    /// Also see the method [Attribute::get_f64](struct.Attribute.html#method.get_f64
    pub fn get_var_attr_f64(&self, var_name: &str, attr_name: &str) -> Option<&[f64]> {
        let attr: &Attribute = (self.find_var_attr_from_name(var_name, attr_name).ok()?.1).1;
        attr.get_f64()
    }

    // ----------------------------------------------------------------
    //
    //                  Global attributes
    //
    // ----------------------------------------------------------------
    fn find_global_attr_from_name(&self, attr_name: &str) -> Result<(usize, &Attribute), InvalidDataSet> {
        self.attrs
            .iter()
            .position(|ref_attr: &Attribute| ref_attr.name == attr_name)
            .map(|attr_index: usize| (attr_index, &self.attrs[attr_index]))
            .ok_or(InvalidDataSet::GlobalAttributeNotDefined(attr_name.to_string()))
    }

    /// Returns a reference to the global attribute.
    pub fn get_global_attr(&self, attr_name: &str) -> Option<&Attribute> {
        self.find_global_attr_from_name(attr_name)
            .ok()
            .map(|(_attr_index, ref_attr)| ref_attr)
    }

    /// Returns a reference of all global attributes.
    pub fn get_global_attrs(&self) -> Vec<&Attribute> {
        self.attrs.iter().collect()
    }

    /// Returns the length (number of elements) of the global attribute.
    pub fn get_global_attr_len(&self, attr_name: &str) -> Option<usize> {
        self.find_global_attr_from_name(attr_name)
            .map(|(_attr_index, attr): (usize, &Attribute)| attr.len())
            .ok()
    }

    /// Returns the data type of the global attribute.
    pub fn get_global_attr_data_type(&self, attr_name: &str) -> Option<DataType> {
        self.find_global_attr_from_name(attr_name)
            .map(|(_attr_index, attr): (usize, &Attribute)| attr.data_type())
            .ok()
    }

    /// Returns the number of global attributes.
    pub fn num_global_attrs(&self) -> usize {
        self.attrs.len()
    }

    /// Returns :
    ///  - `true` if the global attribute is defined.
    ///  - `false` otherwise.
    pub fn has_global_attr(&self, attr_name: &str) -> bool {
        self.find_global_attr_from_name(attr_name).is_ok()
    }

    /// Returns the number of global attributes.
    pub fn get_global_attr_names(&self) -> Vec<String> {
        self.attrs.iter().map(|attr: &Attribute| attr.name().to_string()).collect()
    }

    /// Adds a global `i8` type attribute in the data set.
    pub fn add_global_attr_i8(&mut self, attr_name: &str, attr_data: Vec<i8>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I8(attr_data),
        });
        Ok(())
    }

    /// Adds a global `u8` type attribute in the data set.
    pub fn add_global_attr_u8(&mut self, attr_name: &str, attr_data: Vec<u8>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::U8(attr_data),
        });
        Ok(())
    }

    /// Adds a global `u8` type attribute in the data set.
    pub fn add_global_attr_string<T: AsRef<str>>(&mut self, attr_name: &str, attr_data: T) -> Result<(), InvalidDataSet> {
        self.add_global_attr_u8(attr_name, String::from(attr_data.as_ref()).into_bytes())
    }

    /// Adds a global `i16` type attribute in the data set.
    pub fn add_global_attr_i16(&mut self, attr_name: &str, attr_data: Vec<i16>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I16(attr_data),
        });
        Ok(())
    }

    /// Adds a global `i32` type attribute in the data set.
    pub fn add_global_attr_i32(&mut self, attr_name: &str, attr_data: Vec<i32>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::I32(attr_data),
        });
        Ok(())
    }

    /// Adds a global `f32` type attribute in the data set.
    pub fn add_global_attr_f32(&mut self, attr_name: &str, attr_data: Vec<f32>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::F32(attr_data),
        });
        Ok(())
    }

    /// Add a global `f64` type attribute in the data set.
    pub fn add_global_attr_f64(&mut self, attr_name: &str, attr_data: Vec<f64>) -> Result<(), InvalidDataSet> {
        if self.find_global_attr_from_name(attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(attr_name.to_string()));
        }
        let _ = Attribute::check_attr_name(attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;
        self.attrs.push(Attribute {
            name: attr_name.to_string(),
            data: DataVector::F64(attr_data),
        });
        Ok(())
    }

    pub fn rename_global_attr(&mut self, old_attr_name: &str, new_attr_name: &str) -> Result<(), InvalidDataSet> {
        // Check that both names are different
        if old_attr_name == new_attr_name {
            // nothing to do
        }

        // Check that the `old_attr_name` attribute has been defined
        let renamed_attr_index = self.find_global_attr_from_name(old_attr_name)?.0;

        // Check that the `new_attr_name` attribute has not already benn defined
        if self.find_global_attr_from_name(new_attr_name).is_ok() {
            return Err(InvalidDataSet::GlobalAttributeAlreadyExists(new_attr_name.to_string()));
        }

        // Check that the new name is a NetCDF-3 valid name
        let _ = Attribute::check_attr_name(new_attr_name)
            .map_err(|invalid_attr_name: String| InvalidDataSet::GlobalAttributeNameNotValid(invalid_attr_name))?;

        // Update the attribute name
        self.attrs[renamed_attr_index].name = new_attr_name.to_string();

        Ok(())
    }

    pub fn remove_global_attr(&mut self, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        // Check that the `attr_name` attribute has been defined
        let removed_attr_index = self.find_global_attr_from_name(attr_name)?.0;

        Ok(self.attrs.remove(removed_attr_index))
    }

    /// Returns the attribute value as a `&[i8]`.
    ///
    /// Also see the method [Attribute::get_i8](struct.Attribute.html#method.get_i8).
    pub fn get_global_attr_i8(&self, attr_name: &str) -> Option<&[i8]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_i8()
    }

    /// Returns the attribute value as a `&[u8]`.
    ///
    /// Also see the method [Attribute::get_u8](struct.Attribute.html#method.get_u8).
    pub fn get_global_attr_u8(&self, attr_name: &str) -> Option<&[u8]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_u8()
    }

    /// Returns the global attribute value as a `String`.
    ///
    /// Also see the method [Attribute::get_as_string](struct.Attribute.html#method.get_as_string)
    pub fn get_global_attr_as_string(&self, attr_name: &str) -> Option<String> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_as_string()
    }

    /// Returns the attribute value as a `&[i16]`.
    ///
    /// Also see the method [Attribute::get_i16](struct.Attribute.html#method.get_i16
    pub fn get_global_attr_i16(&self, attr_name: &str) -> Option<&[i16]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_i16()
    }

    /// Returns the attribute value as a `&[i32]`.
    ///
    /// Also see the method [Attribute::get_i32](struct.Attribute.html#method.get_i32).
    pub fn get_global_attr_i32(&self, attr_name: &str) -> Option<&[i32]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_i32()
    }

    /// Returns the attribute value as a `&[f32]`.
    ///
    /// Also see the method [Attribute::get_f32](struct.Attribute.html#method.get_f32).)
    pub fn get_global_attr_f32(&self, attr_name: &str) -> Option<&[f32]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_f32()
    }

    /// Returns the attribute value as a `&[f64]`.
    ///
    /// Also see the method [Attribute::get_f64](struct.Attribute.html#method.get_f64)
    pub fn get_global_attr_f64(&self, attr_name: &str) -> Option<&[f64]> {
        let attr: &Attribute = self.find_global_attr_from_name(attr_name).ok()?.1;
        attr.get_f64()
    }

    /// Returns the size (number of bytes) required by each record stored in the data file.
    ///
    /// Returns `None` if the data set has not a *unlimited-size* dimension.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, Variable};
    /// const UNLIM_DIM_NAME: &str = "unlim_dim";
    /// const UNLIM_DIM_SIZE: usize = 10;
    ///
    /// const FIXED_DIM_NAME: &str = "fixed_dim";
    /// const FIXED_DIM_SIZE: usize = 20;
    ///
    /// const VAR_1D_NAME: &str = "var_1D";
    /// const VAR_2D_NAME: &str = "var_2D";
    ///
    /// // No *unlimited-size* dimension is defined
    /// let mut data_set: DataSet = DataSet::new();
    /// assert_eq!(false,   data_set.has_unlimited_dim());
    /// assert_eq!(None,    data_set.record_size());
    ///
    /// // The *unlimited-size* dimension is defined here, but no variable uses it
    /// data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
    /// assert_eq!(true,                                            data_set.has_unlimited_dim());
    /// assert_eq!(Some(0),                                         data_set.record_size());
    ///
    /// // First : Add a 1D variable (a vector) over the *unlimited-size* dimension
    /// data_set.add_var_i8(VAR_1D_NAME, &[UNLIM_DIM_NAME]).unwrap();
    /// const VAR_1D_CHUNK_SIZE: usize = 4;  // 1 useful byte + 3 zero-padding bytes
    /// assert_eq!(true,                                            data_set.has_unlimited_dim());
    /// assert_eq!(Some(VAR_1D_CHUNK_SIZE),                         data_set.record_size());
    ///
    /// // Second : Add a 2D variable (a matrix) over the* unlimited-size* dimension
    /// data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
    /// data_set.add_var_i8(VAR_2D_NAME, &[UNLIM_DIM_NAME, FIXED_DIM_NAME]).unwrap();
    /// const VAR_2D_CHUNK_SIZE: usize = 20;  // 20 bytes
    ///
    /// // Then: the record size has increased
    /// assert_eq!(true,                                            data_set.has_unlimited_dim());
    /// assert_eq!(Some(VAR_1D_CHUNK_SIZE + VAR_2D_CHUNK_SIZE),     data_set.record_size());
    /// ```
    pub fn record_size(&self) -> Option<usize> {
        if !self.has_unlimited_dim() {
            return None;
        } else {
            let (record_vars, _fixed_size_vars): (Vec<&Variable>, Vec<&Variable>) =
                self.vars.iter().partition(|var: &&Variable| var.is_record_var());
            let record_size: usize = record_vars.into_iter().fold(0, |sum: usize, var: &Variable| sum + var.chunk_size());
            Some(record_size)
        }
    }

    /// Returns the number of records stored in data file.
    ///
    /// Returns `None` if the data set has not an *unlimited-size* dimension.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::DataSet;
    /// const UNLIM_DIM_NAME: &str = "unlim_dim";
    /// const UNLIM_DIM_SIZE: usize = 10;
    ///
    /// let mut data_set: DataSet = DataSet::new();
    ///
    /// // No *unlimited-size* dimension is defined
    /// assert_eq!(false,   data_set.has_unlimited_dim());
    /// assert_eq!(None,    data_set.num_records());
    ///
    /// // The *unlimited-size* dimension is defined here
    /// data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE);
    /// assert_eq!(true,                    data_set.has_unlimited_dim());
    /// assert_eq!(Some(UNLIM_DIM_SIZE),    data_set.num_records());
    /// ```
    pub fn num_records(&self) -> Option<usize> {
        match &self.unlimited_dim {
            None => None,
            Some(dim) => Some(dim.size()),
        }
    }
}
