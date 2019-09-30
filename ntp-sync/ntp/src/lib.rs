extern crate time;
extern crate byteorder;

mod ntp;
mod error;

use std::net::UdpSocket;
use time::Timespec;
pub use error::Error;
pub use ntp::NTPHeader;

const NTP_PORT: u16 = 123;
const UDP_LOCAL: &'static str = "0.0.0.0:35000";

/// `retrieve_ntp_timestamp` retrieves the current time from a NTP server.
///
/// # Arguments
///
/// * `host` - The NTP server (i.e. sundial.columbia.edu).
pub fn retrieve_ntp_timestamp<'a>(host: &'a str, port: u16, selfAddr: &str) -> Result<Timespec, &'a str> {
    let socket = match UdpSocket::bind(selfAddr) {
        Ok(s) => s,
        Err(err) => {
            return Err("socket bind error");
        }
    };
    socket.set_read_timeout(Some(std::time::Duration::from_secs(30)));
    let host = format!("{host}:{port}", host = host, port = port);
    match socket.connect(&host[..]) {
        Ok(()) => {},
        Err(err) => {
            return Err("connect error");
        }
    }

    let header = NTPHeader::new();
    let message = match header.encode() {
        Ok(m) => m,
        Err(err) => {
            return Err("header encode error");
        }
    };

    match socket.send(&message[..]) {
        Ok(_) => {
        },
        Err(err) => {
            return Err("send to error");
        }
    }

    let mut buf = [0u8; 1000];

    // TODO: Rust doesn't support timeouts yet
    let (amt, _) = match socket.recv_from(&mut buf) {
        Ok(v) => {
            v
        },
        Err(err) => {
            return Err("recv from error");
        }
    };

    drop(socket);

    let header = match ntp::NTPHeader::decode(amt, &buf) {
        Ok(h) => h,
        Err(err) => {
            return Err("decode error");
        }
    };

    Ok(header.transmit_timestamp.as_timespec())
}

#[test]
fn receive_timestamp() {
    const NTP_SERVER: &'static str = "pool.ntp.org";

    let t1 = retrieve_ntp_timestamp(NTP_SERVER).unwrap();
    let t2 = retrieve_ntp_timestamp(NTP_SERVER).unwrap();

    assert!(t2 > t1);
}
