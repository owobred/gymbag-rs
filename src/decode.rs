use bytes::Buf;
use prost::Message;

use crate::frame;

pub fn read_gymbag(file: impl Into<std::path::PathBuf>) -> Vec<frame::Frame> {
    let path = file.into();
    let data = bytes::BytesMut::from(&*std::fs::read(path).unwrap());
    let sections = read_gymbag_chunks(data);

    sections.into_iter().map(|section| {
        frame::Frame::decode(&*section).expect("failed to decode frame")
    }).collect::<Vec<_>>()
}

pub fn read_gymbag_chunks(mut raw: bytes::BytesMut) -> Vec<bytes::Bytes> {
    let mut byte_chunks = vec![];

    loop {
        if raw.remaining() < 4 {
            break;
        }

        let length = raw.get_u32_le();  // this is little endian btw
        let data = raw.copy_to_bytes(length as usize);
        byte_chunks.push(data);
    }

    byte_chunks
}