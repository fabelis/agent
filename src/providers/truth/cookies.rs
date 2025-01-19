use std::collections::HashMap;

#[derive(Clone)]
pub struct Cookies {
    pub cookies: HashMap<String, String>,
}

impl Cookies {
    pub fn new() -> Self {
        Cookies {
            cookies: HashMap::new(),
        }
    }

    pub fn format(&self) -> String {
        let mut cookie_str = String::new();
        for (key, value) in &self.cookies {
            if !cookie_str.is_empty() {
                cookie_str.push_str("; ");
            }
            cookie_str.push_str(&format!("{}={}", key, value));
        }
        cookie_str
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cookies.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.cookies.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.cookies.remove(&key);
    }

    pub fn clear(&mut self) {
        self.cookies.clear();
    }
}
