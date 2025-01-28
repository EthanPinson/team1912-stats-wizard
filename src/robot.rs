use chrono::DateTime;
use ftp::types::FileType;
use ftp::{FtpError, FtpStream};

pub struct Credentials<'a> {
    pub team_number: u16, // 1912
    pub user: &'a str, // lvuser
    pub password: &'a str // none
}

pub fn connect_to_robot(creds: Credentials) -> Result<FtpStream, FtpError> {
    let addr: String = format!("roboRIO-{:?}-frc.local", creds.team_number);
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
