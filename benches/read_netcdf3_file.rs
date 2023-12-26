use std::io::Cursor;

use criterion::{criterion_group, criterion_main, Criterion};

use netcdf3::FileReader;

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
};

fn read_netcdf3_file(input_file_path: &std::path::Path ) {
    let mut file_reader = FileReader::open(input_file_path).unwrap();
    let _ = file_reader.read_all_vars();
}

fn read_netcdf3_bytes(input_bytes: Cursor<Vec<u8>> ) {
    let mut file_reader = FileReader::read_bytes("", Box::new(input_bytes)).unwrap();
    let _ = file_reader.read_all_vars();
}

fn criterion_benchmark(c: &mut Criterion) {
    // Copy bytes to a temporary file
    // ------------------------------
    let (_tmp_dir, input_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    c.bench_function("read netcdf3 file", |b| b.iter(|| read_netcdf3_file(&input_file_path)));

    let input_bytes: Cursor<Vec<u8>> =  std::io::Cursor::new(Vec::from(NC3_CLASSIC_FILE_BYTES));
    c.bench_function("read netcdf3 bytes", |b| b.iter(|| read_netcdf3_bytes(input_bytes.clone())));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);