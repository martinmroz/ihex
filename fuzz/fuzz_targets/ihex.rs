#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let reader = ihex::Reader::new(data);

    let output = reader.collect::<Result<Vec<_>, ihex::ReaderError>>();

    let _ = std::hint::black_box(output);
});
