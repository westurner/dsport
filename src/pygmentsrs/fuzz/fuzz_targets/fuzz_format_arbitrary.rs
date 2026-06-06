#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use pygmentsrs::bridge;

#[derive(Arbitrary, Debug)]
struct FormatInput {
    formatter_name: String,
    tokens_data: Vec<u8>,
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = FormatInput::arbitrary(&mut Unstructured::new(data)) {
        // Parse tokens from remaining data
        let mut tokens = Vec::new();
        let mut offset = 0;
        
        while offset + 2 <= input.tokens_data.len() {
            let token_type_len = input.tokens_data[offset] as usize;
            offset += 1;
            
            let end_type = std::cmp::min(offset + token_type_len, input.tokens_data.len());
            let token_type = String::from_utf8_lossy(&input.tokens_data[offset..end_type]).to_string();
            offset = end_type;
            
            if offset >= input.tokens_data.len() {
                break;
            }
            
            let content_len = input.tokens_data[offset] as usize;
            offset += 1;
            
            let end_content = std::cmp::min(offset + content_len, input.tokens_data.len());
            let content = String::from_utf8_lossy(&input.tokens_data[offset..end_content]).to_string();
            offset = end_content;
            
            if !token_type.is_empty() {
                tokens.push((token_type, content));
            }
        }
        
        if !tokens.is_empty() {
            let _ = bridge::format(&input.formatter_name, &tokens);
        }
    }
});
