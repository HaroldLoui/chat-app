const COMMANDS: &[&str] = &[
    "send_message",
    "list_api_config",
    "insert_api_config",
    "set_default_config",
    "delete_api_config",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}