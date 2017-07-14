#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rsdparsa;

use rsdparsa::media_type::parse_media;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_media(s);
    }
});
