use rust_parse::cmd::CCmd;
use time::{Timespec, at};
use ntpclient::retrieve_ntp_timestamp;
use ntp_sync::settime;

use std::thread;
use std::time::Duration;

fn main() {
    let mut cmdHandler = CCmd::new();
    let serverIp = cmdHandler.register_with_desc("-server-ip", "127.0.0.1", "ntp server ip");
    let serverPort = cmdHandler.register_with_desc("-server-port", "123", "ntp server port");
    let localPort = cmdHandler.register_with_desc("-local-port", "35000", "local udp port bind");
    let syncTime = cmdHandler.register_with_desc("-sync-time", "600", "sync interval time, unit: s");
    cmdHandler.parse();

    let serverIp = serverIp.borrow();
    let serverPort = serverPort.borrow();
    let localPort = localPort.borrow();
    let syncTime = syncTime.borrow();

    let serverPort = match serverPort.parse::<u16>() {
        Ok(p) => p,
        Err(err) => {
            println!("[Error] server port is invalid, err: {}", err);
            return;
        }
    };
    let localPort = match localPort.parse::<u16>() {
        Ok(p) => p,
        Err(err) => {
            println!("[Error] local port is invalid, err: {}", err);
            return;
        }
    };
    let syncTime = match syncTime.parse::<u64>() {
        Ok(t) => t,
        Err(err) => {
            println!("[Error] synctime is invalid, err: {}", err);
            return;
        }
    };

    let mut localAddr = String::new();
    localAddr.push_str("0.0.0.0");
    localAddr.push_str(":");
    localAddr.push_str(&localPort.to_string());

    let mut interval: u64 = syncTime;
    loop {
        match retrieve_ntp_timestamp(&*serverIp, serverPort, &localAddr) {
            Ok(t) => {
                interval = syncTime;
                println!("{:?}", &t);
                let t = at(t);
                let ts = t.to_timespec();
                settime::set_system_time(ts.sec as i64, (ts.nsec * 1000) as i64);
            },
            Err(err) => {
                interval = 10;
                println!("[Warning] get time error, err: {}", err);
            }
        };
        println!("sleep time: {}", interval);
        thread::sleep(Duration::from_secs(interval));
    }
}
