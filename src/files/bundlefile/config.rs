use crate::unity_version::{UnityVersion, UnityVersionType};

pub struct ExtractionConfig {
    pub unitycn_key: Option<[u8; 16]>,
    pub fallback_unity_version: UnityVersion,
}

impl ExtractionConfig {
    pub fn new(unitycn_key: Option<[u8; 16]>, fallback_unity_version: UnityVersion) -> Self {
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
            fallback_unity_version: UnityVersion {
                major: 2,
                minor: 5,
                build: 0,
                typ: UnityVersionType::Final,
                build_number: 5,
            },
        }
    }
}
