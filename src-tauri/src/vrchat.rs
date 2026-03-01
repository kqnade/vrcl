use crate::config::{Profile, ProfileOptions};
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn detect_vrchat_path() -> Result<String, String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_key = hklm
        .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")
        .map_err(|e| format!("Steam レジストリキーが見つかりません: {}", e))?;

    let install_path: String = steam_key
        .get_value("InstallPath")
        .map_err(|e| format!("Steam インストールパスが見つかりません: {}", e))?;

    // デフォルトの steamapps を最初にチェック
    let default_vrchat = format!(
        "{}\\steamapps\\common\\VRChat\\VRChat.exe",
        install_path
    );
    if std::path::Path::new(&default_vrchat).exists() {
        return Ok(default_vrchat);
    }

    // libraryfolders.vdf を解析して全ライブラリを確認
    let vdf_path = format!("{}\\steamapps\\libraryfolders.vdf", install_path);
    let content = std::fs::read_to_string(&vdf_path)
        .map_err(|e| format!("libraryfolders.vdf の読み込みに失敗: {}", e))?;

    for path in parse_library_paths(&content) {
        let vrchat_exe = format!("{}\\steamapps\\common\\VRChat\\VRChat.exe", path);
        if std::path::Path::new(&vrchat_exe).exists() {
            return Ok(vrchat_exe);
        }
    }

    Err("いずれの Steam ライブラリにも VRChat が見つかりませんでした".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn detect_vrchat_path() -> Result<String, String> {
    Err("VRChat の自動検出は Windows のみサポートされています".to_string())
}

/// libraryfolders.vdf から "path" の値を全て抽出する
fn parse_library_paths(content: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with("\"path\"") {
            continue;
        }
        // "path"\t\t"C:\\Games\\Steam" の形式
        // 最後のダブルクォートのペアを抽出
        if let Some(last_quote_end) = line.rfind('"') {
            let before = &line[..last_quote_end];
            if let Some(last_quote_start) = before.rfind('"') {
                let path = &line[last_quote_start + 1..last_quote_end];
                let path = path.replace("\\\\", "\\");
                if !path.is_empty() {
                    paths.push(path);
                }
            }
        }
    }
    paths
}

pub fn build_args(options: &ProfileOptions) -> Vec<String> {
    let mut args = Vec::new();

    if options.no_vr {
        args.push("--no-vr".to_string());
    }
    if options.enable_debug_gui {
        args.push("--enable-debug-gui".to_string());
    }
    if options.enable_udon_debug_logging {
        args.push("--enable-udon-debug-logging".to_string());
    }
    if options.enable_sdk_log_levels {
        args.push("--enable-sdk-log-levels".to_string());
    }
    if options.disable_hw_video_decoding {
        args.push("--disable-hw-video-decoding".to_string());
    }
    if let Some(fps) = options.fps {
        args.push(format!("--fps={}", fps));
    }
    if let Some(width) = options.screen_width {
        args.push(format!("-screen-width={}", width));
    }
    if let Some(height) = options.screen_height {
        args.push(format!("-screen-height={}", height));
    }
    if let Some(fullscreen) = options.fullscreen {
        if fullscreen {
            args.push("-fullscreen".to_string());
        } else {
            args.push("-windowed".to_string());
        }
    }
    if options.d3d11 {
        args.push("-force-d3d11".to_string());
    }
    if options.popupwindow {
        args.push("-popupwindow".to_string());
    }
    if !options.custom.is_empty() {
        for part in options.custom.split_whitespace() {
            args.push(part.to_string());
        }
    }

    args
}

pub fn launch_vrchat(vrchat_path: &str, profile: &Profile) -> Result<(), String> {
    let args = build_args(&profile.options);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

        // プロセス優先度クラスの定数
        const IDLE_PRIORITY_CLASS: u32 = 0x00000040;
        const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x00004000;
        const NORMAL_PRIORITY_CLASS: u32 = 0x00000020;
        const ABOVE_NORMAL_PRIORITY_CLASS: u32 = 0x00008000;
        const HIGH_PRIORITY_CLASS: u32 = 0x00000080;

        let priority_class = match profile.options.process_priority {
            Some(-2) => IDLE_PRIORITY_CLASS,
            Some(-1) => BELOW_NORMAL_PRIORITY_CLASS,
            Some(1) => ABOVE_NORMAL_PRIORITY_CLASS,
            Some(2) => HIGH_PRIORITY_CLASS,
            _ => NORMAL_PRIORITY_CLASS,
        };

        Command::new(vrchat_path)
            .args(&args)
            .creation_flags(priority_class)
            .spawn()
            .map_err(|e| format!("VRChat の起動に失敗しました: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(vrchat_path)
            .args(&args)
            .spawn()
            .map_err(|e| format!("VRChat の起動に失敗しました: {}", e))?;
    }

    Ok(())
}
