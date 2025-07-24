pub mod api;
pub mod app;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use b3scale_api::{FrontendConfig, FrontendSettings};

    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_frontend_config_creation() {
        let config = FrontendConfig {
            key: "test-key".to_string(),
            secret: "test-secret".to_string(),
        };
        
        assert_eq!(config.key, "test-key");
        assert_eq!(config.secret, "test-secret");
    }

    #[test]
    fn test_frontend_settings_creation() {
        let mut params = HashMap::new();
        params.insert("test".to_string(), "value".to_string());
        
        let settings = FrontendSettings {
            attendees_limit: None,
            create_default_params: params,
            create_override_params: HashMap::new(),
            default_presentation: None,
            required_tags: None,
            recordings: None,
        };
        
        assert!(!settings.create_default_params.is_empty());
        assert!(settings.create_override_params.is_empty());
    }

    #[test]
    fn test_string_operations() {
        let test_str = "b3scale-admin";
        assert!(test_str.contains("admin"));
        assert_eq!(test_str.len(), 13);
    }

    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_api_models_work() {
        // Test that our API models can be created and used
        let config = FrontendConfig {
            key: "bbb-key-123".to_string(),
            secret: "secret-456".to_string(),
        };

        let mut default_params = HashMap::new();
        default_params.insert("record".to_string(), "false".to_string());
        default_params.insert("autoStartRecording".to_string(), "false".to_string());

        let settings = FrontendSettings {
            attendees_limit: None,
            create_default_params: default_params,
            create_override_params: HashMap::new(),
            default_presentation: None,
            required_tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            recordings: None,
        };

        // Test that the structures were created correctly
        assert_eq!(config.key, "bbb-key-123");
        assert_eq!(config.secret, "secret-456");
        assert_eq!(settings.create_default_params.get("record"), Some(&"false".to_string()));
        assert_eq!(settings.required_tags.as_ref().unwrap().len(), 2);
    }
}