#![cfg(test)]

use crate::{DataSet, Variable};
#[test]
fn test_var_chunk_size() {
    const DIM_NAME_1: &str = "dim_1";
    const DIM_SIZE_1: usize = 1;
    const DIM_NAME_2: &str = "dim_2";
    const DIM_SIZE_2: usize = 2;
    const DIM_NAME_4: &str = "dim_4";
    const DIM_SIZE_4: usize = 4;
    const DIM_NAME_5: &str = "dim_5";
    const DIM_SIZE_5: usize = 5;

    const VAR_I8_1_NAME: &str = "var_i8_1";
    const VAR_I8_4_NAME: &str = "var_i8_4";
    const VAR_I8_5_NAME: &str = "var_i8_5";
    const VAR_I16_1_NAME: &str = "var_i16_1";
    const VAR_I16_2_NAME: &str = "var_i16_2";
    const VAR_I32_1_NAME: &str = "var_i32_1";

    let data_set: DataSet = {
        let mut data_set: DataSet = DataSet::new();
        data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
        data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
        data_set.add_fixed_dim(DIM_NAME_4, DIM_SIZE_4).unwrap();
        data_set.add_fixed_dim(DIM_NAME_5, DIM_SIZE_5).unwrap();

        data_set.add_var_i8(VAR_I8_1_NAME, &[DIM_NAME_1]).unwrap();
        data_set.add_var_i8(VAR_I8_4_NAME, &[DIM_NAME_4]).unwrap();
        data_set.add_var_i8(VAR_I8_5_NAME, &[DIM_NAME_5]).unwrap();
        data_set.add_var_i16(VAR_I16_1_NAME, &[DIM_NAME_1]).unwrap();
        data_set.add_var_i16(VAR_I16_2_NAME, &[DIM_NAME_2]).unwrap();
        data_set.add_var_i32(VAR_I32_1_NAME, &[DIM_NAME_1]).unwrap();

        data_set
    };

    assert_eq!(true, data_set.has_var(VAR_I8_1_NAME));
    assert_eq!(true, data_set.has_var(VAR_I8_4_NAME));
    assert_eq!(true, data_set.has_var(VAR_I8_5_NAME));
    assert_eq!(true, data_set.has_var(VAR_I16_1_NAME));
    assert_eq!(true, data_set.has_var(VAR_I16_2_NAME));
    assert_eq!(true, data_set.has_var(VAR_I32_1_NAME));

    assert_eq!(4, data_set.get_var(VAR_I8_1_NAME).unwrap().chunk_size());
    assert_eq!(4, data_set.get_var(VAR_I8_4_NAME).unwrap().chunk_size());
    assert_eq!(8, data_set.get_var(VAR_I8_5_NAME).unwrap().chunk_size());
    assert_eq!(4, data_set.get_var(VAR_I16_1_NAME).unwrap().chunk_size());
    assert_eq!(4, data_set.get_var(VAR_I16_2_NAME).unwrap().chunk_size());
    assert_eq!(4, data_set.get_var(VAR_I32_1_NAME).unwrap().chunk_size());
}

#[test]
fn test_var_len_per_chunk() {
    const UNLIM_DIM_NAME: &str = "unlim_dim";
    const UNLIM_DIM_SIZE: usize = 30;

    const FIXED_DIM_NAME_1: &str = "fixed_dim_1";
    const FIXED_DIM_SIZE_1: usize = 10;
    const FIXED_DIM_NAME_2: &str = "fixed_dim_2";
    const FIXED_DIM_SIZE_2: usize = 20;

    const VAR_NAME_1: &str = "var_1";
    const VAR_NAME_2: &str = "var_2";

    let data_set: DataSet = {
        let mut data_set: DataSet = DataSet::new();
        data_set.set_unlimited_dim(UNLIM_DIM_NAME, UNLIM_DIM_SIZE).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME_1, FIXED_DIM_SIZE_1).unwrap();
        data_set.add_fixed_dim(FIXED_DIM_NAME_2, FIXED_DIM_SIZE_2).unwrap();

        data_set
            .add_var_i8(VAR_NAME_1, &[UNLIM_DIM_NAME, FIXED_DIM_NAME_1, FIXED_DIM_NAME_2])
            .unwrap();
        data_set.add_var_i8(VAR_NAME_2, &[FIXED_DIM_NAME_1, FIXED_DIM_NAME_2]).unwrap();
        data_set
    };

    assert_eq!(true, data_set.has_var(VAR_NAME_1));
    let var_1: &Variable = data_set.get_var(VAR_NAME_1).unwrap();
    assert_eq!(UNLIM_DIM_SIZE * FIXED_DIM_SIZE_1 * FIXED_DIM_SIZE_2, var_1.len());
    assert_eq!(true, var_1.is_record_var());
    assert_eq!(FIXED_DIM_SIZE_1 * FIXED_DIM_SIZE_2, var_1.chunk_len());

    assert_eq!(true, data_set.has_var(VAR_NAME_2));
    let var_2: &Variable = data_set.get_var(VAR_NAME_2).unwrap();
    assert_eq!(FIXED_DIM_SIZE_1 * FIXED_DIM_SIZE_2, var_2.len());
    assert_eq!(false, var_2.is_record_var());
    assert_eq!(FIXED_DIM_SIZE_1 * FIXED_DIM_SIZE_2, var_2.chunk_len());
}
