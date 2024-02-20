use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use super::IBusErr;

/// get the D-Bus (unix socket) address of ibus
pub fn get_ibus_addr() -> Result<String, Box<dyn Error>> {
    // 查找配置文件
    // ~/.config/ibus/bus/*-unix-wayland-0
    let home = env::var("HOME")?;
    let wd = env::var("WAYLAND_DISPLAY")?;
    let 目录: PathBuf = [home, ".config/ibus/bus".to_string()].iter().collect();
    let 文件名结束 = format!("-unix-{}", wd);

    let 检查文件名 = |p: &PathBuf| -> bool {
        p.file_name().map_or(false, |n| {
            n.to_str().map_or(false, |s| s.ends_with(&文件名结束))
        })
    };

    let 文件 = fs::read_dir(目录)?
        .filter_map(|i| i.ok())
        .map(|i| i.path())
        .find(检查文件名)
        .ok_or(IBusErr::new("can not find ibus addr".to_string()))?;

    // 读取配置文件, 获取里面的地址
    // 忽略注释 (`#`)
    let 地址 = fs::read_to_string(文件.clone())?
        .lines()
        .filter(|i| !i.starts_with("#"))
        .find_map(|i| i.strip_prefix("IBUS_ADDRESS=").map(|i| i.to_string()))
        .ok_or(IBusErr::new(format!("can not find ibus addr: {:?}", 文件)))?;

    Ok(地址.to_string())
}
