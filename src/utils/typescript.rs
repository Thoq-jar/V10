use std::fs;
use std::process::Command;

pub fn strip_types(ts_file_path: &str) -> String {
    let ts_content = fs::read_to_string(ts_file_path).expect("[V12]: Unable to read TypeScript file");
    let re = regex::Regex::new(r":\s*\w+(\[])?|<\w+>").unwrap();
    let js_content = re.replace_all(&ts_content, "").to_string();
    let temp_file_path = format!("{}.js", ts_file_path);
    fs::write(&temp_file_path, js_content).expect("[V12]: Unable to write temporary JavaScript file");
    temp_file_path
}

pub fn run_temp_file(temp_file_path: &str) {
    let output = Command::new("node")
        .arg(temp_file_path)
        .output()
        .expect("[V12]: Failed to execute temporary JavaScript file");

    if !output.stderr.is_empty() {
        eprintln!("[V12]: stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

pub fn process_typescript_file(ts_file_path: &str) {
    let temp_file_path = strip_types(ts_file_path);
    run_temp_file(&temp_file_path);
    fs::remove_file(temp_file_path).expect("[V12]: Unable to delete temporary JavaScript file");
}