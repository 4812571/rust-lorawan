#![no_main]

use libfuzzer_sys::fuzz_target;
use lorawan::parser::parse;

fuzz_target!(|data: &[u8]| {
    let owned_copy = data.to_vec();
    let _ = parse(owned_copy);
});