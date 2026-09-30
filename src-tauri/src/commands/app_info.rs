use serde::Serialize;

#[derive(Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

/// 示例：前端可通过 invoke('get_app_info') 调用。
/// 后续原生能力按模块继续往 commands/ 下加即可。
#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "ToolKit".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    }
}
