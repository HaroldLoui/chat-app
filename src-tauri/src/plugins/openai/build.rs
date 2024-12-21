const COMMANDS: &[&str] = &["send_message"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}