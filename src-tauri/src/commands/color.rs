use serde::Serialize;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
    pub x: i32,
    pub y: i32,
}

fn to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

#[cfg(windows)]
fn sample_at(x: i32, y: i32) -> Result<(u8, u8, u8), String> {
    use windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC};

    unsafe {
        let hdc = GetDC(None);
        if hdc.is_invalid() {
            return Err("无法获取屏幕设备上下文".into());
        }
        let color = GetPixel(hdc, x, y);
        let _ = ReleaseDC(None, hdc);

        // CLR_INVALID == 0xFFFFFFFF
        if color.0 == u32::MAX {
            return Err(format!("无法读取坐标 ({x}, {y}) 的颜色"));
        }

        let r = (color.0 & 0xFF) as u8;
        let g = ((color.0 >> 8) & 0xFF) as u8;
        let b = ((color.0 >> 16) & 0xFF) as u8;
        Ok((r, g, b))
    }
}

#[cfg(not(windows))]
fn sample_at(_x: i32, _y: i32) -> Result<(u8, u8, u8), String> {
    Err("当前仅支持 Windows 屏幕取色".into())
}

#[cfg(windows)]
fn cursor_pos() -> Result<(i32, i32), String> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    let mut point = POINT::default();
    unsafe {
        GetCursorPos(&mut point).map_err(|e| format!("无法获取鼠标位置: {e}"))?;
    }
    Ok((point.x, point.y))
}

#[cfg(not(windows))]
fn cursor_pos() -> Result<(i32, i32), String> {
    Err("当前仅支持 Windows 屏幕取色".into())
}

#[cfg(windows)]
fn left_button_down() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
    unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16 & 0x8000 != 0 }
}

#[cfg(windows)]
fn escape_pressed() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE};
    unsafe { GetAsyncKeyState(VK_ESCAPE.0 as i32) as u16 & 0x8000 != 0 }
}

/// 读取当前鼠标位置下的屏幕颜色。
#[tauri::command]
pub fn get_cursor_color() -> Result<ScreenColor, String> {
    let (x, y) = cursor_pos()?;
    let (r, g, b) = sample_at(x, y)?;
    Ok(ScreenColor {
        r,
        g,
        b,
        hex: to_hex(r, g, b),
        x,
        y,
    })
}

/// 开始屏幕取色：隐藏主窗口，等待下次左键点击取样；Esc 取消。
#[tauri::command]
pub async fn pick_screen_color(app: AppHandle) -> Result<ScreenColor, String> {
    #[cfg(not(windows))]
    {
        let _ = app;
        return Err("当前仅支持 Windows 屏幕取色".into());
    }

    #[cfg(windows)]
    {
        let window = app
            .get_webview_window("main")
            .ok_or_else(|| "找不到主窗口".to_string())?;

        // 先记下是否已跳过任务栏，取色后尽量还原
        let was_visible = window.is_visible().unwrap_or(true);
        let _ = window.hide();

        let result = tauri::async_runtime::spawn_blocking(|| {
            // 避开触发按钮的那次按下
            thread::sleep(Duration::from_millis(180));
            while left_button_down() {
                if escape_pressed() {
                    return Err("已取消取色".into());
                }
                thread::sleep(Duration::from_millis(16));
            }

            // 等待下一次点击或 Esc
            loop {
                if escape_pressed() {
                    return Err("已取消取色".into());
                }
                if left_button_down() {
                    // 稍微等按下稳定后再取
                    thread::sleep(Duration::from_millis(20));
                    let (x, y) = cursor_pos()?;
                    let (r, g, b) = sample_at(x, y)?;
                    // 等到松开，避免把点击带进恢复后的窗口
                    while left_button_down() {
                        thread::sleep(Duration::from_millis(16));
                    }
                    return Ok(ScreenColor {
                        r,
                        g,
                        b,
                        hex: to_hex(r, g, b),
                        x,
                        y,
                    });
                }
                thread::sleep(Duration::from_millis(16));
            }
        })
        .await
        .map_err(|e| format!("取色任务失败: {e}"))?;

        if was_visible {
            let _ = window.set_skip_taskbar(false);
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }

        result
    }
}
