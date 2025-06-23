mod tests;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::data_set::dimension::DimensionSize;
use crate::io::compute_padding_size;
use crate::{is_valid_name, Attribute, DataType, Dimension, InvalidDataSet, NC_MAX_VAR_DIMS};

/// NetCDF-3 variable
///
/// `Variable` instances are managed by the struct [`DataSet`](struct.DataSet.html).
///
/// `DataSet`s allow to create, get, remove and rename `Variable`s.
///
/// # Examples
///
/// ## Create a variable
///
/// ```
/// use netcdf3::{DataSet, Variable, DataType, DimensionType};
///
/// const VAR_NAME: &str = "var_1";
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE_1: usize = 2;
/// const DIM_SIZE_2: usize = 3;
/// const DATA_F32: &'static [f32; 6] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// const DATA_F32_LEN: usize = DATA_F32.len();
///
/// assert_eq!(DATA_F32_LEN, DIM_SIZE_1 * DIM_SIZE_2);
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// // Define 2 dimensions
/// data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
/// data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
/// // Define a `f32` variable
/// data_set.add_var_f32(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap();
///
/// assert_eq!(true,            data_set.has_var(VAR_NAME));
/// let var: &Variable = data_set.get_var(VAR_NAME).unwrap();
/// assert_eq!(VAR_NAME,                        var.name());
/// assert_eq!(true,                            var.is_record_var());
/// assert_eq!(2,                               var.num_dims());
/// assert_eq!(0,                               var.num_attrs());
/// assert_eq!(vec![DIM_NAME_1, DIM_NAME_2],    var.dim_names());
/// assert_eq!(DATA_F32_LEN,                    var.len());
/// assert_eq!(DataType::F32,                   var.data_type());
///
/// ```
///
/// ## Rename a variable
///
/// ```
/// use netcdf3::{DataSet, DataType};
/// const VAR_NAME_1: &str = "var_1";
/// const VAR_NAME_2: &str = "var_2";
/// const DIM_NAME: &str = "dim_1";
/// const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
/// const VAR_DATA_LEN: usize = VAR_DATA.len();
///
/// // Create a data set and a variable
/// let mut data_set: DataSet = DataSet::new();
/// data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
/// data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME]).unwrap();
///
/// assert_eq!(1,                               data_set.num_vars());
/// assert_eq!(true,                            data_set.has_var(VAR_NAME_1));
/// assert_eq!(Some(VAR_DATA_LEN),              data_set.var_len(VAR_NAME_1));
/// assert_eq!(Some(DataType::I32),             data_set.var_data_type(VAR_NAME_1));
/// assert_eq!(false,                           data_set.has_var(VAR_NAME_2));
/// assert_eq!(None,                            data_set.var_len(VAR_NAME_2));
/// assert_eq!(None,                            data_set.var_data_type(VAR_NAME_2));
///
/// // Rename the variable
/// data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap();
///
/// assert_eq!(1,                               data_set.num_vars());
/// assert_eq!(false,                           data_set.has_var(VAR_NAME_1));
/// assert_eq!(None,                            data_set.var_len(VAR_NAME_1));
/// assert_eq!(None,                            data_set.var_data_type(VAR_NAME_1));
/// assert_eq!(true,                            data_set.has_var(VAR_NAME_2));
/// assert_eq!(Some(VAR_DATA_LEN),              data_set.var_len(VAR_NAME_2));
/// assert_eq!(Some(DataType::I32),             data_set.var_data_type(VAR_NAME_2));
/// ```
///
/// ## Remove a variable
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const DIM_NAME: &str = "dim_1";
/// const VAR_NAME: &str = "var_1";
/// const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
/// const VAR_DATA_LEN: usize = VAR_DATA.len();
///
/// // Create a data set and a variable
/// let mut data_set: DataSet = DataSet::new();
///
/// data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
/// data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();
///
/// assert_eq!(1,                               data_set.num_vars());
/// assert_eq!(true,                            data_set.has_var(VAR_NAME));
/// assert_eq!(Some(VAR_DATA_LEN),              data_set.var_len(VAR_NAME));
/// assert_eq!(Some(DataType::I32),             data_set.var_data_type(VAR_NAME));
///
/// // Remove the variable
/// data_set.remove_var(VAR_NAME).unwrap();
///
/// assert_eq!(0,                               data_set.num_vars());
/// assert_eq!(false,                           data_set.has_var(VAR_NAME));
/// assert_eq!(None,                            data_set.var_len(VAR_NAME));
/// assert_eq!(None,                            data_set.var_data_type(VAR_NAME));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) unlimited_dim: Option<Rc<Dimension>>,
    pub(crate) dims: Vec<Rc<Dimension>>,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) data_type: DataType,
}

impl Variable {
    pub(in crate::data_set) fn new(
        var_name: &str,
        var_dims: Vec<Rc<Dimension>>,
        data_type: DataType,
    ) -> Result<Variable, InvalidDataSet> {
        // Check if the name of the variable is a valid NetCDF-3 name.
        let _ = Variable::check_var_name(var_name)?;

        let unlimited_dim: Option<Rc<Dimension>> = match var_dims.first() {
            None => None,
            Some(ref first_dim) => match first_dim.is_unlimited() {
                false => None,
                true => Some(Rc::clone(first_dim)),
            },
        };
        Variable::check_dims_validity(var_name, &var_dims)?;

        Ok(Variable {
            name: var_name.to_string(),
            unlimited_dim: unlimited_dim,
            dims: var_dims,
            attrs: vec![],
            data_type: data_type,
            // data: None,
        })
    }

    /// Return the name of the variable.
    pub fn name(&self) -> &str {
        return &self.name;
    }

    /// Returns the data type of the variable.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, Variable, DataType};
    /// const VAR_NAME: &str = "var_1";
    ///
    /// let data_set: DataSet = {
    ///     let mut data_set = DataSet::new();
    ///     data_set.add_var_i32::<&str>(VAR_NAME, &[]).unwrap();
    ///     data_set
    /// };
    ///
    /// let var: &Variable = data_set.get_var(VAR_NAME).unwrap();
    /// assert_eq!(DataType::I32,               var.data_type());
    /// ```
    pub fn data_type(&self) -> DataType {
        return self.data_type.clone();
    }

    /// Returns the total number of elements.
    ///
    /// If the variable is a record variable then `len = num_chunks * chunk_len`.
    pub fn len(&self) -> usize {
        return self.num_chunks() * self.chunk_len();
    }

    pub fn use_dim(&self, dim_name: &str) -> bool {
        return self
            .dims
            .iter()
            .position(|dim| *dim.name.borrow() == dim_name)
            .is_some();
    }

    /// Returns the number of dimensions (the rank) the the variables
    pub fn num_dims(&self) -> usize {
        return self.dims.len();
    }

    /// Returns the list of the dimensions
    pub fn get_dims(&self) -> Vec<Rc<Dimension>> {
        self.dims.clone()
    }

    /// Returns the list of the dimension names
    pub fn dim_names(&self) -> Vec<String> {
        self.dims
            .iter()
            .map(|dim: &Rc<Dimension>| dim.name().to_string())
            .collect()
    }

    /// Returns :
    ///
    /// - `true` if the variable is defined over the *unlimited size* dimension, then has several records
    /// - `false` otherwise
    pub fn is_record_var(&self) -> bool {
        match self.dims.first() {
            None => false,
            Some(first_dim) => first_dim.is_unlimited(),
        }
    }

    /// Returns the number of attributes.
    pub fn num_attrs(&self) -> usize {
        return self.attrs.len();
    }

    /// Returns :
    ///
    /// - `true` if the variable has the attribute
    /// - `false` if not
    pub fn has_attr(&self, attr_name: &str) -> bool {
        return self.find_attr_from_name(attr_name).is_ok();
    }

    /// Returns the number of elements per chunk.
    ///
    /// If the variable id a *fixed-size* variable then `chunk_len = len`.
    pub fn chunk_len(&self) -> usize {
        let skip_len: usize = if self.is_record_var() { 1 } else { 0 };
        self.dims
            .iter()
            .skip(skip_len)
            .fold(1, |product, dim| product * dim.size())
    }

    /// Returns the size of each chunk (the number of bytes) including the padding bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, Variable};
    ///
    /// const VAR_I8_NAME: &str = "scalar_var_i8";
    /// const VAR_U8_NAME: &str = "scalar_var_u8";
    /// const VAR_I16_NAME: &str = "scalar_var_i16";
    /// const VAR_I32_NAME: &str = "scalar_var_i32";
    /// const VAR_F32_NAME: &str = "scalar_var_f32";
    /// const VAR_F64_NAME: &str = "scalar_var_f64";
    ///
    /// let data_set: DataSet = {
    ///     let mut data_set = DataSet::new();
    ///     data_set.add_var_i8::<&str>(VAR_I8_NAME, &[]).unwrap();
    ///     data_set.add_var_u8::<&str>(VAR_U8_NAME, &[]).unwrap();
    ///     data_set.add_var_i16::<&str>(VAR_I16_NAME, &[]).unwrap();
    ///     data_set.add_var_i32::<&str>(VAR_I32_NAME, &[]).unwrap();
    ///     data_set.add_var_f32::<&str>(VAR_F32_NAME, &[]).unwrap();
    ///     data_set.add_var_f64::<&str>(VAR_F64_NAME, &[]).unwrap();
    ///     data_set
    /// };
    ///
    /// let scalar_var_i8: &Variable = data_set.get_var(VAR_I8_NAME).unwrap();
    /// let scalar_var_u8: &Variable = data_set.get_var(VAR_U8_NAME).unwrap();
    /// let scalar_var_i16: &Variable = data_set.get_var(VAR_I16_NAME).unwrap();
    /// let scalar_var_i32: &Variable = data_set.get_var(VAR_I32_NAME).unwrap();
    /// let scalar_var_f32: &Variable = data_set.get_var(VAR_F32_NAME).unwrap();
    /// let scalar_var_f64: &Variable = data_set.get_var(VAR_F64_NAME).unwrap();
    ///
    /// assert_eq!(4,           scalar_var_i8.chunk_size());
    /// assert_eq!(4,           scalar_var_u8.chunk_size());
    /// assert_eq!(4,           scalar_var_i16.chunk_size());
    /// assert_eq!(4,           scalar_var_i32.chunk_size());
    /// assert_eq!(4,           scalar_var_f32.chunk_size());
    /// assert_eq!(8,           scalar_var_f64.chunk_size());
    /// ```
    pub fn chunk_size(&self) -> usize {
        let mut chunk_size = self.chunk_len() * self.data_type.size_of();
        // append the bytes of the zero padding, if necessary
        chunk_size += compute_padding_size(chunk_size);
        return chunk_size;
    }

    /// Returns the number of chunks.
    pub fn num_chunks(&self) -> usize {
        match self.dims.first() {
            None => 1, // Case : a scalar *fixed-size* variable
            Some(first_dim) => match &first_dim.size {
                DimensionSize::Fixed(_) => 1,
                DimensionSize::Unlimited(size) => *size.borrow(),
            },
        }
    }

    /// Returns all attributs defined in the dataset or in the variable.
    pub fn get_attrs(&self) -> Vec<&Attribute> {
        return self.attrs.iter().collect();
    }

    /// Returns all attributs defined in the dataset or in the variable.
    pub fn get_attr_names(&self) -> Vec<String> {
        return self
            .attrs
            .iter()
            .map(|attr: &Attribute| attr.name().to_string())
            .collect();
    }

    /// Returns a reference counter to the named attribute, return an error if
    /// the attribute is not already defined.
    pub fn get_attr(&self, attr_name: &str) -> Option<&Attribute> {
        return self
            .find_attr_from_name(attr_name)
            .map(|result: (usize, &Attribute)| result.1)
            .ok();
    }

    /// Returns the attribute value as a `&[i8]`.
    ///
    /// Also see the method [Attribute::get_i8](struct.Attribute.html#method.get_i8).
    pub fn get_attr_i8(&self, attr_name: &str) -> Option<&[i8]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_i8()
    }

    /// Returns the attribute value as a `&[u8]`.
    ///
    /// Also see the method [Attribute::get_u8](struct.Attribute.html#method.get_u8).
    pub fn get_attr_u8(&self, attr_name: &str) -> Option<&[u8]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_u8()
    }

    /// Returns the attribute value as a `String`.
    ///
    /// Also see the method [Attribute::get_as_string](struct.Attribute.html#method.get_as_string).
    pub fn get_attr_as_string(&self, attr_name: &str) -> Option<String> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_as_string()
    }

    /// Returns the attribute value as a `&[i16]`.
    ///
    /// Also see the method [Attribute::get_i16](struct.Attribute.html#method.get_i16).
    pub fn get_attr_i16(&self, attr_name: &str) -> Option<&[i16]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_i16()
    }

    /// Returns the attribute value as a `&[i32]`.
    ///
    /// Also see the method [Attribute::get_i32](struct.Attribute.html#method.get_i32).
    pub fn get_attr_i32(&self, attr_name: &str) -> Option<&[i32]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_i32()
    }

    /// Returns the attribute value as a `&[f32]`.
    ///
    /// Also see the method [Attribute::get_f32](struct.Attribute.html#method.get_f32).
    pub fn get_attr_f32(&self, attr_name: &str) -> Option<&[f32]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_f32()
    }

    /// Returns the attribute value as a `&[f64]`.
    ///
    /// Also see the method [Attribute::get_f64](struct.Attribute.html#method.get_f64).
    pub fn get_attr_f64(&self, attr_name: &str) -> Option<&[f64]> {
        let attr: &Attribute = self.get_attr(attr_name)?;
        attr.get_f64()
    }

    /// Appends a new attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    fn add_attr(&mut self, new_attr: Attribute) -> Result<(), InvalidDataSet> {
        // Check if an other same name attribute already exists.
        if self.find_attr_from_name(&new_attr.name).is_ok() {
            return Err(InvalidDataSet::VariableAttributeAlreadyExists {
                var_name: self.name.to_string(),
                attr_name: new_attr.name.to_string(),
            });
        }
        // append the new attribute
        self.attrs.push(new_attr);
        return Ok(());
    }

    /// Append a new `i8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_i8(&mut self, attr_name: &str, i8_data: Vec<i8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_i8(attr_name, i8_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `u8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_u8(&mut self, attr_name: &str, u8_data: Vec<u8>) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_u8(attr_name, u8_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `u8` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_string<T: AsRef<str>>(
        &mut self,
        attr_name: &str,
        str_data: T,
    ) -> Result<(), InvalidDataSet> {
        self.add_attr_u8(attr_name, String::from(str_data.as_ref()).into_bytes())
    }

    /// Append a new `i16` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_i16(
        &mut self,
        attr_name: &str,
        i16_data: Vec<i16>,
    ) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_i16(attr_name, i16_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `i32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_i32(
        &mut self,
        attr_name: &str,
        i32_data: Vec<i32>,
    ) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_i32(attr_name, i32_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f32` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_f32(
        &mut self,
        attr_name: &str,
        f32_data: Vec<f32>,
    ) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_f32(attr_name, f32_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Append a new `f64` attribute.
    ///
    /// An error is returned if an other attribute with the same name has already been added.
    pub fn add_attr_f64(
        &mut self,
        attr_name: &str,
        f64_data: Vec<f64>,
    ) -> Result<(), InvalidDataSet> {
        let attr: Attribute =
            Attribute::new_f64(attr_name, f64_data).map_err(|var_attr_name: String| {
                InvalidDataSet::VariableAttributeNameNotValid {
                    var_name: self.name.to_string(),
                    attr_name: var_attr_name,
                }
            })?;
        self.add_attr(attr)?;
        Ok(())
    }

    /// Rename an existing attribute.
    ///
    /// An error is returned :
    ///  - the `old_attr_name`is not a valid NetCDF-3 name
    ///  - the `old_attr_name` attribute doesn't exist
    ///  - an other `new_attr_name` attribute already exist
    pub(in crate::data_set) fn rename_attr(
        &mut self,
        old_attr_name: &str,
        new_attr_name: &str,
    ) -> Result<(), InvalidDataSet> {
        if old_attr_name == new_attr_name {
            return Ok(());
        }
        // Check if the `old_attr_name` attribute exists
        let renamed_attr_index: usize = self.find_attr_from_name(old_attr_name)?.0;
        // Check if an other `new_attr_name` attribute already exist
        if self.find_attr_from_name(new_attr_name).is_ok() {
            return Err(InvalidDataSet::VariableAttributeAlreadyExists {
                var_name: self.name.to_string(),
                attr_name: new_attr_name.to_string(),
            });
        }

        // Check that `new_attr_name`is a valid NetCDF-3 name
        Attribute::check_attr_name(new_attr_name).map_err(|var_attr_name: String| {
            InvalidDataSet::VariableAttributeNameNotValid {
                var_name: self.name.to_string(),
                attr_name: var_attr_name.to_string(),
            }
        })?;
        let renamed_attr: &mut Attribute = &mut self.attrs[renamed_attr_index];
        renamed_attr.name = new_attr_name.to_string();
        return Ok(());
    }

    // Remove the attribute.
    pub fn remove_attr(&mut self, attr_name: &str) -> Result<Attribute, InvalidDataSet> {
        let removed_attr_index: usize = self.find_attr_from_name(attr_name)?.0;
        let removed_attr: Attribute = self.attrs.remove(removed_attr_index);
        return Ok(removed_attr);
    }

    /// Find a dataset's attribute from is name.
    pub(in crate::data_set) fn find_attr_from_name(
        &self,
        attr_name: &str,
    ) -> Result<(usize, &Attribute), InvalidDataSet> {
        self.attrs
            .iter()
            .position(|attr| {
                // First find the position
                attr.name() == attr_name
            })
            .map(|index| {
                // Then get the referance to the attribute
                return (index, &self.attrs[index]);
            })
            .ok_or(InvalidDataSet::VariableAttributeNotDefined {
                var_name: self.name.to_string(),
                attr_name: attr_name.to_string(),
            })
    }

    pub(super) fn check_var_name(var_name: &str) -> Result<(), InvalidDataSet> {
        return match is_valid_name(var_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::VariableNameNotValid(var_name.to_string())),
        };
    }

    fn check_dims_validity(
        var_name: &str,
        dims: &Vec<Rc<Dimension>>,
    ) -> Result<(), InvalidDataSet> {
        if dims.is_empty() {
            return Ok(());
        }
        // Check that the optional unlimited dimension is defined at first
        if let Some(unlim_dim) = dims
            .iter()
            .skip(1)
            .find(|dim: &&Rc<Dimension>| dim.is_unlimited())
        {
            let dim_names: Vec<String> =
                dims.iter().map(|dim: &Rc<Dimension>| dim.name()).collect();
            return Err(InvalidDataSet::UnlimitedDimensionMustBeDefinedFirst {
                var_name: var_name.to_string(),
                unlim_dim_name: unlim_dim.name(),
                get_dim_names: dim_names,
            });
        }
        // Check that the same dimension is not used multiple times by the variable
        let mut repeated_dim_names: Vec<String> = vec![];
        for (i, ref_dim_1) in dims.iter().enumerate().skip(1) {
            let i32ernal_repeated_dim_names: Vec<String> = dims
                .iter()
                .take(i)
                .filter(|ref_dim_2: &&Rc<Dimension>| Rc::ptr_eq(ref_dim_1, ref_dim_2))
                .map(|ref_dim_2: &Rc<Dimension>| ref_dim_2.name())
                .collect();
            repeated_dim_names.extend(i32ernal_repeated_dim_names.into_iter());
        }
        let repeated_dim_names = HashSet::<String>::from_iter(repeated_dim_names.into_iter());
        if !repeated_dim_names.is_empty() {
            let dim_names: Vec<String> =
                dims.iter().map(|dim: &Rc<Dimension>| dim.name()).collect();
            return Err(InvalidDataSet::DimensionsUsedMultipleTimes {
                var_name: var_name.to_string(),
                get_dim_names: dim_names,
            });
        }
        if dims.len() > NC_MAX_VAR_DIMS {
            return Err(InvalidDataSet::MaximumDimensionsPerVariableExceeded {
                var_name: var_name.to_string(),
                num_dims: dims.len(),
            });
        }
        Ok(())
    }
}
