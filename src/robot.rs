//! Utilities for obtaining .wpilog files from a robot
use ftp::{types::FileType, *};
use std::io::{Read, Write};

pub type RobotFtpResult<T> = Result<T, FtpError>;

pub struct RobotFtpConnection<'a> {
    pub team_number: u16,
    pub ftp_port: u8,
    pub user: &'a str,
    pub password: &'a str,

    stream: FtpStream,
    log_list: Vec<String>
}

impl RobotFtpConnection<'_> {
    pub fn connect(&mut self) -> RobotFtpResult<()> {
        let addr: String = format!("roboRIO-{t}-frc.local:{p}", t = self.team_number, p = self.ftp_port);
        self.stream = FtpStream::connect(addr)?;
        self.stream.login(self.user, self.password)?;
        self.stream.transfer_type(FileType::Binary)?;
        Ok(())
    }

    pub fn get_logs(&mut self) -> RobotFtpResult<()> {
        self.stream.cwd("/home/lvuser/logs")?;
        self.log_list = self.stream.nlst(None)?;
        Ok(())
    }

    pub fn download_logs(&mut self, to_dir: &str) -> RobotFtpResult<()> {
        let lmao: Vec<&str> = self.log_list.iter().map(String::as_str).collect();
        for file_name in lmao {
            let mut buf: Vec<u8> = vec![];
            self.stream.get(file_name)?.read_to_end(&mut buf).map_err(FtpError::ConnectionError)?;
        }
        Ok(())
        // maybe just return vec of u8 from this function, then download elsewhere?
    }
}