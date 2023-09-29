use std::io::{Read, Write};
use crate::quic_varint::QUICVarint;
use crate::syntax::{Decode, Encode, Error};

pub struct Object {
    track_id: QUICVarint,
    group_sequence: QUICVarint,
    object_sequence: QUICVarint,
    priority: QUICVarint,
    payload: Vec<u8>,
}

impl Encode for Object {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        todo!()
    }
}

impl Decode for Object {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> where Self: Sized {
        todo!()
    }
}