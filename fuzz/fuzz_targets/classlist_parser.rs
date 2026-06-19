#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 4096 {
        return;
    }

    if let Ok(input) = std::str::from_utf8(data) {
        let _ = beam_core::parse_classlist(input);
    }
});
