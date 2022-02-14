

// aliases
pub(crate) type NomErrorKind = nom::error::ErrorKind;
pub(crate) type NomError<'a> = nom::Err<(&'a[u8], NomErrorKind)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseHeaderError {
    pub kind: ParseHeaderErrorKind,
    pub invalid_bytes: InvalidBytes,
}

impl ParseHeaderError {

    pub(crate) fn new<'a>(err: NomError<'a>, kind: ParseHeaderErrorKind) -> Self {
        Self {
            kind: kind,
            invalid_bytes: InvalidBytes::from(err),
        }
    }

    pub fn header_is_incomplete(&self) -> bool {
        match self.invalid_bytes {
            InvalidBytes::Incomplete(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidBytes {
    Incomplete(nom::Needed),
    Bytes(Vec<u8>)
}

impl<'a> std::convert::From<NomError<'a>> for InvalidBytes {
    fn from(err: NomError<'a>) -> Self {
        match err {
            NomError::Incomplete(needed) => InvalidBytes::Incomplete(needed),
            NomError::Error((err_bytes, _err_kind)) => InvalidBytes::Bytes(err_bytes.to_owned()),
            NomError::Failure((err_bytes, _err_kind)) => InvalidBytes::Bytes(err_bytes.to_owned()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHeaderErrorKind {
    MagicWord,
    VersionNumber,
    NonNegativeI32,
    // NameString,
    ZeroPadding,
    DimTag,
    // DimId,
    AttrTag,
    VarTag,
    DataType,
    DataElements,
    Utf8,
    Offset,
}