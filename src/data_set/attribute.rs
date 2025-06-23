mod tests;

use crate::data_vector::DataVector;
use crate::name_string::is_valid_name;
use crate::DataType;

/// NetCDF-3 attribute
///
/// `Attribute` instances are managed by the struct [`DataSet`](struct.DataSet.html).
///
/// `DataSet`s allow to create, get, remove and rename `Attribute`s.
///
/// # Examples
///
/// ## Global attributes
///
/// ### Create and get a global attribute
///
/// ```
/// use netcdf3::{DataSet, Attribute, DataType};
///
/// const GLOBAL_ATTR_NAME: &str = "attr_1";
/// const GLOBAL_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const GLOBAL_ATTR_LEN: usize = GLOBAL_ATTR_DATA.len();
///
/// // First create the data set
/// let mut data_set = DataSet::new();
///
/// // Create a `i32` global attribute
/// data_set.add_global_attr_i32(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(1,                           data_set.num_global_attrs());
/// assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(GLOBAL_ATTR_LEN),       data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(DataType::I32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
///
/// // Get the `i32` stored values through the data set
/// assert_eq!(None,                        data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
///
/// // Or through a reference to the global attribute
/// let global_attr: &Attribute = data_set.get_global_attr(GLOBAL_ATTR_NAME).unwrap();
///
/// assert_eq!(GLOBAL_ATTR_NAME,            global_attr.name());
/// assert_eq!(GLOBAL_ATTR_LEN,             global_attr.len());
/// assert_eq!(DataType::I32,               global_attr.data_type());
///
/// assert_eq!(None,                        global_attr.get_i8());
/// assert_eq!(None,                        global_attr.get_u8());
/// assert_eq!(None,                        global_attr.get_i16());
/// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), global_attr.get_i32());
/// assert_eq!(None,                        global_attr.get_f32());
/// assert_eq!(None,                        global_attr.get_f64());
/// ```
///
/// ### Rename a global attribute
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const GLOBAL_ATTR_NAME_1: &str = "attr_1";
/// const GLOBAL_ATTR_NAME_2: &str = "attr_2";
/// const GLOBAL_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const GLOBAL_ATTR_LEN: usize = GLOBAL_ATTR_DATA.len();
///
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// // Create a `i32` variable attribute
/// data_set.add_global_attr_i32(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(1,                           data_set.num_global_attrs());
/// assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
/// assert_eq!(Some(GLOBAL_ATTR_LEN),       data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
/// assert_eq!(Some(DataType::I32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
/// assert_eq!(false,                       data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
/// assert_eq!(None,                        data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
/// assert_eq!(None,                        data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
///
/// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i32(GLOBAL_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME_2));
///
/// // Rename the global attribute
/// data_set.rename_global_attr(GLOBAL_ATTR_NAME_1, GLOBAL_ATTR_NAME_2).unwrap();
///
/// // The global attribute has been renamed
/// assert_eq!(1,                           data_set.num_global_attrs());
/// assert_eq!(false,                       data_set.has_global_attr(GLOBAL_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_global_attr_len(GLOBAL_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_1));
/// assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME_2));
/// assert_eq!(Some(GLOBAL_ATTR_LEN),       data_set.get_global_attr_len(GLOBAL_ATTR_NAME_2));
/// assert_eq!(Some(DataType::I32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME_2));
///
/// assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME_1));
/// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i32(GLOBAL_ATTR_NAME_2));
/// ```
///
/// ### Remove a global attribute
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const GLOBAL_ATTR_NAME: &str = "attr_1";
/// const GLOBAL_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();
///
///
/// // Create a data set
/// let mut data_set: DataSet = DataSet::new();
/// // Create a `i32` variable attribute
/// data_set.add_global_attr_i32(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(1,                           data_set.num_global_attrs());
/// assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(DataType::I32),         data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
/// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
///
/// // Remove the global attribute
/// data_set.remove_global_attr(GLOBAL_ATTR_NAME).unwrap();
///
/// // The global attribute has been removed
/// assert_eq!(0,                           data_set.num_global_attrs());
/// assert_eq!(false,                       data_set.has_global_attr(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
/// ```
///
/// ## Variable attributes
///
/// ### Create and get a variable attribute
///
/// ```
/// use netcdf3::{DataSet, Variable, Attribute, DataType, InvalidDataSet};
///
/// const VAR_NAME: &str = "var_1";
/// const VAR_ATTR_NAME: &str = "attr_1";
/// const VAR_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();
///
/// // Create a data set
/// let mut data_set = DataSet::new();
/// // Create a `i8` variable
/// data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();
/// // Create a `i32` variable attribute
/// data_set.add_var_attr_i32(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(Some(1),                     data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(true),                  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(VAR_ATTR_DATA_LEN),     data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(DataType::I32),         data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));
///
/// // Get the `i32` stored values through the data set
/// assert_eq!(None,                        data_set.get_var_attr_i8(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_u8(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_i16(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_f32(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_f64(VAR_NAME, VAR_ATTR_NAME));
///
/// // Or through a reference to the variable attribute
/// let var_attr: &Attribute = data_set.get_var_attr(VAR_NAME, VAR_ATTR_NAME).unwrap();
///
/// assert_eq!(VAR_ATTR_NAME,               var_attr.name());
/// assert_eq!(VAR_ATTR_DATA_LEN,           var_attr.len());
/// assert_eq!(DataType::I32,               var_attr.data_type());
///
/// assert_eq!(None,                        var_attr.get_i8());
/// assert_eq!(None,                        var_attr.get_u8());
/// assert_eq!(None,                        var_attr.get_i16());
/// assert_eq!(Some(&VAR_ATTR_DATA[..]),    var_attr.get_i32());
/// assert_eq!(None,                        var_attr.get_f32());
/// assert_eq!(None,                        var_attr.get_f64());
/// ```
///
/// ### Rename a variable attribute
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const VAR_NAME: &'static  str = "var_1";
/// const VAR_ATTR_NAME_1: &str = "attr_1";
/// const VAR_ATTR_NAME_2: &str = "attr_2";
/// const VAR_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();
///
/// // Create a data set
/// let mut data_set = DataSet::new();
/// // Create a `i8` variable
/// data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();
/// // Create a `i32` variable attribute
/// data_set.add_var_attr_i32(VAR_NAME, VAR_ATTR_NAME_1, VAR_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(Some(1),                     data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(true),                  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(Some(VAR_ATTR_DATA_LEN),     data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(Some(DataType::I32),         data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(Some(false),                 data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(None,                        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(None,                        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME_2));
///
/// // Rename the variable
/// data_set.rename_var_attr(VAR_NAME, VAR_ATTR_NAME_1, VAR_ATTR_NAME_2).unwrap();
///
/// assert_eq!(Some(1),                     data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(false),                 data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME_1));
/// assert_eq!(Some(true),                  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(Some(VAR_ATTR_DATA_LEN),     data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME_2));
/// assert_eq!(Some(DataType::I32),         data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME_2));
/// ```
///
/// ### Remove a variable attribute
///
/// ```
/// use netcdf3::{DataSet, DataType};
///
/// const VAR_NAME: &'static  str = "var_1";
/// const VAR_ATTR_NAME: &str = "attr_1";
/// const VAR_ATTR_DATA: [i32; 3] = [1, 2, 3];
/// const VAR_ATTR_DATA_LEN: usize = VAR_ATTR_DATA.len();
///
/// // Create a data set
/// let mut data_set = DataSet::new();
/// // Create a `i8` variable
/// data_set.add_var_i8::<&str>(VAR_NAME, &vec![]).unwrap();
/// // Create a `i32` variable attribute
/// data_set.add_var_attr_i32(VAR_NAME, VAR_ATTR_NAME, VAR_ATTR_DATA.to_vec()).unwrap();
///
/// assert_eq!(Some(1),                     data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(true),                  data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(VAR_ATTR_DATA_LEN),     data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(DataType::I32),         data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(Some(&VAR_ATTR_DATA[..]),    data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
///
/// // Remove the variable
/// data_set.remove_var_attr(VAR_NAME, VAR_ATTR_NAME).unwrap();
///
/// assert_eq!(Some(0),                     data_set.num_var_attrs(VAR_NAME));
/// assert_eq!(Some(false),                 data_set.has_var_attr(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_len(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_data_type(VAR_NAME, VAR_ATTR_NAME));
/// assert_eq!(None,                        data_set.get_var_attr_i32(VAR_NAME, VAR_ATTR_NAME));
/// ```
///

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub(crate) name: String,
    pub(crate) data: DataVector,
}

impl Attribute {
    /// Creates a new attribute from a `DataVector`.
    pub(crate) fn new(name: &str, data: DataVector) -> Result<Attribute, String> {
        Attribute::check_attr_name(name)?;
        Ok(Attribute {
            name: name.to_string(),
            data,
        })
    }
    /// Creates a new attribute containing i8 data.
    pub(in crate::data_set) fn new_i8(name: &str, data: Vec<i8>) -> Result<Attribute, String> {
        let data = DataVector::I8(data);
        Attribute::new(name, data)
    }

    /// Creates a new attribute containing *u8* data.
    pub(in crate::data_set) fn new_u8(name: &str, data: Vec<u8>) -> Result<Attribute, String> {
        let data = DataVector::U8(data);
        Attribute::new(name, data)
    }

    /// Create a new attribute containing *i16* data.
    pub(in crate::data_set) fn new_i16(name: &str, data: Vec<i16>) -> Result<Attribute, String> {
        let data = DataVector::I16(data);
        Attribute::new(name, data)
    }

    /// Creates a new attribute containing *i32* data.
    pub(crate) fn new_i32(name: &str, data: Vec<i32>) -> Result<Attribute, String> {
        let data = DataVector::I32(data);
        Attribute::new(name, data)
    }

    /// Creates a new attribute containing *f32* data.
    pub(crate) fn new_f32(name: &str, data: Vec<f32>) -> Result<Attribute, String> {
        let data = DataVector::F32(data);
        Attribute::new(name, data)
    }

    /// Creates a new attribute containing *f64* data.
    pub(crate) fn new_f64(name: &str, data: Vec<f64>) -> Result<Attribute, String> {
        let data = DataVector::F64(data);
        Attribute::new(name, data)
    }

    /// Returns the name of the attribute.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the NetCDF-3 data type of the attribute : *i8*, *u8*, ...
    pub fn data_type(&self) -> DataType {
        self.data.data_type()
    }

    /// Returns the number of elements (the length) of the attribute.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference of the `i8` data or `None` of the attribute has not `i8` data.
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, Attribute, DataType};
    ///
    /// const GLOBAL_ATTR_NAME: &str = "attr_1";
    /// const GLOBAL_ATTR_DATA: [i8; 3] = [1, 2, 3];
    /// const GLOBAL_ATTR_DATA_LEN: usize = GLOBAL_ATTR_DATA.len();
    ///
    /// // Create a data set and add a `i8` global attribute
    /// // -------------------------------------------------
    /// let mut data_set = DataSet::new();
    /// data_set.add_global_attr_i8(GLOBAL_ATTR_NAME, GLOBAL_ATTR_DATA.to_vec()).unwrap();
    ///
    /// // Get the stored `i8` data
    /// // ------------------------
    /// assert_eq!(true,                        data_set.has_global_attr(GLOBAL_ATTR_NAME));
    /// assert_eq!(Some(GLOBAL_ATTR_DATA_LEN),  data_set.get_global_attr_len(GLOBAL_ATTR_NAME));
    /// assert_eq!(Some(DataType::I8),          data_set.get_global_attr_data_type(GLOBAL_ATTR_NAME));
    ///
    /// // Through the data set
    /// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), data_set.get_global_attr_i8(GLOBAL_ATTR_NAME));
    /// assert_eq!(None,                        data_set.get_global_attr_u8(GLOBAL_ATTR_NAME));
    /// assert_eq!(None,                        data_set.get_global_attr_i16(GLOBAL_ATTR_NAME));
    /// assert_eq!(None,                        data_set.get_global_attr_i32(GLOBAL_ATTR_NAME));
    /// assert_eq!(None,                        data_set.get_global_attr_f32(GLOBAL_ATTR_NAME));
    /// assert_eq!(None,                        data_set.get_global_attr_f64(GLOBAL_ATTR_NAME));
    ///
    /// // Or through a reference
    /// let global_attr: &Attribute = data_set.get_global_attr(GLOBAL_ATTR_NAME).unwrap();
    ///
    /// assert_eq!(Some(&GLOBAL_ATTR_DATA[..]), global_attr.get_i8());
    /// assert_eq!(None,                        global_attr.get_u8());
    /// assert_eq!(None,                        global_attr.get_i16());
    /// assert_eq!(None,                        global_attr.get_i32());
    /// assert_eq!(None,                        global_attr.get_f32());
    /// assert_eq!(None,                        global_attr.get_f64());
    /// ```
    pub fn get_i8(&self) -> Option<&[i8]> {
        self.data.get_i8()
    }

    /// Returns a reference of the `u8` data or `None` if the attribute has not `u8` data (also see the method [get_i8](struct.Attribute.html#method.get_i8)).
    pub fn get_u8(&self) -> Option<&[u8]> {
        self.data.get_u8()
    }

    /// Returns the attribute data as a `String`.
    ///
    /// Returns `None` if the attribute is not a `u8` attribute, or of this `u8` attribute does not contain valid UTF-8 encoded bytes
    /// (also see the method [get_i8](struct.Attribute.html#method.get_u8)).
    ///
    /// # Example
    ///
    /// ```
    /// use netcdf3::{DataSet, Attribute, DataType};
    ///
    /// const UTF8_ATTR_NAME: &str = "utf8_attr";
    /// const LATIN1_ATTR_NAME: &str = "latin1_attr";
    ///
    /// let data_set: DataSet = {
    ///     let mut data_set: DataSet = DataSet::new();
    ///
    ///     let utf8_bytes: Vec<u8> = "café".as_bytes().to_vec();           // utf-8 encoding
    ///     data_set.add_global_attr_u8(UTF8_ATTR_NAME, utf8_bytes).unwrap();
    ///
    ///     let latin1_bytes: Vec<u8> = vec![b'c', b'a', b'f', b'\xe9'];    // latin-1 encoding
    ///     data_set.add_global_attr_u8(LATIN1_ATTR_NAME, latin1_bytes).unwrap();
    ///
    ///     data_set
    /// };
    ///
    /// assert_eq!(2,       data_set.num_global_attrs());
    /// assert_eq!(true,    data_set.has_global_attr(UTF8_ATTR_NAME));
    /// {
    ///     let attr: &Attribute = data_set.get_global_attr(UTF8_ATTR_NAME).unwrap();
    ///     assert_eq!(DataType::U8,                                        attr.data_type());
    ///     assert_eq!(Some(&[b'c', b'a', b'f', 0xc3, 0xa9][..]),           attr.get_u8());
    ///     assert_eq!(Some(String::from("café")),                          attr.get_as_string());
    /// }
    /// assert_eq!(true,    data_set.has_global_attr(LATIN1_ATTR_NAME));
    /// {
    ///     let attr: &Attribute = data_set.get_global_attr(LATIN1_ATTR_NAME).unwrap();
    ///     assert_eq!(DataType::U8,                                        attr.data_type());
    ///     assert_eq!(Some(&[b'c', b'a', b'f', b'\xe9'][..]),              attr.get_u8());
    ///     assert_eq!(None,                                                attr.get_as_string());
    /// }
    ///
    ///
    /// ```
    pub fn get_as_string(&self) -> Option<String> {
        self.data.get_as_string()
    }

    /// Returns a reference of the `i16` data or `None` if the attribute has not `i16` data (also see the method [get_i8](struct.Attribute.html#method.get_i8)).
    pub fn get_i16(&self) -> Option<&[i16]> {
        self.data.get_i16()
    }

    /// Returns a reference of the `i32` data or `None` if the attribute has not `i32` data (also see the method [get_i8](struct.Attribute.html#method.get_i8)).
    pub fn get_i32(&self) -> Option<&[i32]> {
        self.data.get_i32()
    }

    /// Returns a reference of the `f32` data or `None` if the attribute has not `f32` data (also see the method [get_i8](struct.Attribute.html#method.get_i8)).
    pub fn get_f32(&self) -> Option<&[f32]> {
        self.data.get_f32()
    }

    /// Returns a reference of the `f64` data or `None` if the attribute has not `f64` data (also see the method [get_i8](struct.Attribute.html#method.get_i8)).
    pub fn get_f64(&self) -> Option<&[f64]> {
        self.data.get_f64()
    }

    pub(crate) fn check_attr_name(attr_name: &str) -> Result<(), String> {
        match is_valid_name(attr_name) {
            true => Ok(()),
            false => Err(attr_name.to_string()),
        }
    }
}
