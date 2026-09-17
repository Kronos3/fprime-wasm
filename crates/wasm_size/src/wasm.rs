/// Section ids from the WebAssembly core specification.
const CODE_SECTION: u8 = 10;
const DATA_SECTION: u8 = 11;

/// The section byte counts of one module.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Sections {
    /// Every byte of the file, including the header and section framing.
    pub total: usize,
    /// Payload of the code section, i.e. function bodies.
    pub code: usize,
    /// Payload of the data section, i.e. initialised memory. Const-encoded
    /// command buffers live here.
    pub data: usize,
}

/// Read a LEB128 unsigned integer at `offset`, returning it and the offset past
/// it.
fn leb128(bytes: &[u8], mut offset: usize) -> Option<(usize, usize)> {
    let mut value: usize = 0;
    let mut shift = 0;

    loop {
        let byte = *bytes.get(offset)?;
        offset += 1;

        // A section length wider than this is not something we produce, and
        // shifting past the width of the accumulator would be a silent wrap.
        if shift >= usize::BITS {
            return None;
        }

        value |= ((byte & 0x7F) as usize) << shift;
        shift += 7;

        if byte & 0x80 == 0 {
            return Some((value, offset));
        }
    }
}

/// Walk the section headers of a wasm module.
///
/// Returns `None` if `bytes` is not a wasm module or is truncated, rather than
/// guessing: a size report that silently reads zero would be worse than no
/// report.
pub fn sections(bytes: &[u8]) -> Option<Sections> {
    // 4 byte magic `\0asm` then a 4 byte version.
    if bytes.len() < 8 || &bytes[0..4] != b"\0asm" {
        return None;
    }

    let mut out = Sections {
        total: bytes.len(),
        ..Sections::default()
    };

    let mut offset = 8;
    while offset < bytes.len() {
        let id = bytes[offset];
        let (length, payload) = leb128(bytes, offset + 1)?;

        // A length that runs off the end means we have lost sync.
        let end = payload.checked_add(length)?;
        if end > bytes.len() {
            return None;
        }

        match id {
            CODE_SECTION => out.code += length,
            DATA_SECTION => out.data += length,
            _ => {}
        }

        offset = end;
    }

    Some(out)
}

#[cfg(test)]
mod test {
    use super::*;

    /// `\0asm`, version 1, then one section of the given id and payload.
    fn module(id: u8, payload: &[u8]) -> Vec<u8> {
        let mut bytes = b"\0asm\x01\0\0\0".to_vec();
        bytes.push(id);
        bytes.push(payload.len() as u8);
        bytes.extend_from_slice(payload);
        bytes
    }

    #[test]
    fn reads_the_code_and_data_sections() {
        let mut bytes = module(CODE_SECTION, &[1, 2, 3, 4]);
        bytes.extend_from_slice(&module(DATA_SECTION, &[9, 9])[8..]);

        let sections = sections(&bytes).expect("should parse");
        assert_eq!(sections.code, 4);
        assert_eq!(sections.data, 2);
        assert_eq!(sections.total, bytes.len());
    }

    #[test]
    fn ignores_other_sections() {
        // Section 1 is the type section; it should count toward the total only.
        let bytes = module(1, &[0, 0, 0]);
        let sections = sections(&bytes).expect("should parse");

        assert_eq!(sections.code, 0);
        assert_eq!(sections.data, 0);
        assert_eq!(sections.total, bytes.len());
    }

    #[test]
    fn reads_a_multi_byte_length() {
        // 200 bytes needs two LEB128 bytes, so this catches a single-byte read.
        let payload = vec![0u8; 200];
        let mut bytes = b"\0asm\x01\0\0\0".to_vec();
        bytes.push(CODE_SECTION);
        bytes.extend_from_slice(&[0xC8, 0x01]);
        bytes.extend_from_slice(&payload);

        assert_eq!(sections(&bytes).expect("should parse").code, 200);
    }

    #[test]
    fn rejects_what_is_not_a_module() {
        assert_eq!(sections(b"not wasm at all"), None);
        assert_eq!(sections(b""), None);
    }

    #[test]
    fn rejects_a_truncated_section() {
        // Claims 40 bytes of payload but carries 4.
        let mut bytes = b"\0asm\x01\0\0\0".to_vec();
        bytes.push(CODE_SECTION);
        bytes.push(40);
        bytes.extend_from_slice(&[1, 2, 3, 4]);

        assert_eq!(sections(&bytes), None);
    }
}
