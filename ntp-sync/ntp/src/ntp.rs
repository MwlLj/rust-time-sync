//! Tools to retrieve Internet-time using NTP protocol.

use time::Timespec;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use error;

const NTP_CLIENT: u8 = 3;
const NTP_HEADER_SIZE: usize = 48; // 12 words
const NTP_TO_UNIX_EPOCH: i64 = 2208988800;

const LEAP_SHIFT: i32 = 6;
const VERSION_SHIFT: i32 = 3;

#[derive(Debug)]
pub struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

impl NTPTimestamp {
    pub fn as_timespec(&self) -> Timespec {
        Timespec {
            sec: (self.seconds as i64) - NTP_TO_UNIX_EPOCH,
            nsec: (((self.fraction as f64) / 2f64.powi(32)) / 1e-3) as i32,
        }
    }
}

#[derive(Debug)]
pub struct NTPHeader {
    leap: u8,
    version: u8,
    mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    reference_id: u32,
    reference_timestamp: NTPTimestamp,
    origin_timestamp: NTPTimestamp,
    receive_timestamp: NTPTimestamp,
    pub transmit_timestamp: NTPTimestamp,
}

impl NTPHeader {
    pub fn new() -> NTPHeader {
        NTPHeader {
            leap: 0,
            version: 3,
            mode: NTP_CLIENT,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            reference_id: 0,
            reference_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            origin_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            receive_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            transmit_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, error::Error> {
        let mut vec = Vec::<u8>::new();

        try!(vec.write_u8(self.leap << LEAP_SHIFT | self.version << VERSION_SHIFT | self.mode));
        try!(vec.write_u8(self.stratum));
        try!(vec.write_u8(self.poll));
        try!(vec.write_u8(self.precision));
        try!(vec.write_u32::<BigEndian>(self.root_delay));
        try!(vec.write_u32::<BigEndian>(self.root_dispersion));
        try!(vec.write_u32::<BigEndian>(self.reference_id));
        try!(vec.write_u32::<BigEndian>(self.reference_timestamp.seconds));
        try!(vec.write_u32::<BigEndian>(self.reference_timestamp.fraction));
        try!(vec.write_u32::<BigEndian>(self.origin_timestamp.seconds));
        try!(vec.write_u32::<BigEndian>(self.origin_timestamp.fraction));
        try!(vec.write_u32::<BigEndian>(self.receive_timestamp.seconds));
        try!(vec.write_u32::<BigEndian>(self.receive_timestamp.fraction));
        try!(vec.write_u32::<BigEndian>(self.transmit_timestamp.seconds));
        try!(vec.write_u32::<BigEndian>(self.transmit_timestamp.fraction));
        Ok(vec)
    }

    pub fn decode(size: usize, buf: &[u8]) -> Result<NTPHeader, error::Error> {
        let mut reader = io::Cursor::new(buf);
        let mut header = NTPHeader::new();

        if size < NTP_HEADER_SIZE {
            return Err(error::Error::UnexpectedSize(NTP_HEADER_SIZE, size));
        }

        let leap_version_mode = try!(reader.read_u8());
        header.leap = (leap_version_mode >> LEAP_SHIFT) & 0b11;
        header.version = (leap_version_mode >> VERSION_SHIFT) & 0b111;
        header.mode = leap_version_mode & 0b111;
        header.stratum = try!(reader.read_u8());
        header.poll = try!(reader.read_u8());
        header.precision = try!(reader.read_u8());
        header.root_delay = try!(reader.read_u32::<BigEndian>());
        header.root_dispersion = try!(reader.read_u32::<BigEndian>());
        header.reference_id = try!(reader.read_u32::<BigEndian>());
        header.reference_timestamp.seconds = try!(reader.read_u32::<BigEndian>());
        header.reference_timestamp.fraction = try!(reader.read_u32::<BigEndian>());
        header.origin_timestamp.seconds = try!(reader.read_u32::<BigEndian>());
        header.origin_timestamp.fraction = try!(reader.read_u32::<BigEndian>());
        header.receive_timestamp.seconds = try!(reader.read_u32::<BigEndian>());
        header.receive_timestamp.fraction = try!(reader.read_u32::<BigEndian>());
        header.transmit_timestamp.seconds = try!(reader.read_u32::<BigEndian>());
        header.transmit_timestamp.fraction = try!(reader.read_u32::<BigEndian>());

        Ok(header)
    }
}
