mod config;
mod preview;

fn main() {
    // Make sure we don't mess with our system while developing
    std::env::set_var("XDG_CONFIG_HOME", "/tmp/test_xkb");
    std::env::set_var("XKB_DEFAULT_OPTIONS", "custom:wombo,caps:ctrl_modifier");

    // Create an example configuration and save it
    let mut model = config::Model::new();
    model.add_special_key(
        "<AC10>".to_string(),
        vec![
            "semicolon".to_string(),
            "colon".to_string(),
            "b".to_string(),
            "b".to_string(),
            "b".to_string(),
            "b".to_string(),
        ],
    );
    model.add_special_key(
        "<AE05>".to_string(),
        vec![
            "5".to_string(),
            "percent".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
            "0x20ac".to_string(),
        ],
    );
    model.export().unwrap();

    // Run preview
    preview::run_preview();
}
