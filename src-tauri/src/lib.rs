// RigorTRAC is a fully self-contained HTML/JS app (see ../src/index.html).
// This shell just hosts it in a native window; no Rust-side commands are needed.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
