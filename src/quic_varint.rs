use super::syntax::{Encode, Decode, Error};
use std::io::{Read, Write};

///
/// Varint (RFC9000)
///

#[derive(Debug, Clone, PartialEq)]
pub struct QUICVarint(pub u64);


fn compute_encode_length(value: u64) -> Result<usize, Error> {
    Ok(match value {
        x if x < 0x3f => 1,
        x if x < 0x3fff => 2,
        x if x < 0x3fff_ffff => 4,
        x if x < 0x3fff_ffff_ffff_ffff => 8,
        _ => return Err(Error::EncodingError(String::from("Integer value too big"))),
    })
}

impl Encode for QUICVarint {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        // [00, 00, 00, 00, 00, 00, xx, xx]
        let mut bytes = self.0.to_be_bytes();

        let num_bytes = compute_encode_length(self.0)?; // => 2
        let start = bytes.len() - num_bytes; // => 6
        let bytes = &mut bytes[start..];
        // [xx, xx]

        // XXX: Replace with log2
        let header = match num_bytes {
            1 => 0b00u8,
            2 => 0b01u8,
            4 => 0b10u8,
            8 => 0b11u8,
            _ => unreachable!(),
        };
        bytes[0] |= header << 6;

        writer.write(bytes).map_err(|e| Error::EncodingError(e.to_string()))?;
        Ok(num_bytes)
    }
}


impl Decode for QUICVarint {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        let mut bytes = [0u8; 8];
        reader.read(&mut bytes[..1]).map_err(|e| Error::DecodingError(e.to_string()))?;
        let header = bytes[0] >> 6;
        bytes[0] &= 0x3f;
        let num_bytes = 1 << header; // already read one byte
        reader.read(&mut bytes[1..num_bytes]).map_err(|e| Error::DecodingError(e.to_string()))?;
        // [xx yy 00 00 00 00 00 00]
        let mut val = u64::from_be_bytes(bytes); //xxyy0000000000
        val >>= 8 * (8 - num_bytes); //00000000xxyy
        Ok(QUICVarint(val))
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quic_varint_encode_decode() {
        // todo (suhas) : add 1, 2, 4 and 8 bytes tests
        // todo (suhas): add test to check max length error conditions
        let qv_before = QUICVarint(494_878_333);
        let mut enc = Vec::new();
        qv_before.encode(&mut enc).expect("Error encoding varint");
        let qv_after = QUICVarint::decode(&mut enc.as_slice()).expect("unable to decode qv");
        assert_eq!(qv_before, qv_after);
    }
}