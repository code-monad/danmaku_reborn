use crate::gen_authorization_input_string;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper class that using for generating signatures when interacting with bili's open-live api
pub struct Authenticator {
    access_key: String,
    access_secret: String,
    pub hmac: Hmac<Sha256>,
}

impl Authenticator {
    /// Create a new Authenticator
    /// If there are errors in creation, it will be None
    pub fn new(access_key: &str, access_secret: &str) -> Option<Authenticator> {
        if let Ok(hmac) = Hmac::<Sha256>::new_from_slice(access_secret.as_bytes()) {
            Some(Authenticator {
                access_key: access_key.to_string(),
                access_secret: access_secret.to_string(),
                hmac,
            })
        } else {
            None
        }
    }

    /// Set the access key
    /// Get it from https://open-live.bilibili.com/
    pub fn access_key(&mut self, access_key: &str) -> &Authenticator {
        self.access_key = access_key.to_string();
        self
    }

    /// Set the access secret
    /// Get it from https://open-live.bilibili.com/
    pub fn access_secret(&mut self, access_secret: &str) -> &Authenticator {
        self.access_secret = access_secret.to_string();
        self
    }

    /// Returns the access key stored
    pub fn get_access_key(&self) -> &str {
        &self.access_key
    }

    /// Returns the access secret stored
    pub fn get_access_secret(&self) -> &str {
        &self.access_secret
    }

    // Build a map for generating authorization string
    fn _build_map_inner(&self) -> HashMap<&str, String> {
        HashMap::from([
            ("Accept", "application/json".to_string()),
            ("Content-Type", "application/json".to_string()),
            ("x-bili-signature-method", "HMAC-SHA256".to_string()),
            ("x-bili-signature-version", "1.0".to_string()),
            (
                "x-bili-signature-nonce",
                format!("{:016}", rand::thread_rng().gen::<u32>()),
            ),
            (
                "x-bili-timestamp",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
            ),
            ("x-bili-accesskeyid", self.access_key.clone()),
        ])
    }

    /// Generates the signature string for access bilibili's openapi endpoints
    /// Check https://open-live.bilibili.com/document/doc&tool/auth.html#%E7%94%9F%E6%88%90%E7%AD%BE%E5%90%8D
    pub fn signature(&self, header_map: &HashMap<&str, String>) -> String {
        let mut mac = self.hmac.clone();
        mac.update(gen_authorization_input_string(header_map).as_bytes());
        format!("{:x}", mac.finalize().into_bytes())
    }

    /// Generates the contents body requesting for a LiveRoom's danmaku info
    pub fn request_body(room_id: u64) -> String {
        format!("{{room_id: {}}}", room_id)
    }

    /// Get a HashMap that contains all HTTP fields needed to authenticate
    pub fn gen_auth_header(&self) -> HashMap<&str, String> {
        let mut result = self._build_map_inner();
        result.insert("Authorization", self.signature(&result));
        result
    }

    /// Generates the full header, also filled the `x-bili-content-md5` filed
    /// In production, this is always the recommended operation to use
    /// Other functions are more line inner verbose steps
    pub fn build_header(&self, content: &[u8]) -> HashMap<&str, String> {
        let mut result = self.gen_auth_header();
        result.insert("x-bili-content-md5", format!("{:x}", md5::compute(content)));
        result
    }
}
