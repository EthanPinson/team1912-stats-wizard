//! Utilities for obtaining .wpilog files from a robot
use ftp::{types::FileType, *};

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
    let addr: String = format!(
        "roboRIO-{team}-frc.local:{port}",
        team = creds.team_number,
        port = creds.port
    );
    let mut stream: FtpStream = FtpStream::connect(addr)?;
    stream.login(creds.user, creds.password)?;
    Ok(stream)
}

pub fn download_logs(mut stream: FtpStream) -> Result<(), FtpError> {
    stream.transfer_type(FileType::Binary)?;

    let mut log_list: Vec<String> = stream.nlst(Some("/home/lvuser/logs"))?;
    log_list.extend(stream.nlst(Some("/u/logs"))?);

    // todo: download all logs
    Ok(())
}
