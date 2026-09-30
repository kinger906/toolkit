use tauri::WebviewWindow;

/// 隐藏窗口并从不在任务栏显示，仅保留系统托盘图标。
#[tauri::command]
pub fn hide_to_tray(window: WebviewWindow) -> Result<(), String> {
    window
        .hide()
        .map_err(|e| format!("隐藏窗口失败: {e}"))?;
    window
        .set_skip_taskbar(true)
        .map_err(|e| format!("隐藏任务栏图标失败: {e}"))?;
    Ok(())
}
