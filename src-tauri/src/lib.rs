use std::io::Write;
use std::process::{Command, Stdio};

#[tauri::command]
fn run_code(code: String, input: String, language: String) -> Result<String, String> {
    // Write code to a temp file
    let ext = match language.as_str() {
        "python" => "py",
        _ => "cpp",
    };

    let tmp_dir = std::env::temp_dir();
    let src_path = tmp_dir.join(format!("tbcpc_run.{}", ext));
    let bin_path = tmp_dir.join("tbcpc_run");

    std::fs::write(&src_path, &code).map_err(|e| e.to_string())?;

    // Compile if C++
    if ext == "cpp" {
        let compile = Command::new("g++")
            .args([src_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap()])
            .output()
            .map_err(|e| e.to_string())?;

        if !compile.status.success() {
            return Ok(String::from_utf8_lossy(&compile.stderr).to_string());
        }
    }

    // Run
    let mut child = if ext == "cpp" {
        Command::new(&bin_path)
    } else {
        let mut cmd = Command::new("python3");
        cmd.arg(src_path.to_str().unwrap());
        cmd
    };

    let mut child = child
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    // Pipe input
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).ok();
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if stderr.is_empty() {
        Ok(stdout)
    } else {
        Ok(format!("{}{}", stdout, stderr))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
