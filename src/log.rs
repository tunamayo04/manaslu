use log::LevelFilter;
use std::fs::File;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        fern::Dispatch::new()
            .level(LevelFilter::Debug)
            .format(|out, message, _record| {
                out.finish(format_args!("{}", message));
            })
            .chain(File::create("actual-test.log").unwrap())
            .apply()
            .unwrap();
    });
}