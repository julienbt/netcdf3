use crate::{DataSet, InvalidDataSet, DataType, DimensionType};

#[test]
fn test_add_var_error_invalid_name() {
    const INVALID_VAR_NAME: &str = "!invalid_name";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(INVALID_VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNameNotValid(INVALID_VAR_NAME.to_string()),
        data_set.add_var_i8::<&str>(INVALID_VAR_NAME, &vec![]).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(INVALID_VAR_NAME));
}

#[test]
fn test_add_var_error_already_exists() {
    const VAR_NAME: &str = "var_1";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));


    data_set.add_var_i8::<String>(VAR_NAME, &vec![]).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(1),             data_set.var_len(VAR_NAME));
    assert_eq!(Some(DataType::I8),  data_set.var_data_type(VAR_NAME));

    // Try to a `i32`  variable with the same name
    assert_eq!(
        InvalidDataSet::VariableAlreadyExists(VAR_NAME.to_string()),
        data_set.add_var_i32::<String>(VAR_NAME, &vec![]).unwrap_err()
    );

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(1),             data_set.var_len(VAR_NAME));
    assert_eq!(Some(DataType::I8),  data_set.var_data_type(VAR_NAME));
}

#[test]
fn test_add_var_error_unlim_dim_first() {
    // If a variable is a record-variable, then the unlimited dimension must be the first dimension of this one
    const VAR_NAME: &str = "var_1";
    const FIXED_DIM_NAME: &str = "fixed_dim";
    const FIXED_DIM_SIZE: usize = 3;
    const UNLIMITED_DIM_NAME: &str = "unlimited_dim";
    const UNLIMITED_DIM_SIZE: usize = 2;
    const INVALID_DIM_LIST: [&str; 2] = [FIXED_DIM_NAME, UNLIMITED_DIM_NAME];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(None,    data_set.dim_size(FIXED_DIM_NAME));
    assert_eq!(None,    data_set.dim_type(FIXED_DIM_NAME));
    assert_eq!(false,   data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(None,    data_set.dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(None,    data_set.dim_type(UNLIMITED_DIM_NAME));

    data_set.add_fixed_dim(FIXED_DIM_NAME, FIXED_DIM_SIZE).unwrap();
    data_set.set_unlimited_dim(UNLIMITED_DIM_NAME, UNLIMITED_DIM_SIZE).unwrap();

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.var_len(VAR_NAME));
    assert_eq!(None,                                data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(Some(FIXED_DIM_SIZE),                data_set.dim_size(FIXED_DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(FIXED_DIM_NAME));
    assert_eq!(true,                                data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(Some(UNLIMITED_DIM_SIZE),            data_set.dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.dim_type(UNLIMITED_DIM_NAME));

    assert_eq!(
        InvalidDataSet::UnlimitedDimensionMustBeDefinedFirst{
            var_name: VAR_NAME.to_string(),
            unlim_dim_name: UNLIMITED_DIM_NAME.to_string(),
            get_dim_names: INVALID_DIM_LIST.iter().map(|s: &&str| String::from(*s)).collect()
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST[..]).unwrap_err()
    );

    assert_eq!(0,                                   data_set.num_vars());
    assert_eq!(false,                               data_set.has_var(VAR_NAME));
    assert_eq!(None,                                data_set.var_len(VAR_NAME));
    assert_eq!(None,                                data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                                   data_set.num_dims());
    assert_eq!(true,                                data_set.has_unlimited_dim());
    assert_eq!(true,                                data_set.has_dim(FIXED_DIM_NAME));
    assert_eq!(Some(FIXED_DIM_SIZE),                data_set.dim_size(FIXED_DIM_NAME));
    assert_eq!(Some(DimensionType::FixedSize),      data_set.dim_type(FIXED_DIM_NAME));
    assert_eq!(true,                                data_set.has_dim(UNLIMITED_DIM_NAME));
    assert_eq!(Some(UNLIMITED_DIM_SIZE),            data_set.dim_size(UNLIMITED_DIM_NAME));
    assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.dim_type(UNLIMITED_DIM_NAME));
}

#[test]
fn test_add_var_error_fixed_dim_used_multiple_times_with() {
    // If a variable is a record-variable, then the unlimited dimension must be the first dimnesion of this one
    const VAR_NAME: &str = "var_1";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_1: usize = 10;
    const DIM_SIZE_2: usize = 20;
    const INVALID_DIM_LIST: [&str; 3] = [DIM_NAME_1, DIM_NAME_2, DIM_NAME_1];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.dim_type(DIM_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.var_len(VAR_NAME));
    assert_eq!(None,                            data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_2));

    assert_eq!(
        InvalidDataSet::DimensionsUsedMultipleTimes{
            var_name: VAR_NAME.to_string(),
            get_dim_names: INVALID_DIM_LIST.iter().map(|s: &&str| String::from(*s)).collect()
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST).unwrap_err()
    );

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.var_len(VAR_NAME));
    assert_eq!(None,                            data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_2));
}

#[test]
fn test_add_var_error_undef_dim() {
    // A variable can't be defined over undefined dimensions.
    const VAR_NAME: &str = "var_1";
    const DIM_NAME_1: &str = "dim_1";
    const DIM_NAME_2: &str = "dim_2";
    const UNDEF_DIM_NAME_1: &str = "undef_dim_1";
    const UNDEF_DIM_NAME_2: &str = "undef_dim_2";
    const DIM_SIZE_1: usize = 10;
    const DIM_SIZE_2: usize = 20;
    const INVALID_DIM_LIST: [&str; 4] = [UNDEF_DIM_NAME_1, DIM_NAME_1, DIM_NAME_2, UNDEF_DIM_NAME_2];

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
    assert_eq!(0,       data_set.num_dims());
    assert_eq!(false,   data_set.has_unlimited_dim());
    assert_eq!(false,   data_set.has_dim(DIM_NAME_1));
    assert_eq!(None,    data_set.dim_size(DIM_NAME_1));
    assert_eq!(None,    data_set.dim_type(DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(DIM_NAME_2));
    assert_eq!(None,    data_set.dim_size(DIM_NAME_2));
    assert_eq!(None,    data_set.dim_type(DIM_NAME_2));
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,    data_set.dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,    data_set.dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,   data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,    data_set.dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,    data_set.dim_type(UNDEF_DIM_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.var_len(VAR_NAME));
    assert_eq!(None,                            data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_2));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.dim_type(UNDEF_DIM_NAME_2));

    assert_eq!(
        InvalidDataSet::DimensionsNotDefined{
            var_name: VAR_NAME.to_string(),
            undef_dim_names: vec![
                String::from(UNDEF_DIM_NAME_1),
                String::from(UNDEF_DIM_NAME_2),
            ]
        },
        data_set.add_var_i8(VAR_NAME, &INVALID_DIM_LIST).unwrap_err()
    );

    assert_eq!(0,                               data_set.num_vars());
    assert_eq!(false,                           data_set.has_var(VAR_NAME));
    assert_eq!(None,                            data_set.var_len(VAR_NAME));
    assert_eq!(None,                            data_set.var_data_type(VAR_NAME));
    assert_eq!(2,                               data_set.num_dims());
    assert_eq!(false,                           data_set.has_unlimited_dim());
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
    assert_eq!(Some(DIM_SIZE_1),                data_set.dim_size(DIM_NAME_1));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_1));
    assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
    assert_eq!(Some(DIM_SIZE_2),                data_set.dim_size(DIM_NAME_2));
    assert_eq!(Some(DimensionType::FixedSize),  data_set.dim_type(DIM_NAME_2));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.dim_size(UNDEF_DIM_NAME_1));
    assert_eq!(None,                            data_set.dim_type(UNDEF_DIM_NAME_1));
    assert_eq!(false,                           data_set.has_dim(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.dim_size(UNDEF_DIM_NAME_2));
    assert_eq!(None,                            data_set.dim_type(UNDEF_DIM_NAME_2));
}

#[test]
fn test_rename_var() {
    const VAR_NAME_1: &str = "var_1";
    const VAR_NAME_2: &str = "var_2";
    const DIM_NAME: &str = "dim_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME_1));
    assert_eq!(None,    data_set.var_len(VAR_NAME_1));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME_1));
    assert_eq!(false,   data_set.has_var(VAR_NAME_2));
    assert_eq!(None,    data_set.var_len(VAR_NAME_2));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME_2));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME]).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32), data_set.var_data_type(VAR_NAME_1));
    assert_eq!(false,               data_set.has_var(VAR_NAME_2));
    assert_eq!(None,                data_set.var_len(VAR_NAME_2));
    assert_eq!(None,                data_set.var_data_type(VAR_NAME_2));

    data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(false,               data_set.has_var(VAR_NAME_1));
    assert_eq!(None,                data_set.var_len(VAR_NAME_1));
    assert_eq!(None,                data_set.var_data_type(VAR_NAME_1));
    assert_eq!(true,                data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32), data_set.var_data_type(VAR_NAME_2));
}

#[test]
fn test_rename_var_error_already_exists() {
    const DIM_NAME_1: &str = "dim_1";
    const VAR_NAME_1: &str = "var_1";
    const VAR_DATA_1: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_1_LEN: usize = VAR_DATA_1.len();

    const DIM_NAME_2: &str = "dim_2";
    const VAR_NAME_2: &str = "var_2";
    const VAR_DATA_2: [i32; 3] = [5, 6, 7];
    const VAR_DATA_2_LEN: usize = VAR_DATA_2.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME_1));
    assert_eq!(None,    data_set.var_len(VAR_NAME_1));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME_1));
    assert_eq!(false,   data_set.has_var(VAR_NAME_2));
    assert_eq!(None,    data_set.var_len(VAR_NAME_2));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME_2));

    data_set.add_fixed_dim(DIM_NAME_1, VAR_DATA_1_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_1, &[DIM_NAME_1]).unwrap();
    data_set.add_fixed_dim(DIM_NAME_2, VAR_DATA_2_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME_2, &[DIM_NAME_2]).unwrap();

    assert_eq!(2,                       data_set.num_vars());
    assert_eq!(true,                    data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_1_LEN),    data_set.var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32),     data_set.var_data_type(VAR_NAME_1));
    assert_eq!(true,                    data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_2_LEN),    data_set.var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32),     data_set.var_data_type(VAR_NAME_2));

    assert_eq!(
        InvalidDataSet::VariableAlreadyExists(VAR_NAME_2.to_string()),
        data_set.rename_var(VAR_NAME_1, VAR_NAME_2).unwrap_err()
    );

    assert_eq!(2,                       data_set.num_vars());
    assert_eq!(true,                    data_set.has_var(VAR_NAME_1));
    assert_eq!(Some(VAR_DATA_1_LEN),    data_set.var_len(VAR_NAME_1));
    assert_eq!(Some(DataType::I32),     data_set.var_data_type(VAR_NAME_1));
    assert_eq!(true,                    data_set.has_var(VAR_NAME_2));
    assert_eq!(Some(VAR_DATA_2_LEN),    data_set.var_len(VAR_NAME_2));
    assert_eq!(Some(DataType::I32),     data_set.var_data_type(VAR_NAME_2));
}

#[test]
fn test_rename_var_error_invalid_name() {
    const DIM_NAME: &str = "dim_1";
    const VAR_NAME: &str = "var_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();
    const INVALID_VAR_NAME: &str = "!invalid_name";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
    assert_eq!(false,   data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_len(INVALID_VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(INVALID_VAR_NAME));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();


    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.var_data_type(VAR_NAME));
    assert_eq!(false,               data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.var_len(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.var_data_type(INVALID_VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNameNotValid(INVALID_VAR_NAME.to_string()),
        data_set.rename_var(VAR_NAME, INVALID_VAR_NAME).unwrap_err()
    );

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.var_data_type(VAR_NAME));
    assert_eq!(false,               data_set.has_var(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.var_len(INVALID_VAR_NAME));
    assert_eq!(None,                data_set.var_data_type(INVALID_VAR_NAME));
}

#[test]
fn test_remove_var() {
    const DIM_NAME: &str = "dim_1";
    const VAR_NAME: &str = "var_1";
    const VAR_DATA: [i32; 4] = [1, 2, 3, 4];
    const VAR_DATA_LEN: usize = VAR_DATA.len();

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));

    data_set.add_fixed_dim(DIM_NAME, VAR_DATA_LEN).unwrap();
    data_set.add_var_i32::<&str>(VAR_NAME, &[DIM_NAME]).unwrap();

    assert_eq!(1,                   data_set.num_vars());
    assert_eq!(true,                data_set.has_var(VAR_NAME));
    assert_eq!(Some(VAR_DATA_LEN),  data_set.var_len(VAR_NAME));
    assert_eq!(Some(DataType::I32), data_set.var_data_type(VAR_NAME));

    data_set.remove_var(VAR_NAME).unwrap();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
}


#[test]
fn test_remove_var_error_not_defined() {

    const VAR_NAME: &str = "var_1";

    let mut data_set: DataSet = DataSet::new();

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));

    assert_eq!(
        InvalidDataSet::VariableNotDefined(VAR_NAME.to_string()),
        data_set.remove_var(VAR_NAME).unwrap_err()
    );

    assert_eq!(0,       data_set.num_vars());
    assert_eq!(false,   data_set.has_var(VAR_NAME));
    assert_eq!(None,    data_set.var_len(VAR_NAME));
    assert_eq!(None,    data_set.var_data_type(VAR_NAME));
}
