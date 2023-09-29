use std::io::{Read, Write};
use core::fmt::{self, Display};

///
/// Error
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    /// An error occurred during encoding.
    EncodingError(String),
    /// An error occurred during decoding.
    DecodingError(String),
    EndOfStream,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::UnexpectedEof => Self::EndOfStream,
            _ => Self::DecodingError(format!("io error: {e:?}")),
        }
    }
}


///
///  Trait Serialize
///
pub trait Encode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}


///
/// Trait Deserialize
///
pub trait Decode {
    // return self typ
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error>
        where Self: Sized;
}

///
/// Implementations for encode/decode on primitives
///

impl Encode for  u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let written = writer.write(&self.to_be_bytes())?;
        debug_assert_eq!(written, 1);
        Ok(written)
    }
}

impl Encode for u16 {
    // self to usize
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let written = writer.write(&self.to_be_bytes())?;
        debug_assert_eq!(written, 2);
        Ok(written)
    }
}



impl Decode for u8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut x = [0];
        reader.read_exact(&mut x)?;
        Ok(<u8>::from_be_bytes(x))
    }
}

impl Decode for u16 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self:Sized {
        let mut x = [0;2];
        reader.read_exact(&mut x)?;
        Ok(<u16>::from_be_bytes(x))
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primitives_encode_decode() {
        let mut enc = Vec::new();
        let u8_before = 12u8;
        let u16_before = 333u16;
        u8_before.encode(&mut enc).expect("Error encoding u8");
        u16_before.encode(&mut enc).expect("Error encoding u16");
        let u8_after = u8::decode(&mut enc.as_slice()).expect("Error decoding u8");
        assert_eq!(u8_before, u8_after);
        let mut enc = &enc[1..];
        let u16_after = u16::decode(&mut enc).expect("Error decoding u16");
        assert_eq!(u16_before, u16_after);
    }

}
