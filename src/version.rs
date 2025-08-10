#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// NetCDF-3 file versions (classic or 64-bit offset)
pub enum Version {
    /// Classic format (use `i32` for the begin offsets)
    Classic = 1,
    /// 64-bit offset format (use `i64` for the begin offsets)
    Offset64Bit = 2,
}

impl std::convert::TryFrom<u8> for Version {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1_u8 => Ok(Version::Classic),
            2_u8 => Ok(Version::Offset64Bit),
            _ => Err("Invalid value for a NetCDF-3 version."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Version;
    use std::convert::TryFrom;

    #[test]
    fn test_version_try_from_u8() {
        assert_eq!(
            Err("Invalid value for a NetCDF-3 version."),
            Version::try_from(0_u8)
        );
        assert_eq!(Ok(Version::Classic), Version::try_from(1_u8));
        assert_eq!(Ok(Version::Offset64Bit), Version::try_from(2_u8));
        assert_eq!(
            Err("Invalid value for a NetCDF-3 version."),
            Version::try_from(3_u8)
        );
    }
}
