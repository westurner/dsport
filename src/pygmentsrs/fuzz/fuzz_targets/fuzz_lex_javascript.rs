#![no_main]
use libfuzzer_sys::fuzz_target;
use pygmentsrs::bridge;

fuzz_target!(|data: &[u8]| {
    // Fuzz JavaScript lexer with arbitrary input
    let code = String::from_utf8_lossy(data);
    let _ = bridge::lex("javascript", &code);
});
