mod file_reader;
mod file_writer;
mod tests_io;

pub use file_reader::FileReader;
pub use file_writer::FileWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Offset {
    I32(i32),
    I64(i64),
}

impl std::convert::From<Offset> for i64 {
    fn from(offset: Offset) -> Self {
        match offset {
            Offset::I32(value) => value as i64,
            Offset::I64(value) => value,
        }
    }
}

/// These bytes mean the list (dimensions, attributes or variable) is not defined.
pub(crate) const ABSENT_TAG: [u8; 8] = [0; 8];
/// Bytes for the list of dimensions
pub(crate) const DIMENSION_TAG: [u8; 4] = [0, 0, 0, 0x0A];
/// Bytes for the list of variables
pub(crate) const VARIABLE_TAG: [u8; 4] = [0, 0, 0, 0x0b];
/// Bytes for the lists attributes (global or for each variable).
pub(crate) const ATTRIBUTE_TAG: [u8; 4] = [0, 0, 0, 0x0C];

#[inline]
/// Compute and return the number of bytes of the padding required to fill remaining bytes up.
///
/// Arguments :
/// - `num_bytes` : the number of useful bytes
pub fn compute_padding_size(num_bytes: usize) -> usize {
    const ALIGNMENT_SIZE: usize = 4;
    return match num_bytes % 4 {
        0 => 0,
        n => ALIGNMENT_SIZE - n,
    };
}
