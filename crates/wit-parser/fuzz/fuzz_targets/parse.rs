#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // fuzzed code goes here
    drop(env_logger::try_init());
    
    drop(wit_parser::parse(data, |_| false))
});
