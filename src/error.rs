pub mod parse_header_error;
pub use parse_header_error::ParseHeaderError;

use crate::{DataType, Dimension};
use std::rc::Rc;

/// NetCDF-3 data set error
///
/// This error occures when the induced [`DataSet`](struct.DataSet.html) is not a valid NetCDF-3 data set.
///
/// # Example
///
/// ```
/// use netcdf3::{
///     DataSet,
///     error::InvalidDataSet,
/// };
///
/// const VAR_NAME: &str = "var_1";
/// const DIM_NAME_1: &str = "undef_dim_1";
/// const DIM_NAME_2: &str = "undef_dim_2";
///
/// // Create a data set
/// let mut data_set = DataSet::new();
///
/// assert_eq!(0,           data_set.num_vars());
/// assert_eq!(false,       data_set.has_var(VAR_NAME));
///
/// // Try to add a new variable
/// assert_eq!(
///     InvalidDataSet::DimensionsNotDefined{
///         var_name: String::from(VAR_NAME),
///         undef_dim_names: vec![String::from(DIM_NAME_1), String::from(DIM_NAME_2)],
///     },
///     data_set.add_var_i8(VAR_NAME, &[DIM_NAME_1, DIM_NAME_2]).unwrap_err()
/// );
///
/// assert_eq!(0,           data_set.num_vars());
/// assert_eq!(false,       data_set.has_var(VAR_NAME));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidDataSet {
    DimensionAlreadyExists(String),
    DimensionNotDefined(String),
    DimensionsNotDefined {
        var_name: String,
        undef_dim_names: Vec<String>,
    },
    DimensionsUsedMultipleTimes {
        var_name: String,
        get_dim_names: Vec<String>,
    },
    UnlimitedDimensionAlreadyExists(String),
    DimensionYetUsed {
        var_names: Vec<String>,
        dim_name: String,
    },
    DimensionNameNotValid(String),
    DimensionIdsNotFound {
        defined: Vec<usize>,
        searched: Vec<usize>,
        not_found: Vec<usize>,
    },
    FixedDimensionWithZeroSize(String),
    MaximumFixedDimensionSizeExceeded {
        dim_name: String,
        get: usize,
    },
    DimensionsNotFound {
        defined: Vec<Rc<Dimension>>,
        searched: Vec<Rc<Dimension>>,
        not_found: Vec<Rc<Dimension>>,
    },

    VariableAttributeAlreadyExists {
        var_name: String,
        attr_name: String,
    },
    VariableAttributeNotDefined {
        var_name: String,
        attr_name: String,
    },
    VariableAttributeNameNotValid {
        var_name: String,
        attr_name: String,
    },

    VariableNotDefined(String),
    VariableNameNotValid(String),
    VariableAlreadyExists(String),
    VariableMismatchDataType {
        var_name: String,
        req: DataType,
        get: DataType,
    },
    VariableMismatchDataLength {
        var_name: String,
        req: usize,
        get: usize,
    },
    UnlimitedDimensionMustBeDefinedFirst {
        var_name: String,
        unlim_dim_name: String,
        get_dim_names: Vec<String>,
    },
    MaximumDimensionsPerVariableExceeded {
        var_name: String,
        num_dims: usize,
    },

    GlobalAttributeAlreadyExists(String),
    GlobalAttributeNotDefined(String),
    GlobalAttributeNameNotValid(String),
}

impl std::fmt::Display for InvalidDataSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InvalidDataSet {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    ParseHeader(ParseHeaderError),
    DataSet(InvalidDataSet),
    VariableNotDefined(String),
    VariableMismatchDataType {
        var_name: String,
        req: DataType,
        get: DataType,
    },
    IOErrorKind(std::io::ErrorKind),
    ComputationNumberOfRecords,
    RecordIndexExceeded {
        index: usize,
        num_records: usize,
    },
    Unexpected,
}

impl ReadError {
    pub fn header_is_incomplete(&self) -> bool {
        let header_is_incomlete: bool = match &self {
            ReadError::ParseHeader(parse_header_err) => parse_header_err.header_is_incomplete(),
            _ => false,
        };
        return header_is_incomlete;
    }
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReadError {}

impl std::convert::From<InvalidDataSet> for ReadError {
    fn from(err: InvalidDataSet) -> Self {
        Self::DataSet(err)
    }
}

impl std::convert::From<ParseHeaderError> for ReadError {
    fn from(err: ParseHeaderError) -> Self {
        Self::ParseHeader(err)
    }
}

impl std::convert::From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        Self::IOErrorKind(err.kind())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    IOErrorKind(std::io::ErrorKind),
    VariableNotDefined(String),
    VariableMismatchDataType {
        var_name: String,
        req: DataType,
        get: DataType,
    },
    VariableMismatchDataLength {
        var_name: String,
        req: usize,
        get: usize,
    },
    ClassicVersionNotPossible,
    HeaderAlreadyDefined,
    HeaderNotDefined,
    RecordIndexExceeded {
        index: usize,
        num_records: usize,
    },
    RecordMismatchDataLength {
        var_name: String,
        req: usize,
        get: usize,
    },
    Unexpected,
}

impl std::convert::From<std::io::Error> for WriteError {
    fn from(err: std::io::Error) -> Self {
        WriteError::IOErrorKind(err.kind())
    }
}
