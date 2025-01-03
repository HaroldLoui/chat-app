const COMMANDS: &[&str] = &[
    "query_proxy",
    "check_proxy",
    "update_proxy",
    "enable_proxy"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}