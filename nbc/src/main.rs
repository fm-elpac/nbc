#![deny(unsafe_code)]

use std::process::ExitCode;

use env_logger;
use log::info;

// TODO

fn main() -> Result<(), ExitCode> {
    // 默认日志级别为信息 (INFO)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // TODO
    info!("666");

    Ok(())
}
