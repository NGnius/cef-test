use serde::{Deserialize, Serialize};

/// WebContent json information retrieved from Chrome DevTools at `http://<IP>:<PORT>/json`
#[derive(Serialize, Deserialize, Debug)]
pub struct WebContent {
    description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    devtools_frontend_url: String,
    id: String,
    title: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: String,
}

impl WebContent {
    /// Get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get url
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get websocket debugger url
    pub fn debug_url(&self) -> &str {
        &self.web_socket_debugger_url
    }

    /// Retrieve WebContent information from CEF instance
    #[allow(clippy::result_large_err)]
    pub fn load_all(domain_name: &str, port: u16) -> Result<Vec<Self>, ureq::Error> {
        Ok(ureq::get(&format!("http://{}:{}/json", domain_name, port))
            .call()?
            .into_json()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retrieve_web_content() {
        let contents = WebContent::load_all(env!("DECK_IP"), 8081).expect("Unable to retrieve inspectable web contents json");
        assert_ne!(contents.len(), 0, "No web contents found!");
        for c in contents {
            println!("{:?}", c);
            assert_ne!(c.id(), "");
            assert_ne!(c.title(), "");
            assert_ne!(c.url(), "");
            assert_ne!(c.debug_url(), "");
        }
    }
}
