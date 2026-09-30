use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::download::auto_download;
use ffmpeg_sidecar::event::{FfmpegEvent, LogLevel};
use ffmpeg_sidecar::version::ffmpeg_version_with_path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FfmpegStatus {
    pub ready: bool,
    pub path: String,
    pub version: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub duration_secs: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRequest {
    pub input: String,
    pub output: String,
    pub start_secs: Option<f64>,
    pub end_secs: Option<f64>,
    pub bitrate_kbps: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResult {
    pub output: String,
    pub message: String,
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("无法获取配置目录: {e}"))?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("无法创建配置目录: {e}"))?;
    }
    Ok(dir.join("ffmpeg_path.txt"))
}

fn load_saved_path(app: &AppHandle) -> Option<PathBuf> {
    let path = config_path(app).ok()?;
    let text = fs::read_to_string(path).ok()?;
    let p = PathBuf::from(text.trim());
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

fn save_path(app: &AppHandle, ffmpeg: &Path) -> Result<(), String> {
    let path = config_path(app)?;
    fs::write(&path, ffmpeg.to_string_lossy().as_bytes())
        .map_err(|e| format!("保存 FFmpeg 路径失败: {e}"))
}

fn looks_like_ffmpeg(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    Command::new(path)
        .arg("-version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn common_candidate_paths() -> Vec<PathBuf> {
    let mut list = Vec::new();

    // PATH / 当前工作目录
    list.push(PathBuf::from("ffmpeg"));
    list.push(PathBuf::from("ffmpeg.exe"));

    // 环境变量
    if let Ok(p) = std::env::var("FFMPEG_PATH") {
        list.push(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("FFMPEG_BINARY") {
        list.push(PathBuf::from(p));
    }

    // 常见安装位置
    let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let program_files_x86 =
        std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".into());
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let user = std::env::var("USERPROFILE").unwrap_or_default();

    let extras = [
        format!(r"{program_files}\Jellyfin\Server\ffmpeg.exe"),
        format!(r"{program_files_x86}\VeryCapture\ffmpeg.exe"),
        format!(r"{program_files}\ffmpeg\bin\ffmpeg.exe"),
        format!(r"{program_files_x86}\ffmpeg\bin\ffmpeg.exe"),
        r"C:\ffmpeg\bin\ffmpeg.exe".to_string(),
        format!(r"{local}\Microsoft\WinGet\Links\ffmpeg.exe"),
        format!(r"{local}\Programs\ffmpeg\bin\ffmpeg.exe"),
        format!(r"{user}\scoop\apps\ffmpeg\current\bin\ffmpeg.exe"),
        r"C:\ProgramData\chocolatey\bin\ffmpeg.exe".to_string(),
        // 同机其他项目曾下载过的 sidecar
        r"C:\my-git\stream-tool\src-tauri\target\release\ffmpeg.exe".to_string(),
        r"C:\my-git\stream-tool\src-tauri\target\debug\ffmpeg.exe".to_string(),
    ];
    list.extend(extras.into_iter().map(PathBuf::from));

    // WinGet 包目录浅扫
    let winget_pkgs = PathBuf::from(&local).join(r"Microsoft\WinGet\Packages");
    if winget_pkgs.is_dir() {
        if let Ok(entries) = fs::read_dir(&winget_pkgs) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name.contains("ffmpeg") {
                    let candidate = entry.path().join("ffmpeg.exe");
                    list.push(candidate.clone());
                    // 有的包在子目录
                    if let Ok(sub) = fs::read_dir(entry.path()) {
                        for s in sub.flatten().take(8) {
                            list.push(s.path().join("ffmpeg.exe"));
                            list.push(s.path().join("bin").join("ffmpeg.exe"));
                        }
                    }
                }
            }
        }
    }

    list
}

fn resolve_ffmpeg(app: &AppHandle) -> Option<PathBuf> {
    if let Some(saved) = load_saved_path(app) {
        if looks_like_ffmpeg(&saved) {
            return Some(saved);
        }
    }

    // sidecar 旁路（ffmpeg-sidecar 默认位置）
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sidecar = dir.join("ffmpeg.exe");
            if looks_like_ffmpeg(&sidecar) {
                return Some(sidecar);
            }
            let sidecar_unix = dir.join("ffmpeg");
            if looks_like_ffmpeg(&sidecar_unix) {
                return Some(sidecar_unix);
            }
        }
    }

    for candidate in common_candidate_paths() {
        if candidate.as_os_str() == "ffmpeg" || candidate.as_os_str() == "ffmpeg.exe" {
            // PATH 探测
            if Command::new(&candidate)
                .arg("-version")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                return Some(candidate);
            }
            continue;
        }
        if looks_like_ffmpeg(&candidate) {
            return Some(candidate);
        }
    }

    None
}

fn companion_ffprobe(ffmpeg: &Path) -> Option<PathBuf> {
    let dir = ffmpeg.parent()?;
    for name in ["ffprobe.exe", "ffprobe"] {
        let p = dir.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn status_from_path(path: PathBuf) -> FfmpegStatus {
    let path_str = path.display().to_string();
    match ffmpeg_version_with_path(&path) {
        Ok(version) => FfmpegStatus {
            ready: true,
            path: path_str,
            version: Some(version.clone()),
            message: format!("FFmpeg {version} 已就绪"),
        },
        Err(e) => FfmpegStatus {
            ready: false,
            path: path_str,
            version: None,
            message: format!("FFmpeg 无法运行: {e}"),
        },
    }
}

#[tauri::command]
pub fn get_ffmpeg_status(app: AppHandle) -> FfmpegStatus {
    match resolve_ffmpeg(&app) {
        Some(path) => {
            // 自动记住扫到的路径，下次秒开
            let _ = save_path(&app, &path);
            status_from_path(path)
        }
        None => FfmpegStatus {
            ready: false,
            path: String::new(),
            version: None,
            message: "未检测到 FFmpeg。可「选择本机 FFmpeg」或尝试自动下载".into(),
        },
    }
}

/// 手动指定 ffmpeg.exe 路径并保存。
#[tauri::command]
pub fn set_ffmpeg_path(app: AppHandle, path: String) -> Result<FfmpegStatus, String> {
    let p = PathBuf::from(path.trim());
    if !looks_like_ffmpeg(&p) {
        return Err("所选文件不是可用的 ffmpeg 可执行文件".into());
    }
    save_path(&app, &p)?;
    Ok(status_from_path(p))
}

/// 先扫描本机常见位置；找不到再尝试自动下载（可能因网络卡住）。
#[tauri::command]
pub async fn ensure_ffmpeg(app: AppHandle) -> Result<FfmpegStatus, String> {
    if let Some(path) = resolve_ffmpeg(&app) {
        let _ = save_path(&app, &path);
        return Ok(status_from_path(path));
    }

    tauri::async_runtime::spawn_blocking(|| {
        auto_download().map_err(|e| {
            format!(
                "自动下载失败（常因 GitHub 网络问题）: {e}。请改用「选择本机 FFmpeg」，例如 Jellyfin 自带的 ffmpeg.exe"
            )
        })
    })
    .await
    .map_err(|e| format!("任务失败: {e}"))??;

    // 下载后再次解析
    match resolve_ffmpeg(&app) {
        Some(path) => {
            let _ = save_path(&app, &path);
            Ok(status_from_path(path))
        }
        None => Err("下载完成但仍未找到 FFmpeg，请手动选择 ffmpeg.exe".into()),
    }
}

#[tauri::command]
pub async fn probe_media_duration(app: AppHandle, path: String) -> Result<ProbeResult, String> {
    let ffmpeg = resolve_ffmpeg(&app).ok_or_else(|| "请先准备或选择 FFmpeg".to_string())?;
    tauri::async_runtime::spawn_blocking(move || probe_duration_blocking(&ffmpeg, &path))
        .await
        .map_err(|e| format!("任务失败: {e}"))?
}

fn probe_duration_blocking(ffmpeg: &Path, path: &str) -> Result<ProbeResult, String> {
    if !Path::new(path).is_file() {
        return Err("文件不存在".into());
    }

    if let Some(ffprobe) = companion_ffprobe(ffmpeg) {
        let output = Command::new(&ffprobe)
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                path,
            ])
            .output()
            .map_err(|e| format!("无法运行 ffprobe: {e}"))?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(duration_secs) = text.parse::<f64>() {
                if duration_secs.is_finite() && duration_secs >= 0.0 {
                    return Ok(ProbeResult { duration_secs });
                }
            }
        }
    }

    let output = Command::new(ffmpeg)
        .args(["-hide_banner", "-i", path])
        .output()
        .map_err(|e| format!("无法运行 ffmpeg: {e}"))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    parse_duration_from_ffmpeg_log(&stderr)
        .map(|duration_secs| ProbeResult { duration_secs })
        .ok_or_else(|| "无法解析媒体时长".into())
}

fn parse_duration_from_ffmpeg_log(log: &str) -> Option<f64> {
    for line in log.lines() {
        if let Some(idx) = line.find("Duration:") {
            let rest = line[idx + "Duration:".len()..].trim();
            let token = rest.split(',').next()?.trim();
            return parse_hms(token);
        }
    }
    None
}

fn parse_hms(text: &str) -> Option<f64> {
    let parts: Vec<&str> = text.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hours: f64 = parts[0].parse().ok()?;
    let minutes: f64 = parts[1].parse().ok()?;
    let seconds: f64 = parts[2].parse().ok()?;
    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

#[tauri::command]
pub async fn convert_video_to_mp3(
    app: AppHandle,
    req: ConvertRequest,
) -> Result<ConvertResult, String> {
    let ffmpeg = resolve_ffmpeg(&app).ok_or_else(|| "请先准备或选择 FFmpeg".to_string())?;
    tauri::async_runtime::spawn_blocking(move || convert_blocking(&ffmpeg, req))
        .await
        .map_err(|e| format!("任务失败: {e}"))?
}

fn convert_blocking(ffmpeg: &Path, req: ConvertRequest) -> Result<ConvertResult, String> {
    if !Path::new(&req.input).is_file() {
        return Err("输入文件不存在".into());
    }
    if req.output.trim().is_empty() {
        return Err("请指定输出路径".into());
    }

    let start = req.start_secs.filter(|v| *v > 0.0);
    let end = req.end_secs.filter(|v| *v > 0.0);

    let duration = match (start, end) {
        (Some(s), Some(e)) => {
            if e <= s {
                return Err("结束时间必须大于开始时间".into());
            }
            Some(e - s)
        }
        (None, Some(e)) => Some(e),
        _ => None,
    };

    let bitrate = req.bitrate_kbps.unwrap_or(192).clamp(64, 320);

    if let Some(parent) = Path::new(&req.output).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("无法创建输出目录: {e}"))?;
        }
    }

    let mut cmd = FfmpegCommand::new_with_path(ffmpeg);
    cmd.hide_banner().input(&req.input);

    if let Some(s) = start {
        cmd.seek(format!("{s:.3}"));
    }
    if let Some(d) = duration {
        cmd.duration(format!("{d:.3}"));
    }

    cmd.no_video()
        .codec_audio("libmp3lame")
        .args(["-b:a", &format!("{bitrate}k")])
        .output(&req.output)
        .overwrite();

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("启动 ffmpeg 失败: {e}"))?;

    let mut fatal = Vec::new();
    let iter = child
        .iter()
        .map_err(|e| format!("读取 ffmpeg 输出失败: {e}"))?;

    for event in iter {
        match event {
            FfmpegEvent::Error(msg) | FfmpegEvent::Log(LogLevel::Fatal, msg) => {
                fatal.push(msg);
            }
            FfmpegEvent::Log(LogLevel::Error, msg) => {
                if msg.contains("Error") || msg.contains("Invalid") {
                    fatal.push(msg);
                }
            }
            _ => {}
        }
    }

    if !Path::new(&req.output).is_file() {
        let detail = if fatal.is_empty() {
            "转换失败，未生成输出文件".into()
        } else {
            fatal.join(" | ")
        };
        return Err(detail);
    }

    let range = match (start, duration) {
        (Some(s), Some(d)) => format!("（截取 {s:.1}s 起，时长 {d:.1}s）"),
        (None, Some(d)) => format!("（截取前 {d:.1}s）"),
        (Some(s), None) => format!("（从 {s:.1}s 截到结尾）"),
        _ => String::new(),
    };

    Ok(ConvertResult {
        output: req.output,
        message: format!("转换完成{range}"),
    })
}
