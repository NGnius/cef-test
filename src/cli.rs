use std::path::PathBuf;
use clap::Parser;

/// -WIP- Automated test tool for CEF UIs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// CEF DevTools port
    #[arg(short, long)]
    port: Option<u16>,

    /// CEF DevTools IP address or domain
    #[arg(short, long)]
    address: Option<String>,

    /// Test file(s)
    test: Vec<PathBuf>,
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
