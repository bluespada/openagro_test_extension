use openagro_extension::extension::ExtensionMetadata;


pub fn metadata() -> ExtensionMetadata {
    ExtensionMetadata {
        extension_id: "openagro.test.extension".to_string(),
        name: "Test Extension".to_string(),
        description: None,
        category: None,
        author: "bluespada".to_string(),
        version: "1.0.0".to_string(),
        depends_on: vec![],
        application: true,
    }
}