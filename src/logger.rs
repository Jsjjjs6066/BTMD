use std::fs::{File, OpenOptions, create_dir};
use std::io::{Result, Write};
use std::path::Path;
use std::sync::{LazyLock, RwLock};

use crate::element::Element;

static LOG_DIR: &str = ".btmd_log";
static LOG_FILE_NAME: &str = "debug.log";
static PAGE_LOG_FILE_NAME: &str = "page_debug.log";

static LOG_FILE: LazyLock<RwLock<Result<File>>> = LazyLock::new(|| {
    create_dir(LOG_DIR).unwrap_or_default();
    RwLock::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .read(true)
            .create(true)
            .open(Path::new(LOG_DIR).join(Path::new(LOG_FILE_NAME))),
    )
});

static PAGE_LOG_FILE: LazyLock<RwLock<Result<File>>> = LazyLock::new(|| {
    create_dir(LOG_DIR).unwrap_or_default();
    RwLock::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .read(true)
            .open(Path::new(LOG_DIR).join(Path::new(PAGE_LOG_FILE_NAME))),
    )
});

pub fn write_log(s: &[u8]) -> Result<()> {
    LOG_FILE.write().unwrap().as_mut().unwrap().write_all(s)?;
    LOG_FILE.write().unwrap().as_mut().unwrap().write_all("\n".as_bytes())?;
    Ok(())
}

pub fn write_page(page_body: &Vec<Element>) -> Result<()> {
    PAGE_LOG_FILE
        .write()
        .unwrap()
        .as_mut()
        .unwrap()
        .write_all(format!("{:#?}", page_body).as_bytes())?;
    Ok(())
}
