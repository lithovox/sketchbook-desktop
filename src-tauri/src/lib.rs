use std::path::Path;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

/// Asks where to save `contents` (a native "Save as" dialog suggesting
/// `file_name`) and writes it there. The chosen path, or None when the user
/// cancelled. Called by sketchbook's saveFile.ts.
#[tauri::command]
async fn save_file(app: tauri::AppHandle, file_name: String, contents: Vec<u8>) -> Result<Option<String>, String> {
  let mut dialog = app.dialog().file().set_file_name(&file_name);
  if let Some(extension) = Path::new(&file_name).extension().and_then(|e| e.to_str()) {
    dialog = dialog.add_filter(extension.to_uppercase(), &[extension]);
  }
  let Some(chosen) = dialog.blocking_save_file() else {
    return Ok(None);
  };
  let path = chosen.into_path().map_err(|e| e.to_string())?;
  std::fs::write(&path, contents).map_err(|e| e.to_string())?;
  Ok(Some(path.display().to_string()))
}

/// Opens `url` (http or https) in the system browser, not in the app's window.
/// Called by sketchbook's website.ts.
#[tauri::command]
fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
  if !url.starts_with("https://") && !url.starts_with("http://") {
    return Err(format!("not a web address: {url}"));
  }
  app.opener().open_url(url, None::<&str>).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![save_file, open_url])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      // The window title gets the version (tauri.conf.json takes it from
      // ../sketchbook/package.json), as the web app's page title: "0.1.0" shows as "0.1".
      if let Some(window) = app.get_webview_window("main") {
        let version = app.package_info().version.to_string();
        let version = version.strip_suffix(".0").unwrap_or(&version);
        window.set_title(&format!("{} v{version}", window.title()?))?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}
