use std::io::{Read, Write};
use crate::quic_varint::QUICVarint;
use crate::syntax::{Decode, Encode, Error};
use crate::track::FullTrackName;

const MESSAGE_TYPE_SUBSCRIBE:u8 = 0x6;
const MESSAGE_TYPE_SUBSCRIBE_OK:u8 = 0x6;
const MESSAGE_TYPE_SUBSCRIBE_ERROR:u8 = 0x6;
const MESSAGE_TYPE_UNSUBSCRIBE:u8 = 0x6;

#[derive(Debug, Clone, PartialEq)]
pub struct Subscribe {
    track: FullTrackName,
    // TODO(Suhas) Add parameters
}

impl Encode for Subscribe {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        MESSAGE_TYPE_SUBSCRIBE.encode(writer)?;
        self.track.encode(writer)
    }
}

impl Decode for Subscribe {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        // validate the message type
        let mut messge_type = [0];
        reader.read_exact(&mut messge_type)?;
        if messge_type[0] != MESSAGE_TYPE_SUBSCRIBE {
            return Err(Error::DecodingError("Invalid message type for subscribe".to_owned()));
        }

        let tn = FullTrackName::decode(reader)?;
        Ok(Subscribe {
            track: tn,
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SubscribeOk {
    track: FullTrackName,
    track_id: QUICVarint,
    expires: QUICVarint,
}

impl Encode for SubscribeOk {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        MESSAGE_TYPE_SUBSCRIBE_OK.encode(writer)?;
        self.track.encode(writer)?;
        self.track_id.encode(writer)?;
        self.expires.encode(writer)
    }
}

impl Decode for SubscribeOk {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        // validate the message type
        let mut messge_type = [0];
        reader.read_exact(&mut messge_type)?;
        if messge_type[0] != MESSAGE_TYPE_SUBSCRIBE_OK {
            return Err(Error::DecodingError("Invalid message type for subscribe ok".to_owned()));
        }
        let track_id = QUICVarint::decode(reader)?;
        let expires = QUICVarint::decode(reader)?;
        let track = FullTrackName::decode(reader)?;
        Ok(SubscribeOk {
            track,
            track_id,
            expires
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SubscribeError {
    track: FullTrackName,
    error_code: QUICVarint,
    reason_phrase: String,
}

impl Encode for SubscribeError {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let mut encoded_len = MESSAGE_TYPE_SUBSCRIBE_ERROR.encode(writer)?;
        encoded_len += self.track.encode(writer)?;
        encoded_len += self.error_code.encode(writer)?;
        let reason_phrase_qv = QUICVarint(self.reason_phrase.len() as u64);
        encoded_len += reason_phrase_qv.encode(writer)?;
        writer.write_all(self.reason_phrase.as_bytes())?;
        encoded_len += self.reason_phrase.as_bytes().len();
        Ok(encoded_len)
    }
}

impl Decode for SubscribeError {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        let mut message_type = [0];
        reader.read_exact(&mut message_type)?;
        if message_type[0] != MESSAGE_TYPE_SUBSCRIBE_ERROR {
            return Err(Error::DecodingError("Invalid message type for subscribe error".to_owned()));
        }

        let track = FullTrackName::decode(reader)?;
        let error_code = QUICVarint::decode(reader)?;
        let reason_phrase_len = QUICVarint::decode(reader)?;
        let mut reason_phrase_bytes = vec![0; reason_phrase_len.0 as usize];
        reader.read(&mut reason_phrase_bytes)?;
        let reason_phrase =  String::from_utf8(reason_phrase_bytes).unwrap();
        Ok(SubscribeError {
            track,
            error_code,
            reason_phrase
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unsubscribe {
    track: FullTrackName,
}

impl Encode for Unsubscribe {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        MESSAGE_TYPE_UNSUBSCRIBE.encode(writer)?;
        self.track.encode(writer)
    }
}
impl Decode for Unsubscribe {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        // validate the message type
        let mut messge_type = [0];
        reader.read_exact(&mut messge_type)?;
        if messge_type[0] != MESSAGE_TYPE_UNSUBSCRIBE {
            return Err(Error::DecodingError("Invalid message type for unsubscribe".to_owned()));
        }

        let tn = FullTrackName::decode(reader)?;
        Ok(Unsubscribe {
            track: tn,
        })
    }
}