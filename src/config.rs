pub struct ExtractionConfig {
    pub unitycn_key: Option<[u8; 16]>,
    pub fallback_unity_version: String,
}

impl ExtractionConfig {
    pub(crate) fn new() -> Self {
        Self {
            unitycn_key: None,
            fallback_unity_version: "2.5.0f5".to_owned(),
        }
    }

    pub fn with_unitycn_key(mut self, key: [u8; 16]) -> Self {
        self.unitycn_key = Some(key);
        self
    }
}
