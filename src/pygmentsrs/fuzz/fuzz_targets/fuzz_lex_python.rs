#![no_main]
use libfuzzer_sys::fuzz_target;
use pygmentsrs::bridge;

fuzz_target!(|data: &[u8]| {
    // Fuzz Python lexer with arbitrary input
    let code = String::from_utf8_lossy(data);
    let _ = bridge::lex("python", &code);
});
