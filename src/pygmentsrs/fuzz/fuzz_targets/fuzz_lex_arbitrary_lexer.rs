#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use pygmentsrs::bridge;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    lexer_name: String,
    code: Vec<u8>,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = FuzzInput::arbitrary(&mut Unstructured::new(data)) {
        // Fuzz lexing with arbitrary lexer name and code
        let code_str = String::from_utf8_lossy(&input.code);
        let _ = bridge::lex(&input.lexer_name, &code_str);
    }
});
