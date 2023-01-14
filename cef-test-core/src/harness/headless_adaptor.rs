use std::collections::HashMap;

use headless_chrome::Browser;

use crate::cef::WebContent;

/// Headless Chrome Adaptor for CEF
pub struct HeadlessAdaptor {
    web_content: Vec<WebContent>,
    connections: HashMap<String, Browser>,
    domain_name: String,
    port_num: u16
}

impl HeadlessAdaptor {
    /// Connect DevTools and prepare to connect to the browser
    #[allow(clippy::result_large_err)]
    pub fn connect(domain_name: &str, port: u16) -> Result<Self, ureq::Error> {
        Ok(Self {
            web_content: WebContent::load_all(domain_name, port)?,
            connections: HashMap::new(),
            domain_name: domain_name.to_owned(),
            port_num: port,
        })
    }
}
