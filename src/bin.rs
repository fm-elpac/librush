//! `ibrus`: ibus component (executable) for pmim-ibus
//!
//! <https://github.com/fm-elpac/pmim-ibus>
#![deny(unsafe_code)]

use env_logger;
use log::debug;
use std::env;
use std::process::ExitCode;

use librush::pmim;

// 编译信息
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// 显示版本信息
fn 版本() {
    let name = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let target = built_info::TARGET;
    let features = built_info::FEATURES_LOWERCASE_STR;
    println!("{} version {} ({}, {})", name, v, target, features);

    // debug
    let git = env!("VERGEN_GIT_DESCRIBE");
    let profile = built_info::PROFILE;
    let time = env!("VERGEN_BUILD_TIMESTAMP");
    let rustc = built_info::RUSTC_VERSION;
    debug!("{} {} {}, {}", git, profile, time, rustc);
}

fn main() -> Result<(), ExitCode> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut flatpak = false;
    // 命令行参数处理
    match env::args().skip(1).next() {
        Some(i) => match i.as_str() {
            "--version" | "--版本" => {
                版本();
                return Ok(());
            }
            "--flatpak" => {
                flatpak = true;
            }
            _ => {}
        },
        _ => {}
    }

    pmim::main(flatpak).unwrap();
    Ok(())
}
