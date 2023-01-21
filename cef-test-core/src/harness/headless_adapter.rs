use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;
use headless_chrome::{Browser, Tab, Element};

use crate::cef::WebContent;
use super::{TestAdapter, TabSelector, ElementSelector, Feedback};

/// Headless Chrome Adapter for CEF
pub struct HeadlessAdapter {
    web_content: Vec<WebContent>,
    connections: HashMap<String, Browser>,
    domain_name: String,
    port_num: u16
}

impl HeadlessAdapter {
    /// Connect DevTools and prepare to connect to the browser
    #[allow(clippy::result_large_err)]
    pub fn connect(domain_name: &str, port: u16) -> Result<Self, ureq::Error> {
        let web_contents = WebContent::load_all(domain_name, port)?;
        // connect to one tab if possible, to give DevTools time to register tabs
        let mut conn_map = HashMap::new();
        if !web_contents.is_empty() {
            if let Ok(browser) = Browser::connect(web_contents[0].debug_url().to_owned()).map_err(|e| format!("{}", e)) {
                if let Ok(version_info) = browser.get_version() {
                    log::info!("CEF Adapter running (protocol {}, product {}, rev {}, js {}, user agent {})", version_info.protocol_version, version_info.product, version_info.revision, version_info.js_version, version_info.user_agent);
                }
                conn_map.insert(web_contents[0].title().to_owned(), browser);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1_000));
        log::info!("HeadlessAdapter ready");
        Ok(Self {
            web_content: web_contents,
            connections: conn_map,
            domain_name: domain_name.to_owned(),
            port_num: port,
        })
    }

    fn tab_title(&self, title: &str) -> Result<Option<Arc<Tab>>, String> {
        if let Some(browser) = self.connections.get(title) {
            for tab in browser.get_tabs().lock().map_err(|e| format!("{}", e))?.iter() {
                if let Ok(info) = tab.get_target_info() {
                    if info.title == title {
                        return Ok(Some(tab.clone()));
                    }
                }
            }
        }
        Ok(None)
    }

    fn tab_web_content(&mut self, tab_select: &TabSelector) -> Result<Option<Arc<Tab>>, String> {
        for web_content in self.web_content.iter() {
            let is_match = match tab_select {
                TabSelector::Title(title) => title == web_content.title(),
                TabSelector::TitleRegex(pattern) => {
                    let pattern = Regex::new(pattern).map_err(|x| x.to_string())?;
                    pattern.is_match(web_content.title())
                },
                TabSelector::Url(url) => url == web_content.url(),
                TabSelector::UrlRegex(pattern) => {
                    let pattern = Regex::new(pattern).map_err(|x| x.to_string())?;
                    pattern.is_match(web_content.url())
                },
                TabSelector::Id(id) => id == web_content.id(),
            };
            if is_match {
                let new_browser = Browser::connect(web_content.debug_url().to_owned()).map_err(|e| format!("{}", e))?;
                self.connections.insert(web_content.title().to_owned(), new_browser);
                return self.tab_title(web_content.title());
            }
        }
        Ok(None)
    }

    fn tab_connection(&mut self, tab_select: &TabSelector) -> Result<Option<Arc<Tab>>, String> {
        for (_title, browser) in self.connections.iter() {
            for tab in browser.get_tabs().lock().map_err(|e| format!("{}", e))?.iter() {
                if let Ok(info) = tab.get_target_info() {
                    let is_match = match tab_select {
                        TabSelector::Url(url) => url == &info.url,
                        TabSelector::Id(id) => {
                            id == &info.target_id
                            || info.opener_id.map(|opener| &opener == id).unwrap_or(false)
                            || info.browser_context_id.map(|ctx_id| &ctx_id == id).unwrap_or(false)
                        },
                        TabSelector::Title(title) => title == &info.title,
                        TabSelector::TitleRegex(pattern) => {
                            let pattern = Regex::new(pattern).map_err(|x| x.to_string())?;
                            pattern.is_match(&info.title)
                        },
                        TabSelector::UrlRegex(pattern) => {
                            let pattern = Regex::new(pattern).map_err(|x| x.to_string())?;
                            pattern.is_match(&info.url)
                        },
                    };
                    if is_match {
                        return Ok(Some(tab.clone()));
                    }
                }
            }
        }
        Ok(None)
    }

    fn select_tab(&mut self, tab: &TabSelector, can_refresh: bool) -> Option<Arc<Tab>> {
        let mut tab_result = None;
        match self.tab_connection(tab) {
            Ok(tab) => tab_result = tab,
            Err(e) => log::warn!("Failed to retrieve tab {} by connections: {}", tab, e),
        }
        if tab_result.is_some() {
            return tab_result;
        }
        match self.tab_web_content(tab) {
            Ok(tab) => tab_result = tab,
            Err(e) => log::warn!("Failed to retrieve tab {} by web content: {}", tab, e),
        }
        if tab_result.is_some() || !can_refresh {
            tab_result
        } else {
            log::info!("Tab not found, refreshing WebContent");
            match WebContent::load_all(&self.domain_name, self.port_num) {
                Ok(content) => {
                    self.web_content = content;
                    self.select_tab(tab, false)
                },
                Err(e) => {
                    log::warn!("Failed to refresh WebContent: {}", e);
                    None
                }
            }
        }
    }

    fn select_element<'a>(&mut self, tab: &'a Tab, element: &ElementSelector) -> Option<Element<'a>> {
        match element {
            ElementSelector::CSS(css) => {
                tab.wait_for_element(css)
                    .map_err(|e| log::error!("Failed to retrieve element {}: {}", element, e))
                    .ok()
            },
        }
    }
}

impl TestAdapter for HeadlessAdapter {
    fn element_click(&mut self, tab_s: &TabSelector, element_s: &ElementSelector) -> Feedback {
        // TODO better feedback
        if let Some(tab) = self.select_tab(tab_s, true) {
            /*if let Some(element) = self.select_element(&tab, element_s) {
                match element.click() {
                    Ok(_) => Feedback::Success,
                    Err(e) => {
                        log::error!("Failed to click on element {}: {}", element_s, e);
                        Feedback::Error
                    }
                }
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }*/
            // FIXME element.click() doesn't actually click
            let result = match element_s {
                ElementSelector::CSS(css) => tab.evaluate(&format!("document.querySelector(\"{}\").click()", css), true),
            };
            match result {
                Ok(_) => Feedback::Success,
                Err(e) => {
                    log::error!("Failed to click on element {}: {}", element_s, e);
                    Feedback::Error
                }
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn element_wait(&mut self, tab_s: &TabSelector, element_s: &ElementSelector) -> Feedback {
        // TODO better feedback
        if let Some(tab) = self.select_tab(tab_s, true) {
            if let Some(_element) = self.select_element(&tab, element_s) {
                // nothing to do -- select_element already waits
                Feedback::Success
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn element_focus(&mut self, tab_s: &TabSelector, element_s: &ElementSelector) -> Feedback {
        // TODO better feedback
        if let Some(tab) = self.select_tab(tab_s, true) {
            if let Some(element) = self.select_element(&tab, element_s) {
                match element.focus() {
                    Ok(_) => Feedback::Success,
                    Err(e) => {
                        log::error!("Failed to click on element {}: {}", element_s, e);
                        Feedback::Error
                    }
                }
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn element_scroll_to(&mut self, tab_s: &TabSelector, element_s: &ElementSelector) -> Feedback {
        // TODO better feedback
        if let Some(tab) = self.select_tab(tab_s, true) {
            if let Some(element) = self.select_element(&tab, element_s) {
                match element.scroll_into_view() {
                    Ok(_) => Feedback::Success,
                    Err(e) => {
                        log::error!("Failed to click on element {}: {}", element_s, e);
                        Feedback::Error
                    }
                }
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn element_value(&mut self, tab_s: &TabSelector, element_s: &ElementSelector) -> Feedback {
        if let Some(tab) = self.select_tab(tab_s, true) {
            if let Some(element) = self.select_element(&tab, element_s) {
                match element.get_inner_text() {
                    Ok(t) => Feedback::Value(t.into()),
                    Err(e) => {
                        log::error!("Failed to get inner text value of element {}: {}", element_s, e);
                        Feedback::Error
                    }
                }
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn element_attribute(&mut self, tab_s: &TabSelector, element_s: &ElementSelector, _attribute: &str) -> Feedback {
        if let Some(tab) = self.select_tab(tab_s, true) {
            if let Some(element) = self.select_element(&tab, element_s) {
                match element.get_attributes() {
                    Ok(Some(_attrs)) => {
                        // TODO
                        //attrs.get(attribute).map(|x| x.into()).unwrap_or(serde_json::Value::Null)
                        Feedback::Unsupported
                    },
                    Ok(None) => Feedback::Value(serde_json::Value::Null),
                    Err(e) => {
                        log::error!("Failed to get attributes of element {}: {}", element_s, e);
                        Feedback::Error
                    }
                }
            } else {
                log::error!("Failed to find element {}", element_s);
                Feedback::Error
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn wait(&mut self, tab_s: &TabSelector, milliseconds: u64) -> Feedback {
        // TODO better feedback
        if let Some(_tab) = self.select_tab(tab_s, true) {
            let duration = std::time::Duration::from_millis(milliseconds);
            std::thread::sleep(duration);
            Feedback::Success
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }

    fn evaluate(&mut self, tab_s: &TabSelector, script: &str) -> Feedback {
        if let Some(tab) = self.select_tab(tab_s, true) {
            match tab.evaluate(script, true) {
                Ok(result) => Feedback::Value(result.value.unwrap_or(serde_json::Value::Null)),
                Err(e) => {
                    log::error!("Failed to evaluate script on tab {}: {}", tab_s, e);
                    Feedback::Error
                }
            }
        } else {
            log::error!("Failed to find tab {}", tab_s);
            Feedback::Error
        }
    }
}
