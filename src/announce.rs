use std::io::{Read, Write};
use crate::quic_varint::QUICVarint;
use crate::syntax::{Decode, Encode, Error};
use crate::track::TrackNamespace;

const MESSAGE_TYPE_ANNOUNCE:u8 = 0x6;
const MESSAGE_TYPE_ANNOUNCE_OK:u8 = 0x7;
const MESSAGE_TYPE_ANNOUNCE_ERROR:u8 = 0x8;
const MESSAGE_TYPE_UNANNOUNCE:u8 = 0x9;


#[derive(Debug, Clone, PartialEq)]
pub struct Announce {
    track_namespace: TrackNamespace
    // TODO(Suhas): Add track_request_parameters
}

impl Encode for Announce {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        MESSAGE_TYPE_ANNOUNCE.encode(writer)?;
        self.track_namespace.encode(writer)
    }
}

impl Decode for Announce {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        // validate the message type
        let mut messge_type = [0];
        reader.read_exact(&mut messge_type)?;
        if messge_type[0] != MESSAGE_TYPE_ANNOUNCE {
            return Err(Error::DecodingError("Invalid message type for announce".to_owned()));
        }

        let tn = TrackNamespace::decode(reader)?;
        Ok(Announce {
            track_namespace: tn,
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AnnounceOk {
    track_namespace: TrackNamespace
}

impl Encode for AnnounceOk {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        MESSAGE_TYPE_ANNOUNCE_OK.encode(writer)?;
        self.track_namespace.encode(writer)
    }
}

impl Decode for AnnounceOk {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        // validate the message type
        let mut messge_type = [0];
        reader.read_exact(&mut messge_type)?;
        if messge_type[0] != MESSAGE_TYPE_ANNOUNCE_OK {
            return Err(Error::DecodingError("Invalid message type for announce".to_owned()));
        }

        let tn = TrackNamespace::decode(reader)?;
        Ok(AnnounceOk {
            track_namespace: tn,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnounceError {
    track_namespace:TrackNamespace,
    error_code: QUICVarint,
    reason_phrase: String,
}


impl Encode for AnnounceError {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let mut encoded_len = MESSAGE_TYPE_ANNOUNCE_ERROR.encode(writer)?;
        encoded_len += self.track_namespace.encode(writer)?;
        encoded_len += self.error_code.encode(writer)?;
        let reason_phrase_qv = QUICVarint(self.reason_phrase.len() as u64);
        encoded_len += reason_phrase_qv.encode(writer)?;
        writer.write_all(self.reason_phrase.as_bytes())?;
        encoded_len += self.reason_phrase.as_bytes().len();
        Ok(encoded_len)
    }
}

impl Decode for AnnounceError {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        let mut message_type = [0];
        reader.read_exact(&mut message_type)?;
        if message_type[0] != MESSAGE_TYPE_ANNOUNCE_ERROR {
            return Err(Error::DecodingError("Invalid message type for announce error".to_owned()));
        }

        let track_namespace = TrackNamespace::decode(reader)?;
        let error_code = QUICVarint::decode(reader)?;
        let reason_phrase_len = QUICVarint::decode(reader)?;
        let mut reason_phrase_bytes = vec![0; reason_phrase_len.0 as usize];
        reader.read(&mut reason_phrase_bytes)?;
        let reason_phrase =  String::from_utf8(reason_phrase_bytes).unwrap();
        Ok(AnnounceError {
            track_namespace,
            error_code,
            reason_phrase
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn announce_encode_decode() {
        let announce_before = Announce {
          track_namespace: TrackNamespace(b"http://example.com/meetings/123/user/1".to_vec()),
        };
        let mut enc = Vec::new();
        announce_before.encode(&mut enc).expect("Encoding failed for announce");
        let announce_after = Announce::decode(&mut enc.as_slice())
            .expect("Decoding failed announce");
        assert_eq!(announce_before, announce_after);
        //String::from_utf8(announce_after.track_namespace).expect("Our bytes should be valid utf8");
    }

    #[test]
    fn announce_error_encode_decode() {
        let announce_err_before = AnnounceError {
            track_namespace: TrackNamespace(b"http://example.com/meetings/123/user/1".to_vec()),
            error_code: QUICVarint(1011),
            reason_phrase: "Authorization Failed".to_string(),
        };
        let mut enc = Vec::new();
        announce_err_before.encode(&mut enc).expect("Encoding failed for announce error");
        let announce_err_after = AnnounceError::decode(&mut enc.as_slice())
            .expect("Decoding failed announce error");
        assert_eq!(announce_err_before, announce_err_after);
        //String::from_utf8(announce_after.track_namespace).expect("Our bytes should be valid utf8");
    }

}