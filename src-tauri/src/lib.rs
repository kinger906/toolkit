mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::app_info::get_app_info,
            commands::power::schedule_shutdown,
            commands::power::schedule_restart,
            commands::power::cancel_power_action,
            commands::media::get_ffmpeg_status,
            commands::media::set_ffmpeg_path,
            commands::media::ensure_ffmpeg,
            commands::media::probe_media_duration,
            commands::media::convert_video_to_mp3,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
