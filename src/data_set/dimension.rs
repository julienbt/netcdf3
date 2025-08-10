mod tests;

use crate::name_string::is_valid_name;
use crate::InvalidDataSet;
use crate::NC_MAX_DIM_SIZE;

use std::cell::RefCell;

/// NetCDF-3 dimension
///
/// `Dimension` instances are managed by the struct [`DataSet`](struct.DataSet.html).
///
/// `DataSet`s allow to create, get, remove and rename `Dimension`s.
///
/// # Examples
///
/// ## Create and get *fixed-size* and *unlimited-size* dimensions
///
/// ```
/// use std::rc::Rc;
/// use netcdf3::{DataSet, Dimension, DimensionType};
///
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_SIZE_1: usize = 10;
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE_2: usize = 20;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Add one *fixed-size* dimensions and set the *unlimited-size* dimension
/// data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
/// data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
///
/// // Read values throught the data set
/// assert_eq!(2,                                   data_set.num_dims());
/// assert_eq!(true,                                data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
/// assert_eq!(Some(DIM_SIZE_1),                    data_set.dim_size(DIM_NAME_1));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.dim_type(DIM_NAME_1));
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
/// assert_eq!(Some(DIM_SIZE_2),                    data_set.dim_size(DIM_NAME_2));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(DIM_NAME_2));
///
/// // Or through references of the dimensions
/// let dim_1: Rc<Dimension> = data_set.get_dim(DIM_NAME_1).unwrap();
/// assert_eq!(DIM_NAME_1,                          dim_1.name());
/// assert_eq!(DIM_SIZE_1,                          dim_1.size());
/// assert_eq!(true,                                dim_1.is_unlimited());
/// assert_eq!(false,                               dim_1.is_fixed());
/// assert_eq!(DimensionType::UnlimitedSize,        dim_1.dim_type());
///
/// let dim_2: Rc<Dimension> = data_set.get_dim(DIM_NAME_2).unwrap();
/// assert_eq!(DIM_NAME_2,                          dim_2.name());
/// assert_eq!(DIM_SIZE_2,                          dim_2.size());
/// assert_eq!(false,                               dim_2.is_unlimited());
/// assert_eq!(true,                                dim_2.is_fixed());
/// assert_eq!(DimensionType::FixedSize,            dim_2.dim_type());
///
/// ```
///
/// ## Rename a dimension
///
/// ```
/// use netcdf3::{DataSet, DimensionType};
///
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE: usize = 10;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Add a *fixed-size* dimension
/// data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE).unwrap();
///
/// assert_eq!(1,                                   data_set.num_dims());
/// assert_eq!(false,                               data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
/// assert_eq!(Some(DIM_SIZE),                      data_set.dim_size(DIM_NAME_1));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(DIM_NAME_1));
/// assert_eq!(false,                               data_set.has_dim(DIM_NAME_2));
/// assert_eq!(None,                                data_set.dim_size(DIM_NAME_2));
/// assert_eq!(None,                                data_set.dim_type(DIM_NAME_2));
///
/// // Rename the *fixed-size* dimension
/// data_set.rename_dim(DIM_NAME_1, DIM_NAME_2).unwrap();
///
/// assert_eq!(1,                                   data_set.num_dims());
/// assert_eq!(false,                               data_set.has_unlimited_dim());
/// assert_eq!(false,                               data_set.has_dim(DIM_NAME_1));
/// assert_eq!(None,                                data_set.dim_size(DIM_NAME_1));
/// assert_eq!(None,                                data_set.dim_type(DIM_NAME_1));
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
/// assert_eq!(Some(DIM_SIZE),                      data_set.dim_size(DIM_NAME_2));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(DIM_NAME_2));
/// ```
///
/// ## Remove a dimension
///
/// ```
/// use std::rc::Rc;
/// use netcdf3::{DataSet, Dimension, DimensionType};
///
/// const DIM_NAME: &str = "dim_1";
/// const DIM_SIZE: usize = 10;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Set the *unlimited-size* dimension
/// data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE).unwrap();
///
/// assert_eq!(1,                                   data_set.num_dims());
/// assert_eq!(true,                                data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME));
/// assert_eq!(Some(DIM_SIZE),                      data_set.dim_size(DIM_NAME));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.dim_type(DIM_NAME));
///
/// // Remove the *unlimited-size* dimension
/// let _removed_dim: Rc<Dimension> = data_set.remove_dim(DIM_NAME).unwrap();
///
/// assert_eq!(0,                                   data_set.num_dims());
/// assert_eq!(false,                               data_set.has_unlimited_dim());
/// assert_eq!(false,                               data_set.has_dim(DIM_NAME));
/// assert_eq!(None,                                data_set.dim_size(DIM_NAME));
/// assert_eq!(None,                                data_set.dim_type(DIM_NAME));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimension {
    pub(crate) name: RefCell<String>,
    pub(crate) size: DimensionSize,
}

/// Internal representation of the size of a dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DimensionSize {
    /// *Unlimited-size* dimension, the unlimited size can be modifed by the NetCDF-3 dataset.
    Unlimited(RefCell<usize>),
    /// *Fixed-size* dimension
    Fixed(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
/// Type of a dimension, *fixed* or *unlimited* size
pub enum DimensionType {
    UnlimitedSize = 0,
    FixedSize = 1,
}

impl DimensionSize {
    /// Create a new *unlimited* or *fixed* size.
    pub(in crate::data_set) fn new(size: usize, r#type: DimensionType) -> DimensionSize {
        match r#type {
            DimensionType::FixedSize => DimensionSize::Fixed(size),
            DimensionType::UnlimitedSize => DimensionSize::Unlimited(RefCell::new(size)),
        }
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn size(&self) -> usize {
        match self {
            DimensionSize::Unlimited(size) => *size.borrow(),
            DimensionSize::Fixed(size) => *size,
        }
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn r#type(&self) -> DimensionType {
        match self {
            DimensionSize::Unlimited(_) => DimensionType::UnlimitedSize,
            DimensionSize::Fixed(_) => DimensionType::FixedSize,
        }
    }
}

impl Dimension {
    /// Creates a new *fixed size* NetCDF-3 dimension.
    pub(crate) fn new_fixed_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        if size == 0 {
            return Err(InvalidDataSet::FixedDimensionWithZeroSize(name.to_string()));
        }
        if size > NC_MAX_DIM_SIZE {
            return Err(InvalidDataSet::MaximumFixedDimensionSizeExceeded {
                dim_name: name.to_string(),
                get: size,
            });
        }
        Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::FixedSize),
        })
    }

    /// Creates a new *unlimited size* NetCDF-3 dimension.
    pub(crate) fn new_unlimited_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::UnlimitedSize),
        })
    }

    /// Returns the name of the NetCDF-3 dimension.
    pub fn name(&self) -> String {
        self.name.borrow().clone()
    }

    /// Returns the size of the NetCDF-3 dimension.
    pub fn size(&self) -> usize {
        self.size.size()
    }

    /// Returns the dimension type (*fixed size* ou *unlimited size*) of the NetCDF-3 dimension.
    pub fn dim_type(&self) -> DimensionType {
        self.size.r#type()
    }

    /// Returns `true` if the dimension is a *unlimited size* dimension, otherwise return `false`.
    pub fn is_unlimited(&self) -> bool {
        self.dim_type() == DimensionType::UnlimitedSize
    }

    /// Returns `true` if the dimension is a *fixed size* dimension, otherwise return `false`.
    pub fn is_fixed(&self) -> bool {
        self.dim_type() == DimensionType::FixedSize
    }

    pub(in crate::data_set) fn check_dim_name(dim_name: &str) -> Result<(), InvalidDataSet> {
        match is_valid_name(dim_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::DimensionNameNotValid(dim_name.to_string())),
        }
    }
}
