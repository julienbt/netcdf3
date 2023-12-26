#![cfg(test)]

use super::DataVector;
use crate::DataType;

#[test]
fn test_get_data_type() {
    assert_eq!(DataType::I8, DataVector::I8(vec![]).data_type());
    assert_eq!(DataType::U8, DataVector::U8(vec![]).data_type());
    assert_eq!(DataType::I16, DataVector::I16(vec![]).data_type());
    assert_eq!(DataType::I32, DataVector::I32(vec![]).data_type());
    assert_eq!(DataType::F32, DataVector::F32(vec![]).data_type());
    assert_eq!(DataType::F64, DataVector::F64(vec![]).data_type());
}

#[test]
fn test_get_len() {
    assert_eq!(0, DataVector::I8(vec![]).len());
    assert_eq!(0, DataVector::U8(vec![]).len());
    assert_eq!(0, DataVector::I16(vec![]).len());
    assert_eq!(0, DataVector::I32(vec![]).len());
    assert_eq!(0, DataVector::F32(vec![]).len());
    assert_eq!(0, DataVector::F64(vec![]).len());

    assert_eq!(3, DataVector::I8(vec![1, 2, 3]).len());
    assert_eq!(3, DataVector::U8(vec![1, 2, 3]).len());
    assert_eq!(3, DataVector::I16(vec![1, 2, 3]).len());
    assert_eq!(3, DataVector::I32(vec![1, 2, 3]).len());
    assert_eq!(3, DataVector::F32(vec![1.0, 2.0, 3.0]).len());
    assert_eq!(3, DataVector::F64(vec![1.0, 2.0, 3.0]).len());
}

#[test]
fn test_get_i8() {
    assert_eq!(Some(&[1, 2, 3][..]), DataVector::I8(vec![1, 2, 3]).get_i8());
    assert_eq!(None, DataVector::U8(vec![1, 2, 3]).get_i8());
    assert_eq!(None, DataVector::I16(vec![1, 2, 3]).get_i8());
    assert_eq!(None, DataVector::I32(vec![1, 2, 3]).get_i8());
    assert_eq!(None, DataVector::F32(vec![1.0, 2.0, 3.0]).get_i8());
    assert_eq!(None, DataVector::F64(vec![1.0, 2.0, 3.0]).get_i8());
}

#[test]
fn test_get_u8() {
    assert_eq!(None, DataVector::I8(vec![1, 2, 3]).get_u8());
    assert_eq!(Some(&[1, 2, 3][..]), DataVector::U8(vec![1, 2, 3]).get_u8());
    assert_eq!(None, DataVector::I16(vec![1, 2, 3]).get_u8());
    assert_eq!(None, DataVector::I32(vec![1, 2, 3]).get_u8());
    assert_eq!(None, DataVector::F32(vec![1.0, 2.0, 3.0]).get_u8());
    assert_eq!(None, DataVector::F64(vec![1.0, 2.0, 3.0]).get_u8());
}

#[test]
fn test_get_i16() {
    assert_eq!(None, DataVector::I8(vec![1, 2, 3]).get_i16());
    assert_eq!(None, DataVector::U8(vec![1, 2, 3]).get_i16());
    assert_eq!(Some(&[1, 2, 3][..]), DataVector::I16(vec![1, 2, 3]).get_i16());
    assert_eq!(None, DataVector::I32(vec![1, 2, 3]).get_i16());
    assert_eq!(None, DataVector::F32(vec![1.0, 2.0, 3.0]).get_i16());
    assert_eq!(None, DataVector::F64(vec![1.0, 2.0, 3.0]).get_i16());
}

#[test]
fn test_get_i32() {
    assert_eq!(None, DataVector::I8(vec![1, 2, 3]).get_i32());
    assert_eq!(None, DataVector::U8(vec![1, 2, 3]).get_i32());
    assert_eq!(None, DataVector::I16(vec![1, 2, 3]).get_i32());
    assert_eq!(Some(&[1, 2, 3][..]), DataVector::I32(vec![1, 2, 3]).get_i32());
    assert_eq!(None, DataVector::F32(vec![1.0, 2.0, 3.0]).get_i32());
    assert_eq!(None, DataVector::F64(vec![1.0, 2.0, 3.0]).get_i32());
}

#[test]
fn test_get_f32() {
    assert_eq!(None, DataVector::I8(vec![1, 2, 3]).get_f32());
    assert_eq!(None, DataVector::U8(vec![1, 2, 3]).get_f32());
    assert_eq!(None, DataVector::I16(vec![1, 2, 3]).get_f32());
    assert_eq!(None, DataVector::I32(vec![1, 2, 3]).get_f32());
    assert_eq!(Some(&[1.0, 2.0, 3.0][..]), DataVector::F32(vec![1.0, 2.0, 3.0]).get_f32());
    assert_eq!(None, DataVector::F64(vec![1.0, 2.0, 3.0]).get_f32());
}

#[test]
fn test_get_f64() {
    assert_eq!(None, DataVector::I8(vec![1, 2, 3]).get_f64());
    assert_eq!(None, DataVector::U8(vec![1, 2, 3]).get_f64());
    assert_eq!(None, DataVector::I16(vec![1, 2, 3]).get_f64());
    assert_eq!(None, DataVector::I32(vec![1, 2, 3]).get_f64());
    assert_eq!(None, DataVector::F32(vec![1.0, 2.0, 3.0]).get_f64());
    assert_eq!(Some(&[1.0, 2.0, 3.0][..]), DataVector::F64(vec![1.0, 2.0, 3.0]).get_f64());
}

#[test]
fn test_get_i8_into() {
    assert_eq!(Ok(vec![1, 2, 3]), DataVector::I8(vec![1, 2, 3]).get_i8_into());
    assert_eq!(Err(DataVector::U8(vec![1, 2, 3])), DataVector::U8(vec![1, 2, 3]).get_i8_into());
    assert_eq!(Err(DataVector::I16(vec![1, 2, 3])), DataVector::I16(vec![1, 2, 3]).get_i8_into());
    assert_eq!(Err(DataVector::I32(vec![1, 2, 3])), DataVector::I32(vec![1, 2, 3]).get_i8_into());
    assert_eq!(
        Err(DataVector::F32(vec![1.0, 2.0, 3.0])),
        DataVector::F32(vec![1.0, 2.0, 3.0]).get_i8_into()
    );
    assert_eq!(
        Err(DataVector::F64(vec![1.0, 2.0, 3.0])),
        DataVector::F64(vec![1.0, 2.0, 3.0]).get_i8_into()
    );
    {
        let data_1: Vec<i8> = vec![1, 2, 3];
        let ptr_1: *const i8 = data_1.as_ptr();

        // Frirst create a `DataVector::I8`
        let data_vec: DataVector = DataVector::I8(data_1);
        assert_eq!(DataType::I8, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<i8> = data_vec.get_i8_into().unwrap();
        let ptr_2: *const i8 = data_2.as_ptr();

        assert_eq!(vec![1, 2, 3], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_get_u8_into() {
    assert_eq!(Err(DataVector::I8(vec![1, 2, 3])), DataVector::I8(vec![1, 2, 3]).get_u8_into());
    assert_eq!(Ok(vec![1, 2, 3]), DataVector::U8(vec![1, 2, 3]).get_u8_into());
    assert_eq!(Err(DataVector::I16(vec![1, 2, 3])), DataVector::I16(vec![1, 2, 3]).get_u8_into());
    assert_eq!(Err(DataVector::I32(vec![1, 2, 3])), DataVector::I32(vec![1, 2, 3]).get_u8_into());
    assert_eq!(
        Err(DataVector::F32(vec![1.0, 2.0, 3.0])),
        DataVector::F32(vec![1.0, 2.0, 3.0]).get_u8_into()
    );
    assert_eq!(
        Err(DataVector::F64(vec![1.0, 2.0, 3.0])),
        DataVector::F64(vec![1.0, 2.0, 3.0]).get_u8_into()
    );

    {
        let data_1: Vec<u8> = vec![1, 2, 3];
        let ptr_1: *const u8 = data_1.as_ptr();

        // Frirst create a `DataVector::I8`
        let data_vec: DataVector = DataVector::U8(data_1);
        assert_eq!(DataType::U8, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_i8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<u8> = data_vec.get_u8_into().unwrap();
        let ptr_2: *const u8 = data_2.as_ptr();

        assert_eq!(vec![1, 2, 3], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_get_i16_into() {
    assert_eq!(Err(DataVector::I8(vec![1, 2, 3])), DataVector::I8(vec![1, 2, 3]).get_i16_into());
    assert_eq!(Err(DataVector::U8(vec![1, 2, 3])), DataVector::U8(vec![1, 2, 3]).get_i16_into());
    assert_eq!(Ok(vec![1, 2, 3]), DataVector::I16(vec![1, 2, 3]).get_i16_into());
    assert_eq!(Err(DataVector::I32(vec![1, 2, 3])), DataVector::I32(vec![1, 2, 3]).get_i16_into());
    assert_eq!(
        Err(DataVector::F32(vec![1.0, 2.0, 3.0])),
        DataVector::F32(vec![1.0, 2.0, 3.0]).get_i16_into()
    );
    assert_eq!(
        Err(DataVector::F64(vec![1.0, 2.0, 3.0])),
        DataVector::F64(vec![1.0, 2.0, 3.0]).get_i16_into()
    );
    {
        let data_1: Vec<i16> = vec![1, 2, 3];
        let ptr_1: *const i16 = data_1.as_ptr();

        // Frirst create a `DataVector::I8`
        let data_vec: DataVector = DataVector::I16(data_1);
        assert_eq!(DataType::I16, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_i8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<i16> = data_vec.get_i16_into().unwrap();
        let ptr_2: *const i16 = data_2.as_ptr();

        assert_eq!(vec![1, 2, 3], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_get_i32_into() {
    assert_eq!(Err(DataVector::I8(vec![1, 2, 3])), DataVector::I8(vec![1, 2, 3]).get_i32_into());
    assert_eq!(Err(DataVector::U8(vec![1, 2, 3])), DataVector::U8(vec![1, 2, 3]).get_i32_into());
    assert_eq!(Err(DataVector::I16(vec![1, 2, 3])), DataVector::I16(vec![1, 2, 3]).get_i32_into());
    assert_eq!(Ok(vec![1, 2, 3]), DataVector::I32(vec![1, 2, 3]).get_i32_into());
    assert_eq!(
        Err(DataVector::F32(vec![1.0, 2.0, 3.0])),
        DataVector::F32(vec![1.0, 2.0, 3.0]).get_i32_into()
    );
    assert_eq!(
        Err(DataVector::F64(vec![1.0, 2.0, 3.0])),
        DataVector::F64(vec![1.0, 2.0, 3.0]).get_i32_into()
    );
    {
        let data_1: Vec<i32> = vec![1, 2, 3];
        let ptr_1: *const i32 = data_1.as_ptr();

        // Frirst create a `DataVector::I32`
        let data_vec: DataVector = DataVector::I32(data_1);
        assert_eq!(DataType::I32, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_i8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<i32> = data_vec.get_i32_into().unwrap();
        let ptr_2: *const i32 = data_2.as_ptr();

        assert_eq!(vec![1, 2, 3], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_get_f32_into() {
    assert_eq!(Err(DataVector::I8(vec![1, 2, 3])), DataVector::I8(vec![1, 2, 3]).get_f32_into());
    assert_eq!(Err(DataVector::U8(vec![1, 2, 3])), DataVector::U8(vec![1, 2, 3]).get_f32_into());
    assert_eq!(Err(DataVector::I16(vec![1, 2, 3])), DataVector::I16(vec![1, 2, 3]).get_f32_into());
    assert_eq!(Err(DataVector::I32(vec![1, 2, 3])), DataVector::I32(vec![1, 2, 3]).get_f32_into());
    assert_eq!(Ok(vec![1.0, 2.0, 3.0]), DataVector::F32(vec![1.0, 2.0, 3.0]).get_f32_into());
    assert_eq!(
        Err(DataVector::F64(vec![1.0, 2.0, 3.0])),
        DataVector::F64(vec![1.0, 2.0, 3.0]).get_f32_into()
    );
    {
        let data_1: Vec<f32> = vec![1.0, 2.0, 3.0];
        let ptr_1: *const f32 = data_1.as_ptr();

        // Frirst create a `DataVector::I32`
        let data_vec: DataVector = DataVector::F32(data_1);
        assert_eq!(DataType::F32, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_i8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f64_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<f32> = data_vec.get_f32_into().unwrap();
        let ptr_2: *const f32 = data_2.as_ptr();

        assert_eq!(vec![1.0, 2.0, 3.0], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_get_f64_into() {
    assert_eq!(Err(DataVector::I8(vec![1, 2, 3])), DataVector::I8(vec![1, 2, 3]).get_f64_into());
    assert_eq!(Err(DataVector::U8(vec![1, 2, 3])), DataVector::U8(vec![1, 2, 3]).get_f64_into());
    assert_eq!(Err(DataVector::I16(vec![1, 2, 3])), DataVector::I16(vec![1, 2, 3]).get_f64_into());
    assert_eq!(Err(DataVector::I32(vec![1, 2, 3])), DataVector::I32(vec![1, 2, 3]).get_f64_into());
    assert_eq!(
        Err(DataVector::F32(vec![1.0, 2.0, 3.0])),
        DataVector::F32(vec![1.0, 2.0, 3.0]).get_f64_into()
    );
    assert_eq!(Ok(vec![1.0, 2.0, 3.0]), DataVector::F64(vec![1.0, 2.0, 3.0]).get_f64_into());
    {
        let data_1: Vec<f64> = vec![1.0, 2.0, 3.0];
        let ptr_1: *const f64 = data_1.as_ptr();

        // Frirst create a `DataVector::I32`
        let data_vec: DataVector = DataVector::F64(data_1);
        assert_eq!(DataType::F64, data_vec.data_type());

        // Try to extract the internal vector with the wrong data types
        let data_vec: DataVector = data_vec.get_i8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_u8_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i16_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_i32_into().unwrap_err();
        let data_vec: DataVector = data_vec.get_f32_into().unwrap_err();

        // Extract the internal vector with the good data type
        let data_2: Vec<f64> = data_vec.get_f64_into().unwrap();
        let ptr_2: *const f64 = data_2.as_ptr();

        assert_eq!(vec![1.0, 2.0, 3.0], data_2);
        // No copy of the buffer has been done
        assert_eq!(ptr_1, ptr_2);
    }
}

#[test]
fn test_equality_operator() {
    // Test equality between empty containers
    {
        let a = DataVector::I8(vec![]);
        let b = DataVector::I8(vec![]);
        assert_eq!(a, b)
    }

    // Test equality between 2 containers with different lengths
    {
        let a = DataVector::I8(vec![0; 0]);
        let b = DataVector::I8(vec![0; 1]);
        assert_ne!(a, b)
    }

    // Test equality between 2 i8-containers with the same length
    {
        let a = DataVector::I8(vec![1, 2, 3, 4]);
        let b = DataVector::I8(vec![1, 2, 3, 4]);
        let c = DataVector::I8(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i8-containers with the same length
    {
        let a = DataVector::U8(vec![b'a', b'b', b'c', b'd']);
        let b = DataVector::U8(vec![b'a', b'b', b'c', b'd']);
        let c = DataVector::U8(vec![b'a', b'b', b'c', b'c']);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i16-containers with the same length
    {
        let a = DataVector::I16(vec![1, 2, 3, 4]);
        let b = DataVector::I16(vec![1, 2, 3, 4]);
        let c = DataVector::I16(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 i32-containers with the same length
    {
        let a = DataVector::I32(vec![1, 2, 3, 4]);
        let b = DataVector::I32(vec![1, 2, 3, 4]);
        let c = DataVector::I32(vec![1, 2, 3, 3]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length
    {
        let a = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let b = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let c = DataVector::F32(vec![1.0, 2.0, 3.0, 3.0]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length and containing NaN
    {
        let a = DataVector::F32(vec![1.0, 2.0, 3.0, std::f32::NAN]);
        let b = DataVector::F32(vec![1.0, 2.0, 3.0, std::f32::NAN]);
        let c = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        assert_ne!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f64-containers with the same length
    {
        let a = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        let b = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        let c = DataVector::F64(vec![1.0, 2.0, 3.0, 3.0]);
        assert_eq!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 f32-containers with the same length and containing NaN
    {
        let a = DataVector::F64(vec![1.0, 2.0, 3.0, std::f64::NAN]);
        let b = DataVector::F64(vec![1.0, 2.0, 3.0, std::f64::NAN]);
        let c = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);
        assert_ne!(a, b);
        assert_ne!(c, a);
        assert_ne!(b, c);
    }

    // Test equality between 2 containers witht different data-types.
    {
        let data_i8 = DataVector::I8(vec![1, 2, 3, 4]);
        let data_u8 = DataVector::U8(vec![1, 2, 3, 4]);
        let data_i16 = DataVector::I16(vec![1, 2, 3, 4]);
        let data_i32 = DataVector::I32(vec![1, 2, 3, 4]);
        let data_f32 = DataVector::F32(vec![1.0, 2.0, 3.0, 4.0]);
        let data_f64 = DataVector::F64(vec![1.0, 2.0, 3.0, 4.0]);

        assert_ne!(data_i8, data_u8);
        assert_ne!(data_i8, data_i16);
        assert_ne!(data_i8, data_i32);
        assert_ne!(data_i8, data_f32);
        assert_ne!(data_i8, data_f64);

        assert_ne!(data_u8, data_i16);
        assert_ne!(data_u8, data_i32);
        assert_ne!(data_u8, data_f32);
        assert_ne!(data_u8, data_f64);

        assert_ne!(data_i16, data_i32);
        assert_ne!(data_i16, data_f32);
        assert_ne!(data_i16, data_f64);

        assert_ne!(data_i32, data_f32);
        assert_ne!(data_i32, data_f64);

        assert_ne!(data_f32, data_f64);
    }
}
