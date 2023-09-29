use std::io::{Read, Write};
use super::syntax::{Encode, Decode, Error};
use super::quic_varint:: {QUICVarint};

/// MOQ Transport Track Namespace
#[derive(Debug, Clone, PartialEq)]
pub struct TrackNamespace(pub Vec<u8>);

impl Encode for TrackNamespace {
   fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
      let len = QUICVarint(self.0.len() as u64);
      let enc_len = len.encode(writer).map_err(|e| Error::EncodingError(e.to_string()))?;
      writer.write_all(self.0.as_slice()).map_err(|e| Error::EncodingError(e.to_string()))?;
      Ok(self.0.len() + enc_len)
   }
}

impl Decode for TrackNamespace {
   fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
      let mut len = [0];
      reader.read_exact(&mut len)?;
      // TODO (Suhas) Can we use match here ?
      if len[0] == 0 {
         return Err(Error::DecodingError("TrackNamespace of zero length".to_owned()));
      }
      let num_bytes: usize = len[0] as usize;
      let mut bytes = vec![0; num_bytes];
      reader.read_exact(&mut bytes[..num_bytes]).map_err(|e| Error::DecodingError(e.to_string()))?;
      Ok(TrackNamespace(bytes))
   }
}


#[derive(Debug, Clone, PartialEq)]
pub struct FullTrackName(pub Vec<u8>);

impl Encode for FullTrackName {
   fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
      let len = QUICVarint(self.0.len() as u64);
      let enc_len = len.encode(writer).map_err(|e| Error::EncodingError(e.to_string()))?;
      writer.write_all(self.0.as_slice()).map_err(|e| Error::EncodingError(e.to_string()))?;
      Ok(self.0.len() + enc_len)
   }
}

impl Decode for FullTrackName {
   fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
      let mut len = [0];
      reader.read_exact(&mut len)?;
      // TODO (Suhas) Can we use match here ?
      if len[0] == 0 {
         return Err(Error::DecodingError("FullTrackName of zero length".to_owned()));
      }
      let num_bytes: usize = len[0] as usize;
      let mut bytes = vec![0; num_bytes];
      reader.read_exact(&mut bytes[..num_bytes]).map_err(|e| Error::DecodingError(e.to_string()))?;
      Ok(FullTrackName(bytes))
   }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn tracknamespace_encode_decode() {
      let tn_before = TrackNamespace(b"http://example.com/meetings/123/user/1".to_vec());
      let mut enc = Vec::new();
      tn_before.encode(&mut enc).expect("Encoding failed for track namespace");
      let tn_after = TrackNamespace::decode(&mut enc.as_slice())
                                       .expect("Decoding failed for track namespace");
      assert_eq!(tn_before, tn_after);
      // String::from_utf8(tn_after.0).expect("Our bytes should be valid utf8");
   }

   #[test]
   fn fulltrackname_encode_decode() {
      let ftn_before = TrackNamespace(b"http://example.com/meetings/123/user/1/video/2".to_vec());
      let mut enc = Vec::new();
      ftn_before.encode(&mut enc).expect("Encoding failed for full track name");
      let ftn_after = TrackNamespace::decode(&mut enc.as_slice())
          .expect("Decoding failed for full track name");
      assert_eq!(ftn_before, ftn_after);
      // String::from_utf8(tn_after.0).expect("Our bytes should be valid utf8");
   }

}