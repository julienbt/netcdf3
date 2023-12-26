#![cfg(test)]

use crate::{
    DataSet, Version,
    FileReader, io::file_reader::SeekRead,
};

use super::ComputedDataSetMetadata;

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
};

#[test]
fn test_compute_header_required_size() {

    const EXPECTED_HEADER_SIZE: usize = 1_684;

    let (data_set, version): (DataSet, Version) = {
        let (tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);
        let file_reader = FileReader::open(input_data_file_path).unwrap();
        let (data_set, version, _input_cursor): (DataSet, Version, Box<dyn SeekRead>) = file_reader.close();
        tmp_dir.close().unwrap();
        (data_set, version)
    };

    let header_size: usize = ComputedDataSetMetadata::compute_header_required_size(&data_set, version);
    assert_eq!(EXPECTED_HEADER_SIZE,        header_size);
}
