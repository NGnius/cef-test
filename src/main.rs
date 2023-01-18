mod cli;

use simplelog::{LevelFilter, WriteLogger, TermLogger, CombinedLogger};

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), String> {
    let args = cli::Cli::parse();
    println!("Got args {:?}", &args);

    let log_filepath = format!("./{}-{}-v{}.log", cef_test_core::util::timestamp_now(), PACKAGE_NAME, PACKAGE_VERSION);

    CombinedLogger::init(vec![
        WriteLogger::new(
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
        ),
        TermLogger::new(
            #[cfg(debug_assertions)]
            {
                LevelFilter::Debug
            },
            #[cfg(not(debug_assertions))]
            {
                LevelFilter::Info
            },
            Default::default(),
            Default::default(),
            simplelog::ColorChoice::Auto,
        )
    ]).expect("Couldn't start log");

    let (addr, port) = if let Some(addr) = args.address {
        if let Some(port) = args.port {
            (addr, port)
        } else if addr.contains("localhost") || addr.contains("127.0.0."){
            (addr, 8080)
        } else {
            (addr, 8081)
        }
    } else {
        if let Some(port) = args.port {
            ("localhost".into(), port)
        } else {
            ("localhost".into(), 8080)
        }
    };

    log::info!("Initializing test adapter");
    let adapter = cef_test_core::harness::HeadlessAdapter::connect(&addr, port).map_err(|e| e.to_string())?;

    log::info!("Initializing test runners");
    let mut runners = Vec::with_capacity(args.test.len());

    for test_file in args.test {
        runners.push(cef_test_core::harness::JsonRunner::from_file(test_file).map_err(|e| e.to_string())?);
    }
    log::info!("Initializing test harness");
    let harness = cef_test_core::harness::TestHarness::new(adapter, runners);

    log::info!("Starting test harness");
    if let Err(errs) = harness.execute() {
        Err(format!("{} tests failed.", errs.len()))
    } else {
        Ok(())
    }
}
