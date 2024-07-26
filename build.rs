use std::error::Error;
use std::path::PathBuf;

use built;
use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    // 每次编译都重新运行 `build.rs`
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(
            &GitclBuilder::default()
                .describe(true, false, None)
                .sha(false)
                .build()?,
        )?
        .emit()?;

    // `.git/index`
    match PathBuf::from("./.git/index").canonicalize() {
        Ok(p) => {
            println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
        }
        _ => {
            println!("cargo:warning=can not find ./.git/index");
        }
    }

    // 收集编译信息
    built::write_built_file()?;

    Ok(())
}
