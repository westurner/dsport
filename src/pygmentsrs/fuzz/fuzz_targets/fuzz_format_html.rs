#![no_main]
use libfuzzer_sys::fuzz_target;
use pygmentsrs::bridge;

fuzz_target!(|data: &[u8]| {
    // Fuzz HTML formatter with arbitrary token stream
    // Parse input as alternating token type + content length pairs
    let mut offset = 0;
    let mut tokens = Vec::new();
    
    while offset + 2 <= data.len() {
        // Read token type (1 byte: 0=Text, 1=Keyword, 2=String, 3=Comment, 4=Number)
        let token_type = match data[offset] % 5 {
            0 => "Token.Text",
            1 => "Token.Keyword",
            2 => "Token.String",
            3 => "Token.Comment",
            _ => "Token.Number",
        };
        offset += 1;
        
        // Read content length (1 byte)
        let content_len = data[offset] as usize;
        offset += 1;
        
        // Read content
        let end = std::cmp::min(offset + content_len, data.len());
        let content = String::from_utf8_lossy(&data[offset..end]).to_string();
        offset = end;
        
        tokens.push((token_type.to_string(), content));
    }
    
    if !tokens.is_empty() {
        let _ = bridge::format("html", &tokens);
    }
});
