//! ibus module for pmim
//!
//! <https://github.com/fm-elpac/pmim>
use std::error::Error;

use log::{debug, info};
use tokio::{
    runtime::Runtime,
    time::{sleep, Duration},
};

use crate::ibus::get_ibus_addr;
use crate::ibus::IBus;

pub mod engine;
mod server;

use engine::PmimFactory;

pub fn main() -> Result<(), Box<dyn Error>> {
    debug!("init");

    let 地址 = get_ibus_addr()?;
    info!("ibus addr: {}", 地址);

    let rt = Runtime::new()?;
    rt.block_on(async {
        let s = server::初始化pmims().await?;

        let 名称 = "org.fm_elpac.pmim";
        let _b = IBus::new(地址, PmimFactory::new(s), 名称.to_string()).await?;

        info!("初始化完毕");

        loop {
            sleep(Duration::from_secs(10)).await;
        }
        //Ok(())
    })
}
