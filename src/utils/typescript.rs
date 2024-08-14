use crate::utils::helper::register_console;
use boa_engine::{Context, JsResult, Source};
use std::env;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

fn path(ts_file_path: &str) -> PathBuf {
  let current_dir: PathBuf = env::current_dir().expect("[V12]: Unable to get current directory");
  current_dir.join(ts_file_path)
}

pub fn strip_types(ts_file_path: &str) -> String {
  let full_path: PathBuf = path(ts_file_path);
  let ts_content: String = match fs::read_to_string(&full_path) {
    Ok(content) => content,
    Err(_) => {
      eprintln!(
        "[V12]: Unable to read TypeScript file: {:?}",
        full_path.display()
      );
      std::process::exit(1);
    }
  };
  let re: Regex = regex::Regex::new(r":\s*\w+(\[])?|<\w+>").unwrap();
  let js_content: String = re.replace_all(&ts_content, "").to_string();
  let temp_file_path: String = format!("{}.js", ts_file_path);
  fs::write(&temp_file_path, js_content).expect(&format!(
    "[V12]: Unable to write JavaScript file: {:?}",
    temp_file_path
  ));
  temp_file_path
}

pub fn exec_js(temp_file_path: &str) -> JsResult<()> {
  let full_path: PathBuf = path(temp_file_path);
  let script: String = fs::read_to_string(&full_path).expect(&format!(
    "[V12]: Unable to read temporary JavaScript file: {:?}",
    full_path.display()
  ));
  let mut context: Context = Context::default();

  register_console(&mut context);
  context.eval(Source::from_bytes(&script))?;

  Ok(())
}

pub fn process_typescript_file(ts_file_path: &str) {
  let temp_file_path: String = strip_types(ts_file_path);
  exec_js(&temp_file_path).expect(&format!(
    "[V12]: Failed to execute temporary JavaScript file: {:?}",
    temp_file_path
  ));
  fs::remove_file(temp_file_path.clone()).expect(&format!(
    "[V12]: Unable to delete temporary JavaScript file: {:?}",
    temp_file_path
  ));
}
