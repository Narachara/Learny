const COMMANDS: &[&str] = &["pick_image", "pick_archive"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
