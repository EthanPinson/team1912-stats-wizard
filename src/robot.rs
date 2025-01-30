//! Utilities for obtaining .wpilog files from a robot
use ftp::{types::FileType, *};
use std::io::Read;

/// Credentials to locate and connect to a robot
pub struct Credentials<'a> {
    pub team_number: u16,
    pub port: u8,
    pub user: &'a str,
    pub password: &'a str,
}

impl<'a> Default for Credentials<'a> {
    fn default() -> Self {
        Credentials {
            team_number: 1912,
            port: 21,
            user: "lvuser",
            password: "",
        }
    }
}

pub fn connect_to_robot(creds: Credentials) -> Result<FtpStream, FtpError> {
    let addr: String = format!("roboRIO-{t}-frc.local:{p}", t = creds.team_number, p = creds.port);
    let mut stream: FtpStream = FtpStream::connect(addr)?;
    stream.login(creds.user, creds.password)?;
    Ok(stream)
}

pub fn get_logs(mut stream: FtpStream) -> Result<Vec<String>, FtpError> {
    stream.transfer_type(FileType::Binary)?;
    stream.cwd("/home/lvuser/logs")?;

    Ok(stream.nlst(None)?)
}

pub fn download_logs(mut stream: FtpStream, log_list: Vec<String>, to_dir: &str) -> Result<(), FtpError> {
    let lmao: Vec<&str> = log_list.iter().map(String::as_str).collect();
    for file_name in lmao {
        let mut buf: Vec<u8> = vec![];
        stream.get(file_name)?.read_to_end(&mut buf).map_err(FtpError::ConnectionError)?;
    }
    Ok(())
}