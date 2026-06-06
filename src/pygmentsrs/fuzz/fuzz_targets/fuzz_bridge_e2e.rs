#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use pygmentsrs::bridge;

#[derive(Arbitrary, Debug)]
struct E2EInput {
    lexer_name: String,
    code: Vec<u8>,
    formatter_name: String,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = E2EInput::arbitrary(&mut Unstructured::new(data)) {
        let code_str = String::from_utf8_lossy(&input.code);
        
        // Lex the code
        if let Some(tokens) = bridge::lex(&input.lexer_name, &code_str) {
            // Format the tokens
            let _ = bridge::format(&input.formatter_name, &tokens);
        }
    }
});
