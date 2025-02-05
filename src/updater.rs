use std::env::consts;

pub struct FrcInstaller {
    
}

impl FrcInstaller {
    pub fn new() -> Self {
        if consts::FAMILY != "windows" {
            // throw error
        }
        Self {  }
    }
}