//! Minimal STORED-only ZIP writer (no compression).
//!
//! Just enough of PKZIP `APPNOTE.TXT` v6.3.3 to produce a valid
//! ODF container: local file headers + raw data + central directory
//! + end-of-central-directory record. No deflate, no encryption.

pub struct ZipBuilder {
    out: Vec<u8>,
    entries: Vec<Entry>,
}

struct Entry {
    name: Vec<u8>,
    crc32: u32,
    size: u32,
    offset: u32,
}

impl ZipBuilder {
    pub fn new() -> Self {
        Self {
            out: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_file(&mut self, name: &str, data: &[u8]) {
        let offset = self.out.len() as u32;
        let crc = crc32(data);
        let size = data.len() as u32;
        let name_bytes = name.as_bytes().to_vec();
        // Local file header (signature 0x04034b50).
        self.out.extend_from_slice(&0x04034b50u32.to_le_bytes());
        self.out.extend_from_slice(&20u16.to_le_bytes()); // version needed
        self.out.extend_from_slice(&0u16.to_le_bytes()); // flags
        self.out.extend_from_slice(&0u16.to_le_bytes()); // method = stored
        self.out.extend_from_slice(&0u16.to_le_bytes()); // mtime
        self.out.extend_from_slice(&0x21u16.to_le_bytes()); // mdate (1980-01-01)
        self.out.extend_from_slice(&crc.to_le_bytes());
        self.out.extend_from_slice(&size.to_le_bytes()); // compressed
        self.out.extend_from_slice(&size.to_le_bytes()); // uncompressed
        self.out
            .extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        self.out.extend_from_slice(&0u16.to_le_bytes()); // extra
        self.out.extend_from_slice(&name_bytes);
        self.out.extend_from_slice(data);
        self.entries.push(Entry {
            name: name_bytes,
            crc32: crc,
            size,
            offset,
        });
    }

    pub fn finish(mut self) -> Vec<u8> {
        let cd_offset = self.out.len() as u32;
        for e in &self.entries {
            // Central directory file header (signature 0x02014b50).
            self.out.extend_from_slice(&0x02014b50u32.to_le_bytes());
            self.out.extend_from_slice(&20u16.to_le_bytes()); // made by
            self.out.extend_from_slice(&20u16.to_le_bytes()); // needed
            self.out.extend_from_slice(&0u16.to_le_bytes()); // flags
            self.out.extend_from_slice(&0u16.to_le_bytes()); // method
            self.out.extend_from_slice(&0u16.to_le_bytes()); // mtime
            self.out.extend_from_slice(&0x21u16.to_le_bytes()); // mdate
            self.out.extend_from_slice(&e.crc32.to_le_bytes());
            self.out.extend_from_slice(&e.size.to_le_bytes());
            self.out.extend_from_slice(&e.size.to_le_bytes());
            self.out
                .extend_from_slice(&(e.name.len() as u16).to_le_bytes());
            self.out.extend_from_slice(&0u16.to_le_bytes()); // extra
            self.out.extend_from_slice(&0u16.to_le_bytes()); // comment
            self.out.extend_from_slice(&0u16.to_le_bytes()); // disk
            self.out.extend_from_slice(&0u16.to_le_bytes()); // internal attrs
            self.out.extend_from_slice(&0u32.to_le_bytes()); // external attrs
            self.out.extend_from_slice(&e.offset.to_le_bytes());
            self.out.extend_from_slice(&e.name);
        }
        let cd_size = (self.out.len() as u32) - cd_offset;
        // End of central directory record (signature 0x06054b50).
        self.out.extend_from_slice(&0x06054b50u32.to_le_bytes());
        self.out.extend_from_slice(&0u16.to_le_bytes()); // this disk
        self.out.extend_from_slice(&0u16.to_le_bytes()); // disk w/ cd
        self.out
            .extend_from_slice(&(self.entries.len() as u16).to_le_bytes());
        self.out
            .extend_from_slice(&(self.entries.len() as u16).to_le_bytes());
        self.out.extend_from_slice(&cd_size.to_le_bytes());
        self.out.extend_from_slice(&cd_offset.to_le_bytes());
        self.out.extend_from_slice(&0u16.to_le_bytes()); // comment len
        self.out
    }
}

/// CRC-32 (IEEE 802.3 polynomial 0xEDB88320), table-driven.
fn crc32(data: &[u8]) -> u32 {
    static TABLE: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut t = [0u32; 256];
        for (i, slot) in t.iter_mut().enumerate() {
            let mut c = i as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    0xEDB88320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
            *slot = c;
        }
        t
    });
    let mut c = 0xFFFFFFFFu32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFFFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_empty_zero() {
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn crc32_known_vector() {
        // Known CRC-32 of "123456789" is 0xCBF43926.
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn zip_roundtrip_minimum() {
        let mut z = ZipBuilder::new();
        z.add_file("a.txt", b"hello");
        let bytes = z.finish();
        // Signatures present.
        assert!(bytes.windows(4).any(|w| w == 0x04034b50u32.to_le_bytes()));
        assert!(bytes.windows(4).any(|w| w == 0x02014b50u32.to_le_bytes()));
        assert!(bytes.windows(4).any(|w| w == 0x06054b50u32.to_le_bytes()));
        // Contains payload verbatim.
        assert!(bytes.windows(5).any(|w| w == b"hello"));
    }
}
