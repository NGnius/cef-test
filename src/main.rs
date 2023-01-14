mod cli;

use simplelog::{LevelFilter, WriteLogger};

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::Cli::parse();
    println!("Got args {:?}", &args);

    let log_filepath = format!("./{}-{}-v{}.log", cef_test_core::util::timestamp_now(), PACKAGE_NAME, PACKAGE_VERSION);

    WriteLogger::init(
        #[cfg(debug_assertions)]
        {
            LevelFilter::Debug
        },
        #[cfg(not(debug_assertions))]
        {
            LevelFilter::Info
        },
        Default::default(),
        std::fs::File::create(&log_filepath).unwrap(),
    ).expect("Couldn't init file log");
}
