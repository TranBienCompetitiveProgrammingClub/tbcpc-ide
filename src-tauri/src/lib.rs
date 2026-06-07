use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[tauri::command]
fn run_code(
    code: String,
    input: String,
    language: String,
    io_mode: String,
    file_path: String,
) -> Result<String, String> {
    let ext = match language.as_str() {
        "python" => "py",
        _ => "cpp",
    };

    let tmp_dir = std::env::temp_dir();
    let src_path = tmp_dir.join(format!("tbcpc_run.{}", ext));
    let bin_path = tmp_dir.join("tbcpc_run");

    std::fs::write(&src_path, &code).map_err(|e| e.to_string())?;

    if ext == "cpp" {
        let compile = Command::new("g++")
            .args([
                src_path.to_str().unwrap(),
                "-o",
                bin_path.to_str().unwrap(),
                "-std=c++17",
            ])
            .output()
            .map_err(|e| e.to_string())?;

        if !compile.status.success() {
            return Ok(String::from_utf8_lossy(&compile.stderr).to_string());
        }
    }

    let base = PathBuf::from(&file_path);
    let stem = base
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("untitled");

    let inp_path = tmp_dir.join(format!("{}.inp", stem));
    let out_path = tmp_dir.join(format!("{}.out", stem));

    if io_mode == "file" {
        eprintln!("Writing inp file...");
        match std::fs::write(&inp_path, &input) {
            Ok(_) => eprintln!("Written successfully to {:?}", inp_path),
            Err(e) => eprintln!("Write failed: {}", e),
        }
        eprintln!("File exists after write: {}", inp_path.exists());
    }

    let mut cmd = if ext == "cpp" {
        Command::new(&bin_path)
    } else {
        let mut c = Command::new("python3");
        c.arg(src_path.to_str().unwrap());
        c
    };

    let stdin_cfg = if io_mode == "file" {
        let f = std::fs::File::open(&inp_path)
            .map_err(|e| format!("Cannot open {}: {}", inp_path.display(), e))?;
        Stdio::from(f)
    } else {
        Stdio::piped()
    };

    let stdout_cfg = if io_mode == "file" {
        let f = std::fs::File::create(&out_path)
            .map_err(|e| format!("Cannot create {}: {}", out_path.display(), e))?;
        Stdio::from(f)
    } else {
        Stdio::piped()
    };

    let mut child = cmd
        .stdin(stdin_cfg)
        .stdout(stdout_cfg)
        .stderr(Stdio::piped())
        .current_dir(&tmp_dir)
        .spawn()
        .map_err(|e| e.to_string())?;

    if io_mode != "file" {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes()).ok();
        }
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if io_mode == "file" {
        let out_content = std::fs::read_to_string(&out_path).unwrap_or_default();
        eprintln!("out_content: {:?}", out_content);
        if stderr.is_empty() {
            Ok(out_content)
        } else {
            Ok(format!("{}{}", out_content, stderr))
        }
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if stderr.is_empty() {
            Ok(stdout)
        } else {
            Ok(format!("{}{}", stdout, stderr))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![run_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
