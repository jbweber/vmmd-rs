use log::error;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;
use vmmd::libvirt::hooks::qemu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = File::create("/var/log/libvirt_hook_qemu.log")?;
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), log_file)?;

    match qemu() {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("error occured: {}", err);
            return Err(err);
        }
    }
}
