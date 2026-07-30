use log::{debug, info};
use simplelog::*;
use std::fs::File;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("actual-test.log").unwrap(),
        )
            .unwrap();
    });
}