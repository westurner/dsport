//! Source byte decoding compatible with Docutils input handling.

use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDecodeError {
    pub encoding: String,
    pub message: String,
}

impl fmt::Display for SourceDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unable to decode source as {}: {}",
            self.encoding, self.message
        )
    }
}

impl std::error::Error for SourceDecodeError {}

/// Decode source bytes using an explicit encoding.
///
/// This follows the Sphinx/Docutils default of `utf-8-sig` while retaining
/// strict decoding. Latin-1 is supported explicitly, not as a silent fallback
/// for malformed UTF-8.
pub fn decode_source(bytes: &[u8], encoding: &str) -> Result<String, SourceDecodeError> {
    let normalized = encoding.trim().to_ascii_lowercase().replace('_', "-");
    let compact: String = normalized
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect();
    if matches!(
        compact.as_str(),
        "latin1" | "l1" | "cp819" | "ibm819" | "csisolatin1" | "iso88591"
    ) {
        return Ok(bytes.iter().map(|&byte| char::from(byte)).collect());
    }
    if compact == "ascii" || compact == "usascii" {
        if bytes.iter().any(|&byte| byte >= 0x80) {
            return Err(error(encoding, "non-ASCII byte"));
        }
        return String::from_utf8(bytes.to_vec()).map_err(|_| error(encoding, "invalid ASCII"));
    }
    if compact == "utf8sig" {
        return std::str::from_utf8(bytes)
            .map(|source| {
                source
                    .strip_prefix('\u{feff}')
                    .unwrap_or(source)
                    .to_string()
            })
            .map_err(|err| error(encoding, &err.to_string()));
    }
    if compact == "utf8" {
        return std::str::from_utf8(bytes)
            .map(str::to_string)
            .map_err(|err| error(encoding, &err.to_string()));
    }
    if matches!(compact.as_str(), "utf16" | "utf16le" | "utf16be") {
        let little_endian = match compact.as_str() {
            "utf16be" => false,
            "utf16le" => true,
            _ => !bytes.starts_with(&[0xfe, 0xff]),
        };
        if bytes.len() % 2 != 0 {
            return Err(error(encoding, "odd number of bytes"));
        }
        let units: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| {
                if little_endian {
                    u16::from_le_bytes([chunk[0], chunk[1]])
                } else {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                }
            })
            .collect();
        let units = if units.first() == Some(&0xfeff) {
            &units[1..]
        } else {
            &units
        };
        return String::from_utf16(units).map_err(|err| error(encoding, &err.to_string()));
    }
    if matches!(compact.as_str(), "utf32" | "utf32le" | "utf32be") {
        let little_endian = match compact.as_str() {
            "utf32be" => false,
            "utf32le" => true,
            _ => !bytes.starts_with(&[0, 0, 0xfe, 0xff]),
        };
        if bytes.len() % 4 != 0 {
            return Err(error(encoding, "length is not a multiple of four bytes"));
        }
        let mut source = String::new();
        for chunk in bytes.chunks_exact(4) {
            let codepoint = if little_endian {
                u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            } else {
                u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            };
            if codepoint == 0xfeff && source.is_empty() {
                continue;
            }
            let Some(character) = char::from_u32(codepoint) else {
                return Err(error(encoding, "invalid Unicode code point"));
            };
            source.push(character);
        }
        return Ok(source);
    }

    let codec = encoding_rs::Encoding::for_label(normalized.as_bytes())
        .ok_or_else(|| error(encoding, "unknown encoding"))?;
    let decoded: Option<Cow<'_, str>> =
        codec.decode_without_bom_handling_and_without_replacement(bytes);
    decoded
        .map(Cow::into_owned)
        .ok_or_else(|| error(encoding, "invalid byte sequence"))
}

/// Decode source bytes using Docutils-style declaration detection.
pub fn decode_source_auto(bytes: &[u8]) -> Result<String, SourceDecodeError> {
    let encoding = detect_encoding(bytes).unwrap_or_else(|| "utf-8".to_string());
    decode_source(bytes, &encoding)
}

fn detect_encoding(bytes: &[u8]) -> Option<String> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Some("utf-8-sig".to_string());
    }
    if bytes.starts_with(&[0xff, 0xfe, 0, 0]) {
        return Some("utf-32-le".to_string());
    }
    if bytes.starts_with(&[0, 0, 0xfe, 0xff]) {
        return Some("utf-32-be".to_string());
    }
    if bytes.starts_with(&[0xff, 0xfe]) {
        return Some("utf-16-le".to_string());
    }
    if bytes.starts_with(&[0xfe, 0xff]) {
        return Some("utf-16-be".to_string());
    }
    let scan = &bytes[..bytes.len().min(2048)];
    let mut line_start = 0;
    let mut lines_seen = 0;
    let mut index = 0;
    while lines_seen < 2 && index <= scan.len() {
        let line_end = if index == scan.len() {
            scan.len()
        } else if matches!(scan[index], b'\n' | b'\r') {
            index
        } else {
            index += 1;
            continue;
        };
        let line = &scan[line_start..line_end];
        let lower = line.to_ascii_lowercase();
        for marker in [b"coding:", b"coding="] {
            if let Some(start) = lower.windows(marker.len()).position(|w| w == marker) {
                let value = line[start + marker.len()..].trim_ascii_start();
                let end = value
                    .iter()
                    .position(|byte| {
                        !byte.is_ascii_alphanumeric()
                            && *byte != b'-'
                            && *byte != b'_'
                            && *byte != b'.'
                    })
                    .unwrap_or(value.len());
                if end > 0 {
                    let label = std::str::from_utf8(&value[..end]).ok()?;
                    return Some(label.to_string());
                }
            }
        }
        lines_seen += 1;
        if index == scan.len() {
            break;
        }
        index += 1;
        if line_end < scan.len() && scan[line_end] == b'\r' && scan.get(index) == Some(&b'\n') {
            index += 1;
        }
        line_start = index;
    }
    None
}

fn error(encoding: &str, message: &str) -> SourceDecodeError {
    SourceDecodeError {
        encoding: encoding.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_utf8_sig_without_bom() {
        assert_eq!(
            decode_source(b"\xef\xbb\xbfTitle", "utf-8-sig").unwrap(),
            "Title"
        );
    }

    #[test]
    fn decodes_explicit_latin1() {
        assert_eq!(decode_source(b"Caf\xfc", "latin-1").unwrap(), "Caf\u{fc}");
    }

    #[test]
    fn all_common_latin1_aliases_use_iso8859_1_mapping() {
        for alias in [
            "latin-1",
            "latin1",
            "l1",
            "cp819",
            "ibm819",
            "csisolatin1",
            "iso-8859-1",
            "iso8859-1",
        ] {
            assert_eq!(
                decode_source(b"\x80\x9f", alias).unwrap(),
                "\u{80}\u{9f}",
                "{alias}"
            );
        }
    }

    #[test]
    fn rejects_invalid_utf8_instead_of_falling_back() {
        assert!(decode_source(b"Caf\xfc", "utf-8-sig").is_err());
    }

    #[test]
    fn detects_latin1_coding_declaration() {
        assert_eq!(
            decode_source_auto(b".. coding: latin-1\nCaf\xfc").unwrap(),
            ".. coding: latin-1\nCaf\u{fc}"
        );
    }

    #[test]
    fn detects_arbitrary_encoding_declaration() {
        assert_eq!(
            decode_source_auto(b".. coding: windows-1252\nEuro: \x80").unwrap(),
            ".. coding: windows-1252\nEuro: \u{20ac}"
        );
    }

    #[test]
    fn decodes_utf16_bom() {
        assert_eq!(
            decode_source_auto(b"\xff\xfeT\0i\0t\0l\0e\0").unwrap(),
            "Title"
        );
    }

    #[test]
    fn decodes_bare_utf16_using_bom_endianness() {
        assert_eq!(
            decode_source(b"\xfe\xff\x00T\x00i\x00t\x00l\x00e", "utf-16").unwrap(),
            "Title"
        );
    }

    #[test]
    fn decodes_bare_utf32_using_bom_endianness() {
        assert_eq!(
            decode_source(
                b"\x00\x00\xfe\xff\x00\x00\x00T\x00\x00\x00i\x00\x00\x00t\x00\x00\x00l\x00\x00\x00e",
                "utf-32"
            )
            .unwrap(),
            "Title"
        );
    }

    #[test]
    fn detects_coding_declaration_with_classic_mac_line_endings() {
        assert_eq!(
            decode_source_auto(b".. coding: latin-1\rCaf\xfc").unwrap(),
            ".. coding: latin-1\rCaf\u{fc}"
        );
    }

    #[test]
    fn bounds_coding_declaration_scan_window() {
        let mut bytes = vec![b'x'; 3000];
        bytes.extend_from_slice(b" coding: latin-1\rCaf\xfc");
        assert!(decode_source_auto(&bytes).is_err());
    }
}
