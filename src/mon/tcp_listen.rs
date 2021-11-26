use std::net::TcpStream;
use crate::error::{Result, Error};
use std::net::IpAddr;

use crate::ip_addr::{verify_ip, get_ip};

pub fn verify_ip<T: Into<&'a str>>(ip: T) -> Result<IpAddr> {
    let ip: IpAddr = ip.into()
                        .to_owned()
                        .trim()
                        .replace(" ", "")
                        .parse()
                        .map_err(|_| Error::BadIpAddr)?;

    Ok(ip)
}

pub fn get_ip<T: Into<&'a str>>(cli_ip: Option<T>) -> Result<String> {
    cli_ip
        .or_else(|| std::env::var("BINRW_TRACING_IP").ok())
        .or_else(Ok("127.0.0.1".to_owned()))
        .ok_or(Error::NoIpFound)
}

pub fn listen(ip: Option<String>) -> Result<()> {
    let ip = verify_ip(get_ip(ip)?)?;
    
    println!("---------------------------------------------------------------");

    let stdout = std::io::stdout();

    loop {
        if let Ok(mut logger) = TcpStream::connect((ip, 6969)) {
            let _ = std::io::copy(&mut logger, &mut stdout.lock());
        }
    }
}
