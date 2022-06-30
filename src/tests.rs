use crate::config::Model;

// test mapping Ctrl+; to Ctrl+b
// The quest for the best tmux prefix. The mapping that started it all.
#[test]
fn ctrl_semicolon_to_ctrl_b() {
    let mut model = Model::new();
    model.add_special_key("<AC10>", "semicolon colon b b b b");
    model.export().unwrap();
}
