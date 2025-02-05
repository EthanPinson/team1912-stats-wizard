//! Utilities for obtaining .wpilog files from a robot
//  2025 Ethan Pinson
use ftp::{types::FileType, *};
use std::io::{Read, Write};

pub type RobotFtpResult<T> = Result<T, FtpError>;

pub struct LogFile {
    pub name: String,
    pub data: Vec<u8>,
}

/// test
pub struct Credentials<'a> {
    pub team_number: u16,
    pub ftp_port: u8,
    
    pub user: &'a str,
    pub password: &'a str
}

pub struct RioFs {
    pub logs: Vec<LogFile>,
    pub team: u16,
    stream: FtpStream
}

impl RioFs {
    pub fn connect(team: u16, creds: Credentials) -> Result<Self, FtpError> {
        let addr: String = format!("roboRIO-{}-frc.local:21", team);

        let mut stream: FtpStream = FtpStream::connect(addr)?;
        stream.login(creds.user, creds.password)?;

        Ok(Self { logs: vec![], team, stream })
    }

    pub fn get_logs(&mut self) -> Result<(), FtpError> {
        let mut xd_list: Vec<String> = vec![];

        let mut lvuser_logs: Vec<String> = self.get_logs_from_dir("/home/lvuser/logs")?;
        xd_list.append(&mut lvuser_logs);

        // dont expect usb stick to be present by default
        let mut usb_logs: Vec<String> = self.get_logs_from_dir("/u/logs").unwrap_or(vec![]);
        xd_list.append(&mut usb_logs);

        Ok(())
    }

    fn get_logs_from_dir(&mut self, dir: &str) -> Result<Vec<String>, FtpError> {
        self.stream.cwd(dir)?;

        let file_list: Vec<String> = self.stream
            .nlst(None)?
            .into_iter()
            .filter(|filename| filename.ends_with(".wpilog"))
            .collect();

        Ok(file_list)
    }
}

/// Represents a connection to a robot FTP server
pub struct Connection {
    stream: FtpStream,
}

impl Connection {
    pub fn new(creds: Credentials) -> Result<Self, FtpError> {
        let addr: String = format!("roboRIO-{}-frc.local:21", creds.team_number);
        Ok(Self { stream: FtpStream::connect(addr)? })
    }

    pub fn get_logs(&mut self, path: &str) -> Result<Vec<LogFile>, FtpError> {
        self.stream.cwd(path)?;
        let log_list: Vec<String> = self.stream.nlst(None)?;

        let mut log_files: Vec<LogFile> = vec![];

        for file_name in log_list {
            let mut idk: LogFile = LogFile {
                name: file_name.clone(),
                data: vec![]
            };

            self.stream.get(&idk.name)?
                .read_to_end(&mut idk.data)
                .map_err(FtpError::ConnectionError)?;
            log_files.push(idk);
        }

        Ok(log_files)
    }
}


pub struct RobotFtpConnection<'a> {
    pub team_number: u16,
    pub user: &'a str,
    pub password: &'a str,

    stream: FtpStream,
    log_list: Vec<String>
}

impl RobotFtpConnection<'_> {
    pub fn connect(&mut self) -> RobotFtpResult<()> {
        let addr: String = format!("roboRIO-{}-frc.local:21", self.team_number);
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
        // return vecs of vec<u8>, with filename taking first 100 bytes
    }
}