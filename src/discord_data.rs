use std::error::Error;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use interprocess::local_socket::LocalSocketStream;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    HandShake = 0,
    Frame = 1,
    Close = 2,
    Ping = 3,
    Pong = 4,
}

#[derive(Debug)]
pub struct DiscordData {
    opcode: OpCode,
    length: u32,
    data: String,
}

impl DiscordData {
    pub fn from(opcode: OpCode, data: &str) -> DiscordData {
        DiscordData {
            opcode,
            length: data.len() as u32,
            data: String::from(data),
        }
    }

    pub fn read_from(stream: &mut LocalSocketStream) -> Result<DiscordData, Box<dyn Error>> {
        let mut header_buff = [0u8;8];
        stream.read(&mut header_buff)?;

        let mut rdr = Cursor::new(&header_buff[0..4]);
        let opcode: OpCode = match rdr.read_u32::<NativeEndian>()? {
            0 => OpCode::HandShake,
            1 => OpCode::Frame,
            2 => OpCode::Close,
            3 => OpCode::Ping,
            4 => OpCode::Pong,
            _ => OpCode::HandShake, // I'm lazy
        };
        let mut rdr = Cursor::new(&header_buff[4..8]);
        let length = rdr.read_u32::<NativeEndian>()? as usize;

        let mut data_buff: Vec<u8> = vec![0; length];
        stream.read(&mut data_buff)?;
        let data = String::from_utf8(data_buff)?;

        Ok(DiscordData {
            opcode,
            length: length as u32,
            data,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        let mut opcode = vec![];
        opcode.write_u32::<NativeEndian>(self.opcode as u32).unwrap();
        ret.append(&mut opcode);

        let mut length = vec![];
        length.write_u32::<NativeEndian>(self.length).unwrap();
        ret.append(&mut length);

        let mut data = self.data.as_bytes().to_vec();
        ret.append(&mut data);

        ret
    }
}
