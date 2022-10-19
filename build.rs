use std::{
    env,
    path::Path,
    process::{self},
};

use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    println!("Building...");
    #[cfg(windows)]
    pub const YARN: &'static str = "yarn.cmd";

    #[cfg(not(windows))]
    pub const YARN: &'static str = "yarn";

    let cwd = Path::new("./src/front-end/");
    env::set_current_dir(cwd).ok();
    let _ = process::Command::new(YARN).arg("install").status();
    let _ = process::Command::new(YARN).arg("build").status();

    let _ = resource_dir(
        "/Users/shafiquejamal/projects/rust/learn-actix/learn-actix-web/src/front-end/build",
    )
    .build();
    Ok(())
}
