use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerActionResult {
    pub ok: bool,
    pub message: String,
}

fn run_shutdown(args: &[&str]) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("shutdown")
            .args(args)
            .output()
            .map_err(|e| format!("无法执行 shutdown 命令: {e}"))?;

        // Windows shutdown 成功时 stdout/stderr 常为空，失败时写 stderr
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output.status.success() {
            let msg = if !stdout.is_empty() {
                stdout
            } else if !stderr.is_empty() {
                // 取消时部分环境也会附带提示文本
                stderr
            } else {
                String::new()
            };
            Ok(msg)
        } else {
            let detail = if !stderr.is_empty() {
                stderr
            } else if !stdout.is_empty() {
                stdout
            } else {
                format!("退出码 {}", output.status.code().unwrap_or(-1))
            };
            Err(detail)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = args;
        Err("当前仅支持 Windows 系统".into())
    }
}

/// 安排定时关机。`seconds` 为倒计时秒数（0–315360000）。
#[tauri::command]
pub fn schedule_shutdown(seconds: u64) -> Result<PowerActionResult, String> {
    if seconds > 315_360_000 {
        return Err("倒计时不能超过 365 天".into());
    }

    let secs = seconds.to_string();
    run_shutdown(&["/s", "/t", &secs, "/c", "ToolKit 定时关机"])?;

    let message = if seconds == 0 {
        "正在关机…".into()
    } else {
        format!("已安排 {seconds} 秒后关机")
    };

    Ok(PowerActionResult { ok: true, message })
}

/// 安排定时重启。
#[tauri::command]
pub fn schedule_restart(seconds: u64) -> Result<PowerActionResult, String> {
    if seconds > 315_360_000 {
        return Err("倒计时不能超过 365 天".into());
    }

    let secs = seconds.to_string();
    run_shutdown(&["/r", "/t", &secs, "/c", "ToolKit 定时重启"])?;

    let message = if seconds == 0 {
        "正在重启…".into()
    } else {
        format!("已安排 {seconds} 秒后重启")
    };

    Ok(PowerActionResult { ok: true, message })
}

/// 取消尚未执行的关机/重启计划。
#[tauri::command]
pub fn cancel_power_action() -> Result<PowerActionResult, String> {
    match run_shutdown(&["/a"]) {
        Ok(msg) => Ok(PowerActionResult {
            ok: true,
            message: if msg.is_empty() {
                "已取消关机/重启计划".into()
            } else {
                msg
            },
        }),
        Err(e) => {
            // 没有待取消任务时 Windows 通常返回错误
            Err(if e.is_empty() {
                "当前没有可取消的关机/重启计划".into()
            } else {
                e
            })
        }
    }
}
