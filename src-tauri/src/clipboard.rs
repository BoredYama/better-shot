//! Clipboard operations module

use crate::utils::AppResult;
use std::process::Command;

/// Copy an image file to the system clipboard
pub fn copy_image_to_clipboard(image_path: &str) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"set the clipboard to (read (POSIX file "{}") as «class PNGf»)"#,
            image_path
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to copy image to clipboard: {}", stderr));
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        use std::process::Stdio;

        // Read the image file ourselves, then pipe to xclip via stdin.
        // Using xclip -i <file> with .output() hangs because xclip forks
        // to serve clipboard requests and the parent waits forever.
        let image_bytes = std::fs::read(image_path)
            .map_err(|e| format!("Failed to read image file '{}': {}", image_path, e))?;

        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-t")
            .arg("image/png")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| {
                "Failed to copy image to clipboard. Please install 'xclip': sudo apt install xclip"
                    .to_string()
            })?;

        if let Some(ref mut stdin) = child.stdin {
            stdin
                .write_all(&image_bytes)
                .map_err(|e| format!("Failed to write to xclip stdin: {}", e))?;
        }
        // Drop stdin to signal EOF so xclip can proceed
        drop(child.stdin.take());

        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for xclip: {}", e))?;

        if !status.success() {
            return Err("xclip failed to copy image to clipboard".to_string());
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        return Err("Clipboard operations are not supported on this platform".to_string());
    }

    Ok(())
}

/// Copy text to the system clipboard
pub fn copy_text_to_clipboard(text: &str) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        let escaped_text = text.replace('"', "\\\"");
        let script = format!(r#"set the clipboard to "{}""#, escaped_text);

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to copy text to clipboard: {}", stderr));
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        use std::process::Stdio;

        // Try xclip first
        let xclip_result = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .spawn();

        match xclip_result {
            Ok(mut child) => {
                if let Some(ref mut stdin) = child.stdin {
                    stdin
                        .write_all(text.as_bytes())
                        .map_err(|e| format!("Failed to write to xclip stdin: {}", e))?;
                }
                let status = child
                    .wait()
                    .map_err(|e| format!("Failed to wait for xclip: {}", e))?;
                if !status.success() {
                    return Err("xclip failed to copy text".to_string());
                }
            }
            Err(_) => {
                // Fallback to xsel
                let xsel_result = Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--input")
                    .stdin(Stdio::piped())
                    .spawn();

                match xsel_result {
                    Ok(mut child) => {
                        if let Some(ref mut stdin) = child.stdin {
                            stdin
                                .write_all(text.as_bytes())
                                .map_err(|e| format!("Failed to write to xsel stdin: {}", e))?;
                        }
                        let status = child
                            .wait()
                            .map_err(|e| format!("Failed to wait for xsel: {}", e))?;
                        if !status.success() {
                            return Err("xsel failed to copy text".to_string());
                        }
                    }
                    Err(_) => {
                        return Err("Failed to copy text to clipboard. Please install 'xclip' or 'xsel': sudo apt install xclip".to_string());
                    }
                }
            }
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        return Err("Clipboard operations are not supported on this platform".to_string());
    }

    Ok(())
}
