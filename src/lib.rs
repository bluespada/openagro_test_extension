use openagro_extension::extension::ExtensionMetadata;


pub async fn metadata() -> ExtensionMetadata {
    ExtensionMetadata {
        extension_id: "".to_string(),
        name: "".to_string(),
        description: None,
        category: None,
        author: "".to_string(),
        version: "1.0.0".to_string(),
        depends_on: vec![],
        application: true,
    }
}