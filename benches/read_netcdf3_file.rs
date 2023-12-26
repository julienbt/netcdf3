use criterion::{criterion_group, criterion_main, Criterion};

use netcdf3::FileReader;

use copy_to_tmp_file::{
    copy_bytes_to_tmp_file,
    NC3_CLASSIC_FILE_NAME, NC3_CLASSIC_FILE_BYTES,
};

fn read_netcdf3_file(file_reader: &mut FileReader ) {
    let _ = file_reader.read_all_vars();
}

fn criterion_benchmark(c: &mut Criterion) {
    // Copy bytes to a temporary file
    // ------------------------------
    let (_tmp_dir, input_data_file_path) = copy_bytes_to_tmp_file(NC3_CLASSIC_FILE_BYTES, NC3_CLASSIC_FILE_NAME);

    // Open the NetCDF-3 file
    // ----------------------
    let mut file_reader = FileReader::open(input_data_file_path).unwrap();
    c.bench_function("read netcdf3 file", |b| b.iter(|| read_netcdf3_file(&mut file_reader)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);