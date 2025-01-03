const COMMANDS: &[&str] = &[
    "send_message",
    "list_api_config",
    "insert_api_config",
    "set_default_config",
    "delete_api_config",
    "query_enable_stream",
    "update_enable_stream",
    "query_associated_context",
    "update_associated_context",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}