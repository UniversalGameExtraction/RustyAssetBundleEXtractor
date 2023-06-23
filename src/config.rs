pub struct ExtractionConfig {
    pub unitycn_key: Option<[u8; 16]>,
    pub fallback_unity_version: String,
}

impl ExtractionConfig {
    pub fn new(unitycn_key: Option<[u8; 16]>, fallback_unity_version: String) -> Self {
        Self {
            unitycn_key,
            fallback_unity_version,
        }
    }

    pub fn with_unitycn_key(mut self, key: [u8; 16]) -> Self {
        self.unitycn_key = Some(key);
        self
    }
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            unitycn_key: None,
            fallback_unity_version: "2.5.0f5".to_owned(),
        }
    }
}
